/**
 * @purpose
 * Reconcile confirmed trades from the Polymarket CLOB API into Redis state.
 *
 * @dependencies
 * - reqwest: REST calls
 * - hmac/sha2: CLOB auth headers
 * - redis: position updates
 *
 * @notes
 * - Uses /data/trades with L2 headers.
 * - De-duplicates trade ids in Redis to avoid double counting.
 */
use base64::engine::general_purpose;
use base64::Engine as _;
use chrono::DateTime;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use secrecy::ExposeSecret;
use serde::Deserialize;
use std::collections::HashSet;
use std::time::Duration;

use crate::accounting::keys::REALIZED_PNL_KEY;
use crate::accounting::trade_events::{is_seen_trade, mark_seen_trade, record_realized_pnl_event};
use crate::config::Config;
use crate::error::{BankaiError, Result};
use crate::execution::signer::Eip712Signer;
use crate::security::Secrets;
use crate::storage::redis::RedisManager;

type HmacSha256 = Hmac<sha2::Sha256>;

const DEFAULT_CHAIN_ID: u64 = 137;
const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 4_000;
const DEFAULT_TRADES_PATH: &str = "/data/trades";
const DEFAULT_TRADES_LIMIT: usize = 200;
const DEFAULT_LOOKBACK_SECS: u64 = 3600;
const HEADER_POLY_ADDRESS: &str = "POLY_ADDRESS";
const HEADER_POLY_API_KEY: &str = "POLY_API_KEY";
const HEADER_POLY_PASSPHRASE: &str = "POLY_PASSPHRASE";
const HEADER_POLY_SIGNATURE: &str = "POLY_SIGNATURE";
const HEADER_POLY_TIMESTAMP: &str = "POLY_TIMESTAMP";

const ENV_CHAIN_ID: &str = "POLYGON_CHAIN_ID";
const ENV_TRADES_PATH: &str = "POLYMARKET_TRADES_PATH";
const ENV_TRADES_LIMIT: &str = "POLYMARKET_TRADES_LIMIT";

pub struct TradeReconciler {
    base_url: String,
    trades_path: String,
    client: Client,
    auth: ClobAuth,
    wallet_key: String,
    interval: Duration,
    limit: usize,
    redis: RedisManager,
}

impl TradeReconciler {
    pub fn from_env(
        config: &Config,
        secrets: &Secrets,
        redis: RedisManager,
    ) -> Result<Option<Self>> {
        let api_key = secrets
            .polymarket_api_key
            .as_ref()
            .map(|value| value.expose_secret().to_string());
        let api_secret = secrets
            .polymarket_api_secret
            .as_ref()
            .map(|value| value.expose_secret().to_string());
        let api_passphrase = secrets
            .polymarket_api_passphrase
            .as_ref()
            .map(|value| value.expose_secret().to_string());

        let Some(api_key) = api_key else {
            tracing::warn!("polymarket api key missing; trade reconciler disabled");
            return Ok(None);
        };
        let Some(api_secret) = api_secret else {
            tracing::warn!("polymarket api secret missing; trade reconciler disabled");
            return Ok(None);
        };
        let Some(api_passphrase) = api_passphrase else {
            tracing::warn!("polymarket api passphrase missing; trade reconciler disabled");
            return Ok(None);
        };

        let chain_id = read_env_u64(ENV_CHAIN_ID)?.unwrap_or(DEFAULT_CHAIN_ID);
        let signer = Eip712Signer::from_secrets(secrets, chain_id)?;
        let wallet_key = format!("{}", signer.address()).to_ascii_lowercase();

        let trades_path = read_env_value(ENV_TRADES_PATH)
            .unwrap_or_else(|| DEFAULT_TRADES_PATH.to_string());
        let limit = read_env_usize(ENV_TRADES_LIMIT)?.unwrap_or(DEFAULT_TRADES_LIMIT);

        let client = Client::builder()
            .timeout(Duration::from_millis(DEFAULT_REQUEST_TIMEOUT_MS))
            .build()?;

        Ok(Some(Self {
            base_url: config.endpoints.relayer_http.clone(),
            trades_path,
            client,
            auth: ClobAuth {
                address: wallet_key.clone(),
                api_key,
                api_passphrase,
                api_secret,
            },
            wallet_key,
            interval: Duration::from_secs(config.execution.trade_reconcile_interval_secs.max(3)),
            limit,
            redis,
        }))
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "trade reconciler stopped");
            }
        })
    }

    async fn run(self) -> Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        loop {
            interval.tick().await;
            if let Err(error) = self.reconcile().await {
                tracing::warn!(?error, "trade reconcile failed");
            }
        }
    }

    async fn reconcile(&self) -> Result<()> {
        let after = current_unix_timestamp().saturating_sub(DEFAULT_LOOKBACK_SECS);
        let mut trades = self.fetch_trades(Some(&self.wallet_key), None, after).await?;
        if trades.is_empty() {
            trades = self.fetch_trades(None, Some(&self.wallet_key), after).await?;
        }

        if trades.is_empty() {
            return Ok(());
        }

        let mut seen = HashSet::new();
        for trade in trades {
            if !seen.insert(trade.id.clone()) {
                continue;
            }
            if !trade.is_confirmed() {
                continue;
            }
            if trade.asset_id.is_none() || trade.side.is_none() {
                continue;
            }
            let asset_id = trade.asset_id.clone().unwrap();
            let size = trade.size.parse::<f64>().unwrap_or(0.0);
            let price = trade.price.parse::<f64>().unwrap_or(0.0);
            if size <= 0.0 || price <= 0.0 {
                continue;
            }

            if is_seen_trade(&self.redis, &self.wallet_key, &trade.id).await? {
                continue;
            }
            mark_seen_trade(&self.redis, &self.wallet_key, &trade.id).await?;

            match trade.side.as_deref().unwrap_or("").to_ascii_lowercase().as_str() {
                "buy" => {
                    self.apply_buy(&asset_id, size, price).await?;
                }
                "sell" => {
                    let timestamp = trade
                        .timestamp()
                        .unwrap_or_else(current_unix_timestamp);
                    self.apply_sell(&asset_id, size, price, &trade.id, timestamp)
                        .await?;
                }
                _ => continue,
            }

            let message = format!(
                "[TRADE] {} {} size={:.4} price={:.4}",
                trade.side.unwrap_or_else(|| "UNKNOWN".to_string()).to_uppercase(),
                asset_id,
                size,
                price
            );
            let _ = self.redis.push_order_log(&message, 8).await;
        }

        Ok(())
    }

    async fn apply_buy(&self, asset_id: &str, size: f64, price: f64) -> Result<()> {
        let current = self
            .redis
            .get_tracked_position(&self.wallet_key, asset_id)
            .await?;
        let entry = self
            .redis
            .get_entry_price(&self.wallet_key, asset_id)
            .await?
            .unwrap_or(0.0);
        let new_balance = current + size;
        let weighted = if current > 0.0 && entry > 0.0 {
            ((entry * current) + (price * size)) / new_balance
        } else {
            price
        };
        self.redis
            .set_tracked_position(&self.wallet_key, asset_id, new_balance)
            .await?;
        self.redis
            .set_entry_price(&self.wallet_key, asset_id, weighted)
            .await?;
        let peak = self
            .redis
            .get_peak_price(&self.wallet_key, asset_id)
            .await?
            .unwrap_or(weighted);
        if price > peak {
            self.redis
                .set_peak_price(&self.wallet_key, asset_id, price)
                .await?;
        }
        Ok(())
    }

    async fn apply_sell(
        &self,
        asset_id: &str,
        size: f64,
        price: f64,
        trade_id: &str,
        timestamp: u64,
    ) -> Result<()> {
        let current = self
            .redis
            .get_tracked_position(&self.wallet_key, asset_id)
            .await?;
        if current <= 0.0 {
            return Ok(());
        }
        let entry = self
            .redis
            .get_entry_price(&self.wallet_key, asset_id)
            .await?
            .unwrap_or(0.0);
        let new_balance = (current - size).max(0.0);
        self.redis
            .set_tracked_position(&self.wallet_key, asset_id, new_balance)
            .await?;
        if new_balance <= 0.0 {
            self.redis
                .set_entry_price(&self.wallet_key, asset_id, 0.0)
                .await?;
            self.redis
                .set_peak_price(&self.wallet_key, asset_id, 0.0)
                .await?;
        }

        if entry > 0.0 {
            let realized = (price - entry) * size;
            let _ = self.redis.incr_float(REALIZED_PNL_KEY, realized).await?;
            let _ =
                record_realized_pnl_event(&self.redis, &self.wallet_key, trade_id, timestamp, realized)
                    .await;
        }
        Ok(())
    }

    async fn fetch_trades(
        &self,
        maker: Option<&str>,
        taker: Option<&str>,
        after: u64,
    ) -> Result<Vec<TradeSnapshot>> {
        let mut url = Url::parse(self.base_url.trim_end_matches('/'))
            .map_err(|_| BankaiError::InvalidArgument("clob base url is invalid".to_string()))?;
        url.set_path(self.trades_path.trim_start_matches('/'));
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(maker) = maker {
                if !maker.trim().is_empty() {
                    pairs.append_pair("maker", maker.trim());
                }
            }
            if let Some(taker) = taker {
                if !taker.trim().is_empty() {
                    pairs.append_pair("taker", taker.trim());
                }
            }
            if after > 0 {
                pairs.append_pair("after", &after.to_string());
            }
            if self.limit > 0 {
                pairs.append_pair("limit", &self.limit.to_string());
            }
        }
        let request_path = build_request_path(&url);
        let headers = build_clob_headers(&self.auth, "GET", &request_path, "")?;
        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .error_for_status()?;
        let trades: Vec<TradeSnapshot> = response.json().await.unwrap_or_default();
        Ok(trades)
    }
}

#[derive(Debug, Clone)]
struct ClobAuth {
    address: String,
    api_key: String,
    api_passphrase: String,
    api_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
struct TradeSnapshot {
    pub id: String,
    #[serde(default)]
    pub asset_id: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub size: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub last_update: Option<String>,
    #[serde(default)]
    pub match_time: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
}

impl TradeSnapshot {
    fn is_confirmed(&self) -> bool {
        matches!(
            self.status.as_deref().unwrap_or("").to_ascii_uppercase().as_str(),
            "CONFIRMED"
        )
    }

    fn timestamp(&self) -> Option<u64> {
        if let Some(value) = self.timestamp.as_deref() {
            if let Some(parsed) = parse_timestamp(value) {
                return Some(parsed);
            }
        }
        if let Some(value) = self.last_update.as_deref() {
            if let Some(parsed) = parse_timestamp(value) {
                return Some(parsed);
            }
        }
        if let Some(value) = self.match_time.as_deref() {
            return parse_timestamp(value);
        }
        None
    }
}

fn build_clob_headers(auth: &ClobAuth, method: &str, path: &str, body: &str) -> Result<HeaderMap> {
    let timestamp = current_unix_timestamp();
    let signature = build_hmac_signature(&auth.api_secret, timestamp, method, path, body)?;

    let mut headers = HeaderMap::new();
    headers.insert(HEADER_POLY_ADDRESS, header_value(&auth.address)?);
    headers.insert(HEADER_POLY_API_KEY, header_value(&auth.api_key)?);
    headers.insert(HEADER_POLY_PASSPHRASE, header_value(&auth.api_passphrase)?);
    headers.insert(HEADER_POLY_SIGNATURE, header_value(&signature)?);
    headers.insert(HEADER_POLY_TIMESTAMP, header_value(&timestamp.to_string())?);
    Ok(headers)
}

fn build_hmac_signature(
    secret: &str,
    timestamp: u64,
    method: &str,
    path: &str,
    body: &str,
) -> Result<String> {
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

fn build_request_path(url: &Url) -> String {
    match url.query() {
        Some(query) => format!("{}?{}", url.path(), query),
        None => url.path().to_string(),
    }
}

fn header_value(value: &str) -> Result<HeaderValue> {
    HeaderValue::from_str(value).map_err(|_| {
        BankaiError::InvalidArgument("header value contains invalid characters".to_string())
    })
}

fn read_env_value(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn read_env_u64(key: &str) -> Result<Option<u64>> {
    let Some(raw) = read_env_value(key) else {
        return Ok(None);
    };
    raw.parse::<u64>()
        .map(Some)
        .map_err(|_| BankaiError::InvalidArgument(format!("{key} must be a valid integer")))
}

fn read_env_usize(key: &str) -> Result<Option<usize>> {
    let Some(raw) = read_env_value(key) else {
        return Ok(None);
    };
    raw.parse::<usize>()
        .map(Some)
        .map_err(|_| BankaiError::InvalidArgument(format!("{key} must be a valid integer")))
}

fn current_unix_timestamp() -> u64 {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    duration.as_secs()
}

#[allow(dead_code)]
fn parse_timestamp(value: &str) -> Option<u64> {
    if let Ok(parsed) = value.parse::<u64>() {
        return Some(parsed);
    }
    if let Ok(parsed) = DateTime::parse_from_rfc3339(value) {
        return Some(parsed.timestamp() as u64);
    }
    if let Ok(parsed) = DateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S%.f%z") {
        return Some(parsed.timestamp() as u64);
    }
    None
}
