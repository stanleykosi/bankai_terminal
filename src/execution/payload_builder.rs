/**
 * @description
 * Polymarket order payload builder with pricing, slippage, and sizing guards.
 *
 * @dependencies
 * - arc-swap: live config access
 * - ethers-core: address and U256 helpers
 * - serde_json: relayer payload construction
 *
 * @notes
 * - Builds signed EIP-712 orders and relayer HMAC auth headers.
 * - Applies max slippage/impact constraints for taker (Snipe) orders.
 */
use arc_swap::ArcSwap;
use ethers_core::types::{Address, U256};
use secrecy::ExposeSecret;
use serde_json::json;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::Config;
use crate::engine::types::{TradeIntent, TradeMode, TradeSide};
use crate::error::{BankaiError, Result};
use crate::execution::orchestrator::{ExecutionPayloadBuilder, ExecutionPayloads};
use crate::execution::relayer::RelayerAuth;
use crate::execution::signer::{Eip712Signer, OrderSignaturePayload};
use crate::security::Secrets;
use crate::storage::orderbook::{BookSide, OrderBookStore};
use crate::storage::redis::RedisManager;

const USDC_DECIMALS: u32 = 6;
const LOT_SIZE_DECIMALS: u32 = 2;
const ORDER_PATH: &str = "/order";
const VWAP_LEVEL_LIMIT: usize = 50;
const SNIPE_FEE_SIZE_SCALE_DENOM_BPS: f64 = 2000.0;
const SNIPE_FEE_SIZE_MIN_SCALE: f64 = 0.25;
const SNIPE_FEE_CURVE_ALPHA: f64 = 0.5;
const SNIPE_SLIPPAGE_SIZE_SCALE_DENOM_BPS: f64 = 300.0;
const SNIPE_SLIPPAGE_SIZE_MIN_SCALE: f64 = 0.4;

#[derive(Debug, Clone)]
pub struct PolymarketPayloadBuilder {
    config: Arc<ArcSwap<Config>>,
    redis: RedisManager,
    orderbook: OrderBookStore,
    signer: Eip712Signer,
    exchange_address: Address,
    api_key: String,
    api_passphrase: String,
    api_secret: String,
    order_path: String,
}

impl PolymarketPayloadBuilder {
    pub fn new(
        config: Arc<ArcSwap<Config>>,
        redis: RedisManager,
        orderbook: OrderBookStore,
        secrets: &Secrets,
        exchange_address: Address,
        chain_id: u64,
    ) -> Result<Self> {
        let api_key = secrets
            .polymarket_api_key
            .as_ref()
            .map(|value| value.expose_secret().trim().to_string())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                BankaiError::InvalidArgument("polymarket api key missing".to_string())
            })?;
        let api_passphrase = secrets
            .polymarket_api_passphrase
            .as_ref()
            .map(|value| value.expose_secret().trim().to_string())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                BankaiError::InvalidArgument("polymarket api passphrase missing".to_string())
            })?;
        let api_secret = secrets
            .polymarket_api_secret
            .as_ref()
            .map(|value| value.expose_secret().trim().to_string())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                BankaiError::InvalidArgument("polymarket api secret missing".to_string())
            })?;
        let signer = Eip712Signer::from_secrets(secrets, chain_id)?;

        Ok(Self {
            config,
            redis,
            orderbook,
            signer,
            exchange_address,
            api_key,
            api_passphrase,
            api_secret,
            order_path: ORDER_PATH.to_string(),
        })
    }

    async fn build_order_payload(&self, intent: &TradeIntent) -> Result<OrderBuildResult> {
        let config = self.config.load_full();
        let execution = &config.execution;
        if !execution.enable_trading {
            return Err(BankaiError::InvalidArgument(
                "execution disabled in config".to_string(),
            ));
        }

        let token_id = parse_token_id(&intent.asset_id)?;
        let book = fetch_book_quotes(&self.orderbook, &intent.asset_id).await?;
        let metadata = self.redis.get_market_metadata(&intent.market_id).await?;
        let fee_rate_bps = self
            .redis
            .get_fee_rate_bps(&intent.asset_id)
            .await?
            .or(metadata.fee_rate_bps)
            .unwrap_or(config.fees.taker_fee_bps);
        let min_tick_size = metadata.min_tick_size.unwrap_or(0.001);

        let price = match (intent.side, intent.mode) {
            (TradeSide::Buy, TradeMode::Ladder) => {
                let offset = intent.spread_offset_bps / 10_000.0;
                let base = book.mid * (1.0 - offset);
                let max_price = (book.best_ask - min_tick_size).max(0.0);
                let price = base.min(max_price).max(min_tick_size);
                round_down_to_tick(price, min_tick_size)
            }
            (TradeSide::Buy, TradeMode::Snipe) => round_down_to_tick(book.best_ask, min_tick_size),
            (TradeSide::Sell, TradeMode::Ladder) => {
                let offset = intent.spread_offset_bps / 10_000.0;
                let base = book.mid * (1.0 + offset);
                let min_price = (book.best_bid + min_tick_size).min(1.0);
                let price = base.max(min_price).min(1.0);
                round_down_to_tick(price, min_tick_size)
            }
            (TradeSide::Sell, TradeMode::Snipe) => round_down_to_tick(book.best_bid, min_tick_size),
        };

        if price <= 0.0 || price >= 1.0 {
            return Err(BankaiError::InvalidArgument(
                "computed order price invalid".to_string(),
            ));
        }

        let mut size = if let Some(requested) = intent.requested_size {
            requested
        } else if intent.side == TradeSide::Sell {
            return Err(BankaiError::InvalidArgument(
                "sell intent missing requested size".to_string(),
            ));
        } else {
            compute_order_size(
                &self.redis,
                intent.true_prob,
                price,
                execution.default_order_usdc,
                execution.min_order_usdc,
                execution.max_order_usdc,
                config.strategy.kelly_fraction,
            )
            .await?
        };

        if intent.mode == TradeMode::Snipe && fee_rate_bps > 0.0 {
            let price_curve = (4.0 * price * (1.0 - price)).clamp(0.0, 1.0);
            let effective_fee_bps = fee_rate_bps * (1.0 + (SNIPE_FEE_CURVE_ALPHA * price_curve));
            let scale = (1.0 - (effective_fee_bps / SNIPE_FEE_SIZE_SCALE_DENOM_BPS))
                .clamp(SNIPE_FEE_SIZE_MIN_SCALE, 1.0);
            size *= scale;
        }

        if let Some(min_size) = metadata.min_order_size {
            if size < min_size {
                let notional = min_size * price;
                if notional > execution.max_order_usdc {
                    return Err(BankaiError::InvalidArgument(
                        "min order size exceeds max order notional".to_string(),
                    ));
                }
                size = min_size;
            }
        }

        size = round_down(size, LOT_SIZE_DECIMALS);
        if size <= 0.0 {
            return Err(BankaiError::InvalidArgument(
                "order size too small after rounding".to_string(),
            ));
        }

        if intent.mode == TradeMode::Snipe {
            let min_size_guard = metadata.min_order_size.unwrap_or(0.0);
            let mut attempts = 0u8;
            let (side, best_price, mid_price) = match intent.side {
                TradeSide::Buy => (BookSide::Ask, book.best_ask, book.mid),
                TradeSide::Sell => (BookSide::Bid, book.best_bid, book.mid),
            };
            loop {
                let vwap = self
                    .orderbook
                    .vwap_for_size(&intent.asset_id, side, size, VWAP_LEVEL_LIMIT)
                    .await?
                    .ok_or_else(|| {
                        BankaiError::InvalidArgument("insufficient order book depth".to_string())
                    })?;
                let slippage_bps = if intent.side == TradeSide::Buy {
                    ((vwap.avg_price - best_price) / best_price) * 10_000.0
                } else {
                    ((best_price - vwap.avg_price) / best_price) * 10_000.0
                };
                let impact_bps = if intent.side == TradeSide::Buy {
                    ((vwap.avg_price - mid_price) / mid_price) * 10_000.0
                } else {
                    ((mid_price - vwap.avg_price) / mid_price) * 10_000.0
                };
                let mut scale: f64 = 1.0;
                if slippage_bps > execution.max_slippage_bps {
                    scale = scale.min(execution.max_slippage_bps / slippage_bps);
                }
                if impact_bps > execution.max_impact_bps {
                    scale = scale.min(execution.max_impact_bps / impact_bps);
                }
                if slippage_bps > 0.0 {
                    let slip_scale = (1.0 - (slippage_bps / SNIPE_SLIPPAGE_SIZE_SCALE_DENOM_BPS))
                        .clamp(SNIPE_SLIPPAGE_SIZE_MIN_SCALE, 1.0);
                    scale = scale.min(slip_scale);
                }
                if scale >= 0.999 || attempts >= 3 {
                    if slippage_bps > execution.max_slippage_bps
                        || impact_bps > execution.max_impact_bps
                    {
                        tracing::warn!(
                            slippage_bps = slippage_bps,
                            impact_bps = impact_bps,
                            size = size,
                            "snipe slippage/impact above guard; proceeding with reduced size"
                        );
                    }
                    break;
                }
                let new_size = (size * scale).max(min_size_guard);
                if (new_size - size).abs() < 1e-6 {
                    break;
                }
                size = new_size;
                attempts += 1;
            }
        }

        if let Some(min_size) = metadata.min_order_size {
            if size < min_size {
                let notional = min_size * price;
                if notional > execution.max_order_usdc {
                    return Err(BankaiError::InvalidArgument(
                        "min order size exceeds max order notional".to_string(),
                    ));
                }
                size = min_size;
            }
        }

        size = round_down(size, LOT_SIZE_DECIMALS);
        if size <= 0.0 {
            return Err(BankaiError::InvalidArgument(
                "order size too small after rounding".to_string(),
            ));
        }

        let (maker_amount, taker_amount, side) = match intent.side {
            TradeSide::Buy => (price * size, size, 0u8),
            TradeSide::Sell => (size, price * size, 1u8),
        };

        let maker_amount = to_fixed_u256(maker_amount, USDC_DECIMALS)?;
        let taker_amount = to_fixed_u256(taker_amount, USDC_DECIMALS)?;
        let order_type = resolve_order_type(intent.mode, execution);
        let now = current_unix_timestamp();
        let expiration = match order_type.as_str() {
            "GTD" => gtd_expiration(
                now,
                intent.market_window.as_ref().map(|w| w.end_time_ms),
                execution.gtd_min_expiry_secs,
                execution.order_expiry_secs,
            ),
            "GTC" => 0u64,
            _ => now + execution.order_expiry_secs,
        };
        let order = OrderSignaturePayload {
            salt: U256::from(now_ms()?),
            maker: self.signer.address(),
            signer: self.signer.address(),
            taker: Address::zero(),
            token_id,
            maker_amount,
            taker_amount,
            expiration: U256::from(expiration),
            nonce: U256::from(0u64),
            fee_rate_bps: fee_rate_bps.round().max(0.0) as u64,
            side,
            signature_type: 0,
        };

        let typed_data = self
            .signer
            .order_typed_data(&order, self.exchange_address)?;
        let signature = self.signer.sign_typed_data(&typed_data).await?;
        let signature_hex = signature.to_string();

        Ok(OrderBuildResult {
            token_id,
            price,
            size,
            maker_amount,
            taker_amount,
            order,
            signature: signature_hex,
            order_type,
            fee_rate_bps,
            best_bid: book.best_bid,
            best_ask: book.best_ask,
            mid: book.mid,
        })
    }
}

#[async_trait::async_trait]
impl ExecutionPayloadBuilder for PolymarketPayloadBuilder {
    async fn build_payloads(&self, intent: &TradeIntent) -> Result<ExecutionPayloads> {
        let order = self.build_order_payload(intent).await?;
        let order_json = json!({
            "salt": order.order.salt.to_string(),
            "maker": format!("{}", order.order.maker),
            "signer": format!("{}", order.order.signer),
            "taker": format!("{}", order.order.taker),
            "tokenId": order.order.token_id.to_string(),
            "makerAmount": order.order.maker_amount.to_string(),
            "takerAmount": order.order.taker_amount.to_string(),
            "expiration": order.order.expiration.to_string(),
            "nonce": order.order.nonce.to_string(),
            "feeRateBps": order.order.fee_rate_bps.to_string(),
            "side": order.order.side.to_string(),
            "signatureType": order.order.signature_type.to_string(),
            "signature": order.signature,
        });
        let relayer_payload = json!({
            "order": order_json,
            "owner": self.api_key.clone(),
            "orderType": order.order_type,
        });
        let relayer_payload = if intent.mode == TradeMode::Ladder
            && self.config.load_full().execution.post_only_ladder
        {
            let mut payload = relayer_payload;
            if let Some(map) = payload.as_object_mut() {
                map.insert("postOnly".to_string(), json!(true));
            }
            payload
        } else {
            relayer_payload
        };

        let body = serde_json::to_string(&relayer_payload)?;
        let timestamp = current_unix_timestamp();
        let signature =
            build_hmac_signature(&self.api_secret, timestamp, "POST", &self.order_path, &body)?;
        let auth = RelayerAuth {
            address: format!("{}", self.signer.address()),
            api_key: self.api_key.clone(),
            passphrase: self.api_passphrase.clone(),
            signature,
            timestamp: timestamp.to_string(),
            builder: None,
        };

        let metadata = json!({
            "asset_id": intent.asset_id.as_str(),
            "token_id": order.token_id.to_string(),
            "side": match intent.side {
                TradeSide::Buy => "BUY",
                TradeSide::Sell => "SELL",
            },
            "price": order.price,
            "size": order.size,
            "maker_amount": format!("{}", order.maker_amount),
            "taker_amount": format!("{}", order.taker_amount),
            "best_bid": order.best_bid,
            "best_ask": order.best_ask,
            "mid": order.mid,
        });

        Ok(ExecutionPayloads {
            relayer_payload,
            relayer_auth: Some(auth),
            direct_request: None,
            fees_paid: estimate_fee_paid(order.fee_rate_bps, order.price, order.size),
            metadata: Some(metadata),
        })
    }
}

struct OrderBuildResult {
    token_id: U256,
    price: f64,
    size: f64,
    maker_amount: U256,
    taker_amount: U256,
    order: OrderSignaturePayload,
    signature: String,
    order_type: String,
    fee_rate_bps: f64,
    best_bid: f64,
    best_ask: f64,
    mid: f64,
}

struct BookQuotes {
    best_bid: f64,
    best_ask: f64,
    mid: f64,
}

async fn fetch_book_quotes(orderbook: &OrderBookStore, token_id: &str) -> Result<BookQuotes> {
    let bid = orderbook
        .best_level(token_id, BookSide::Bid)
        .await?
        .ok_or_else(|| BankaiError::InvalidArgument("best bid missing".to_string()))?;
    let ask = orderbook
        .best_level(token_id, BookSide::Ask)
        .await?
        .ok_or_else(|| BankaiError::InvalidArgument("best ask missing".to_string()))?;
    let best_bid = bid
        .price
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("best bid not numeric".to_string()))?;
    let best_ask = ask
        .price
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("best ask not numeric".to_string()))?;
    if best_bid <= 0.0 || best_ask <= 0.0 {
        return Err(BankaiError::InvalidArgument(
            "order book prices invalid".to_string(),
        ));
    }
    let mid = (best_bid + best_ask) / 2.0;
    Ok(BookQuotes {
        best_bid,
        best_ask,
        mid,
    })
}

async fn compute_order_size(
    redis: &RedisManager,
    true_prob: f64,
    price: f64,
    default_order_usdc: f64,
    min_order_usdc: f64,
    max_order_usdc: f64,
    kelly_fraction: f64,
) -> Result<f64> {
    let bankroll = redis.get_float("sys:bankroll:usdc").await?;
    let odds = if price > 0.0 { 1.0 / price } else { 0.0 };
    let kelly = calculate_kelly(true_prob, odds);
    let target = if let Some(bankroll) = bankroll {
        bankroll * kelly_fraction * kelly
    } else {
        default_order_usdc
    };
    let mut notional = target.clamp(min_order_usdc, max_order_usdc);
    if notional < min_order_usdc {
        return Err(BankaiError::InvalidArgument(
            "order notional below min".to_string(),
        ));
    }
    if notional > max_order_usdc {
        notional = max_order_usdc;
    }
    Ok(notional / price)
}

fn calculate_kelly(win_prob: f64, odds: f64) -> f64 {
    if win_prob <= 0.0 || win_prob >= 1.0 {
        return 0.0;
    }
    if odds <= 1.0 {
        return 0.0;
    }
    let payout = odds - 1.0;
    let loss_prob = 1.0 - win_prob;
    let kelly = (win_prob * payout - loss_prob) / payout;
    kelly.clamp(0.0, 1.0)
}

fn round_down(value: f64, decimals: u32) -> f64 {
    if decimals == 0 {
        return value.floor();
    }
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).floor() / factor
}

fn round_down_to_tick(price: f64, tick: f64) -> f64 {
    if tick <= 0.0 {
        return price;
    }
    (price / tick).floor() * tick
}

fn to_fixed_u256(value: f64, decimals: u32) -> Result<U256> {
    if value < 0.0 {
        return Err(BankaiError::InvalidArgument(
            "negative amount not allowed".to_string(),
        ));
    }
    let factor = 10_f64.powi(decimals as i32);
    let scaled = (value * factor).floor();
    let scaled = if scaled < 0.0 { 0.0 } else { scaled };
    Ok(U256::from(scaled as u128))
}

fn parse_token_id(token_id: &str) -> Result<U256> {
    let trimmed = token_id.trim();
    if trimmed.is_empty() {
        return Err(BankaiError::InvalidArgument("token id missing".to_string()));
    }
    U256::from_dec_str(trimmed)
        .map_err(|_| BankaiError::InvalidArgument("token id not numeric".to_string()))
}

fn estimate_fee_paid(fee_rate_bps: f64, price: f64, size: f64) -> f64 {
    if fee_rate_bps <= 0.0 {
        return 0.0;
    }
    let notional = price * size;
    notional * (fee_rate_bps / 10_000.0)
}

fn resolve_order_type(mode: TradeMode, execution: &crate::config::ExecutionConfig) -> String {
    let ladder = execution.ladder_order_type.trim().to_ascii_uppercase();
    let snipe = execution.snipe_order_type.trim().to_ascii_uppercase();
    match mode {
        TradeMode::Ladder => match ladder.as_str() {
            "GTD" | "GTC" => ladder,
            _ => "GTD".to_string(),
        },
        TradeMode::Snipe => match snipe.as_str() {
            "FOK" | "FAK" => snipe,
            _ => "FOK".to_string(),
        },
    }
}

fn gtd_expiration(
    now_secs: u64,
    window_end_ms: Option<u64>,
    min_expiry_secs: u64,
    fallback_secs: u64,
) -> u64 {
    let target_secs = window_end_ms
        .map(|value| value / 1000)
        .unwrap_or_else(|| now_secs + fallback_secs);
    let min_secs = now_secs + min_expiry_secs;
    target_secs.max(min_secs)
}

fn now_ms() -> Result<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now.as_millis() as u64)
}

fn current_unix_timestamp() -> u64 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    duration.as_secs()
}

fn build_hmac_signature(
    secret: &str,
    timestamp: u64,
    method: &str,
    path: &str,
    body: &str,
) -> Result<String> {
    use base64::engine::general_purpose;
    use base64::Engine as _;
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<sha2::Sha256>;

    let key = general_purpose::STANDARD
        .decode(secret)
        .map_err(|err| BankaiError::InvalidArgument(format!("api secret decode error: {err}")))?;
    let message = format!("{timestamp}{method}{path}{body}");
    let mut mac = HmacSha256::new_from_slice(&key)
        .map_err(|_| BankaiError::InvalidArgument("api secret is invalid for hmac".to_string()))?;
    mac.update(message.as_bytes());
    let result = mac.finalize().into_bytes();
    let signature = general_purpose::STANDARD.encode(result);
    Ok(signature.replace('+', "-").replace('/', "_"))
}

#[cfg(test)]
mod tests_end {
    use super::*;
    use crate::config::ExecutionConfig;

    #[test]
    fn test_gtd_expiration_respects_minimum() {
        let now = 1_000u64;
        let window_end_ms = Some((now + 10) * 1000);
        let expiration = gtd_expiration(now, window_end_ms, 60, 30);
        assert_eq!(expiration, now + 60);
    }

    #[test]
    fn test_resolve_order_type_defaults() {
        let execution = ExecutionConfig::default();
        assert_eq!(resolve_order_type(TradeMode::Ladder, &execution), "GTD");
        assert_eq!(resolve_order_type(TradeMode::Snipe, &execution), "FOK");
    }
}
