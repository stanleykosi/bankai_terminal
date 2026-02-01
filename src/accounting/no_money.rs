/**
 * @purpose
 * Paper-trade evaluator for no-money mode.
 *
 * @notes
 * - Records intents without placing orders.
 * - Scores them using start/end prices from Chainlink RTDS.
 */
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::task::JoinHandle;

use crate::config::Config;
use crate::engine::types::{TradeIntent, TradeMode, TradeSide};
use crate::error::{BankaiError, Result};
use crate::storage::orderbook::{BookSide, OrderBookStore};
use crate::storage::redis::{MarketMetadata, RedisManager};
use arc_swap::ArcSwap;

const PAPER_ZSET_KEY: &str = "paper:intents";
const PAPER_STATS_WINS: &str = "paper:stats:wins";
const PAPER_STATS_LOSSES: &str = "paper:stats:losses";
const PAPER_STATS_TOTAL: &str = "paper:stats:total";
const PAPER_STATS_ACCURACY: &str = "paper:stats:accuracy_pct";
const PAPER_STATS_MISSED: &str = "paper:stats:missed";
const PAPER_BANKROLL_KEY: &str = "paper:bankroll:usdc";
const PAPER_BANKROLL_START_KEY: &str = "paper:bankroll:start_usdc";
const PAPER_LOG_LIMIT: usize = 200;
const PAPER_VWAP_LEVELS: usize = 50;

#[derive(Debug, Clone)]
pub struct PaperSimConfig {
    pub start_bankroll_usdc: f64,
    pub kelly_fraction: f64,
    pub min_order_usdc: f64,
    pub max_order_usdc: f64,
    pub default_order_usdc: f64,
    pub slippage_bps: f64,
    pub latency_ms: u64,
    pub taker_fee_bps: f64,
}

impl PaperSimConfig {
    pub fn from_config(config: &Config) -> Self {
        Self {
            start_bankroll_usdc: config.execution.paper_start_bankroll_usdc,
            kelly_fraction: config.strategy.kelly_fraction,
            min_order_usdc: config.execution.min_order_usdc,
            max_order_usdc: config.execution.max_order_usdc,
            default_order_usdc: config.execution.default_order_usdc,
            slippage_bps: config.execution.paper_slippage_bps,
            latency_ms: config.execution.paper_latency_ms,
            taker_fee_bps: config.fees.taker_fee_bps,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperIntent {
    pub id: String,
    pub asset: String,
    pub market_id: String,
    pub asset_id: String,
    pub predicted: String,
    pub mode: String,
    pub side: String,
    pub implied_prob: f64,
    pub true_prob: f64,
    pub edge_bps: f64,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
    pub emitted_at_ms: u64,
    #[serde(default)]
    pub entry_price: Option<f64>,
    #[serde(default)]
    pub size: Option<f64>,
    #[serde(default)]
    pub notional_usdc: Option<f64>,
    #[serde(default)]
    pub fee_bps: Option<f64>,
    #[serde(default)]
    pub filled: bool,
    #[serde(default)]
    pub fill_reason: Option<String>,
    #[serde(default)]
    pub orderbook_age_ms: Option<u64>,
}

pub fn spawn_no_money_tracker(config: Arc<ArcSwap<Config>>, redis: RedisManager) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(error) = run_tracker(config, redis).await {
            tracing::error!(?error, "no-money tracker stopped");
        }
    })
}

pub async fn record_no_money_intent(
    redis: &RedisManager,
    intent: &TradeIntent,
    sim: &PaperSimConfig,
) -> Result<()> {
    let window = intent.market_window.ok_or_else(|| {
        BankaiError::InvalidArgument("paper intent missing market window".to_string())
    })?;
    let asset = resolve_asset_for_market(redis, &intent.market_id).await?;
    let predicted = resolve_prediction(redis, &intent.market_id, &intent.asset_id).await?;
    if predicted == "UNKNOWN" {
        return Ok(());
    }
    let id = format!("paper:intent:{}:{}", asset, window.start_time_ms);
    if redis.get_string(&id).await?.is_some() {
        return Ok(());
    }
    let orderbook = OrderBookStore::new(redis.clone());
    let now = now_ms()?;
    let (entry_price, size, notional_usdc, fee_bps, filled, fill_reason, orderbook_age_ms) =
        simulate_entry(redis, &orderbook, intent, sim, now).await?;
    let record = PaperIntent {
        id: id.clone(),
        asset,
        market_id: intent.market_id.clone(),
        asset_id: intent.asset_id.clone(),
        predicted,
        mode: mode_label(intent.mode).to_string(),
        side: side_label(intent.side).to_string(),
        implied_prob: intent.implied_prob,
        true_prob: intent.true_prob,
        edge_bps: intent.edge_bps,
        start_time_ms: window.start_time_ms,
        end_time_ms: window.end_time_ms,
        emitted_at_ms: intent.timestamp_ms,
        entry_price,
        size,
        notional_usdc,
        fee_bps,
        filled,
        fill_reason,
        orderbook_age_ms,
    };
    let payload = serde_json::to_string(&record)?;
    redis.set_string(&id, &payload).await?;
    redis
        .zadd(PAPER_ZSET_KEY, record.end_time_ms as f64, &id)
        .await?;
    Ok(())
}

async fn run_tracker(config: Arc<ArcSwap<Config>>, redis: RedisManager) -> Result<()> {
    let mut tick = tokio::time::interval(Duration::from_secs(1));
    let mut reset_done = false;
    loop {
        tick.tick().await;
        let cfg = config.load_full();
        if !cfg.execution.no_money_mode {
            reset_done = false;
            continue;
        }
        if !reset_done {
            if !cfg.execution.paper_stats_persist {
                reset_paper_state(&redis).await?;
            }
            ensure_paper_bankroll(&redis, &cfg).await?;
            reset_done = true;
        }

        let now = now_ms()?;
        let pending = redis.zrange_with_scores(PAPER_ZSET_KEY, 0, -1).await?;
        for (key, score) in pending {
            if score as u64 > now {
                break;
            }
            let payload = match redis.get_string(&key).await? {
                Some(value) => value,
                None => {
                    let _ = redis.zrem(PAPER_ZSET_KEY, &key).await;
                    continue;
                }
            };
            let record: PaperIntent = match serde_json::from_str(&payload) {
                Ok(value) => value,
                Err(_) => {
                    let _ = redis.zrem(PAPER_ZSET_KEY, &key).await;
                    let _ = redis.del(&key).await;
                    continue;
                }
            };

            let Some((start_ms, start_price)) = redis.get_asset_start_price(&record.asset).await?
            else {
                continue;
            };
            if start_ms != record.start_time_ms {
                continue;
            }
            let Some((end_ms, end_price)) = redis.get_asset_end_price(&record.asset).await? else {
                continue;
            };
            if end_ms != record.end_time_ms {
                continue;
            }

            if !record.filled {
                let _ = redis.incr_float(PAPER_STATS_MISSED, 1.0).await;
                let reason = record
                    .fill_reason
                    .clone()
                    .unwrap_or_else(|| "unfilled".to_string());
                let message = format!(
                    "[PAPER] asset={} market={} skipped reason={} orderbook_age_ms={:?}",
                    record.asset, record.market_id, reason, record.orderbook_age_ms
                );
                let _ = redis.push_activity_log(&message, PAPER_LOG_LIMIT).await;
                let _ = redis.zrem(PAPER_ZSET_KEY, &key).await;
                let _ = redis.del(&key).await;
                continue;
            }

            let actual = if end_price >= start_price {
                "UP"
            } else {
                "DOWN"
            };
            let correct = record.predicted == actual;

            let entry_price = record.entry_price.unwrap_or(record.implied_prob);
            let size = record.size.unwrap_or(0.0);
            let notional = record.notional_usdc.unwrap_or(entry_price * size);
            let fee_bps = record.fee_bps.unwrap_or(0.0);
            let fees_paid = if fee_bps > 0.0 {
                notional * (fee_bps / 10_000.0)
            } else {
                0.0
            };
            let pnl = if correct {
                (size * (1.0 - entry_price)) - fees_paid
            } else {
                -notional - fees_paid
            };
            let bankroll = redis
                .get_float(PAPER_BANKROLL_KEY)
                .await?
                .unwrap_or(cfg.execution.paper_start_bankroll_usdc);
            let new_bankroll = (bankroll + pnl).max(0.0);
            let _ = redis.set_float(PAPER_BANKROLL_KEY, new_bankroll).await;

            if correct {
                let _ = redis.incr_float(PAPER_STATS_WINS, 1.0).await;
            } else {
                let _ = redis.incr_float(PAPER_STATS_LOSSES, 1.0).await;
            }
            let total = redis
                .incr_float(PAPER_STATS_TOTAL, 1.0)
                .await
                .unwrap_or(0.0);
            let wins = redis.get_float(PAPER_STATS_WINS).await?.unwrap_or(0.0);
            let accuracy = if total > 0.0 {
                (wins / total) * 100.0
            } else {
                0.0
            };
            let _ = redis.set_float(PAPER_STATS_ACCURACY, accuracy).await;

            let message = format!(
                "[PAPER] asset={} market={} predicted={} actual={} entry={:.4} size={:.2} pnl={:.4} bankroll={:.4} ok={} accuracy={:.2}%",
                record.asset,
                record.market_id,
                record.predicted,
                actual,
                entry_price,
                size,
                pnl,
                new_bankroll,
                correct,
                accuracy
            );
            let _ = redis.push_activity_log(&message, PAPER_LOG_LIMIT).await;

            let _ = redis.zrem(PAPER_ZSET_KEY, &key).await;
            let _ = redis.del(&key).await;
        }
    }
}

async fn reset_paper_state(redis: &RedisManager) -> Result<()> {
    let pending = redis.zrange_with_scores(PAPER_ZSET_KEY, 0, -1).await?;
    for (key, _) in pending {
        let _ = redis.del(&key).await;
    }
    let _ = redis.del(PAPER_ZSET_KEY).await;
    let _ = redis.del(PAPER_STATS_WINS).await;
    let _ = redis.del(PAPER_STATS_LOSSES).await;
    let _ = redis.del(PAPER_STATS_TOTAL).await;
    let _ = redis.del(PAPER_STATS_ACCURACY).await;
    let _ = redis.del(PAPER_STATS_MISSED).await;
    let _ = redis.del(PAPER_BANKROLL_KEY).await;
    let _ = redis.del(PAPER_BANKROLL_START_KEY).await;
    Ok(())
}

async fn ensure_paper_bankroll(redis: &RedisManager, cfg: &Config) -> Result<()> {
    let start = cfg.execution.paper_start_bankroll_usdc.max(0.0);
    let current = redis.get_float(PAPER_BANKROLL_KEY).await?;
    if current.is_none() {
        let _ = redis.set_float(PAPER_BANKROLL_KEY, start).await;
    }
    let stored_start = redis.get_float(PAPER_BANKROLL_START_KEY).await?;
    if stored_start.is_none() {
        let _ = redis.set_float(PAPER_BANKROLL_START_KEY, start).await;
    }
    Ok(())
}

async fn simulate_entry(
    redis: &RedisManager,
    orderbook: &OrderBookStore,
    intent: &TradeIntent,
    sim: &PaperSimConfig,
    now_ms: u64,
) -> Result<(
    Option<f64>,
    Option<f64>,
    Option<f64>,
    Option<f64>,
    bool,
    Option<String>,
    Option<u64>,
)> {
    let price = intent.implied_prob;
    if !(0.0..=1.0).contains(&price) || price == 0.0 {
        return Ok((
            None,
            None,
            None,
            None,
            false,
            Some("invalid_price".to_string()),
            None,
        ));
    }
    let bankroll = redis
        .get_float(PAPER_BANKROLL_KEY)
        .await?
        .unwrap_or(sim.start_bankroll_usdc);
    let odds = 1.0 / price;
    let kelly = calculate_kelly(intent.true_prob, odds);
    let target = if kelly > 0.0 {
        bankroll * sim.kelly_fraction * kelly
    } else {
        sim.default_order_usdc
    };
    let notional = target.clamp(sim.min_order_usdc, sim.max_order_usdc);
    if notional < sim.min_order_usdc {
        return Ok((
            None,
            None,
            None,
            None,
            false,
            Some("min_order".to_string()),
            None,
        ));
    }
    let size = notional / price;

    let last_update = orderbook.last_update_ms(&intent.asset_id).await?;
    let orderbook_age = last_update.map(|ts| now_ms.saturating_sub(ts));
    if sim.latency_ms > 0 {
        if last_update.is_none() || orderbook_age.unwrap_or(u64::MAX) > sim.latency_ms {
            return Ok((
                None,
                Some(size),
                Some(notional),
                None,
                false,
                Some("latency".to_string()),
                orderbook_age,
            ));
        }
    }

    let side = match intent.side {
        TradeSide::Buy => BookSide::Ask,
        TradeSide::Sell => BookSide::Bid,
    };
    let vwap = orderbook
        .vwap_for_size(&intent.asset_id, side, size, PAPER_VWAP_LEVELS)
        .await?;
    let Some(vwap) = vwap else {
        return Ok((
            None,
            Some(size),
            Some(notional),
            None,
            false,
            Some("depth".to_string()),
            orderbook_age,
        ));
    };
    let mut entry_price = vwap.avg_price;
    if sim.slippage_bps > 0.0 {
        let slip = sim.slippage_bps / 10_000.0;
        entry_price = match intent.side {
            TradeSide::Buy => entry_price * (1.0 + slip),
            TradeSide::Sell => entry_price * (1.0 - slip),
        };
    }
    let notional_adj = size * entry_price;
    let fee_bps = if intent.mode == TradeMode::Snipe {
        redis
            .get_fee_rate_bps(&intent.asset_id)
            .await?
            .unwrap_or(sim.taker_fee_bps)
    } else {
        0.0
    };

    Ok((
        Some(entry_price),
        Some(size),
        Some(notional_adj),
        Some(fee_bps),
        true,
        None,
        orderbook_age,
    ))
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

async fn resolve_asset_for_market(redis: &RedisManager, market_id: &str) -> Result<String> {
    for asset in ["BTC", "ETH", "SOL"] {
        if let Some(window) = redis.get_asset_window(&asset).await? {
            if window.market_id == market_id {
                return Ok(asset.to_string());
            }
        }
    }
    Err(BankaiError::InvalidArgument(
        "unable to resolve asset for market".to_string(),
    ))
}

async fn resolve_prediction(
    redis: &RedisManager,
    market_id: &str,
    asset_id: &str,
) -> Result<String> {
    let metadata: MarketMetadata = redis.get_market_metadata(market_id).await?;
    if metadata
        .outcome_up_token_id
        .as_ref()
        .map(|value| value == asset_id)
        .unwrap_or(false)
    {
        return Ok("UP".to_string());
    }
    if metadata
        .outcome_down_token_id
        .as_ref()
        .map(|value| value == asset_id)
        .unwrap_or(false)
    {
        return Ok("DOWN".to_string());
    }
    Ok("UNKNOWN".to_string())
}

fn mode_label(mode: TradeMode) -> &'static str {
    match mode {
        TradeMode::Ladder => "LADDER",
        TradeMode::Snipe => "SNIPE",
    }
}

fn side_label(side: TradeSide) -> &'static str {
    match side {
        TradeSide::Buy => "BUY",
        TradeSide::Sell => "SELL",
    }
}

fn now_ms() -> Result<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now.as_millis() as u64)
}
