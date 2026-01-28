use crate::config::Config;
use crate::error::{BankaiError, Result};
use crate::execution::signer::Eip712Signer;
use crate::security::Secrets;
use crate::storage::redis::RedisManager;
/**
 * @purpose
 * Periodically refresh open orders from Polymarket CLOB and sync to Redis.
 *
 * @notes
 * - Uses /data/orders with CLOB auth headers.
 * - Updates orders:open:* and orders:details:* hashes for UI visibility.
 */
use base64::engine::general_purpose;
use base64::Engine as _;
use chrono::DateTime;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::time::Duration;

type HmacSha256 = Hmac<sha2::Sha256>;

const DEFAULT_CHAIN_ID: u64 = 137;
const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 4_000;
const DEFAULT_ORDERS_PATH: &str = "/data/orders";
const DEFAULT_ORDERS_LIMIT: usize = 200;
const DEFAULT_REFRESH_SECS: u64 = 15;
const DEFAULT_CANCEL_ORDER_PATH: &str = "/order";

const ENV_CHAIN_ID: &str = "POLYGON_CHAIN_ID";
const ENV_ORDERS_PATH: &str = "POLYMARKET_ORDERS_PATH";
const ENV_ORDERS_LIMIT: &str = "POLYMARKET_ORDERS_LIMIT";
const ENV_CANCEL_ORDER_PATH: &str = "POLYMARKET_CANCEL_ORDER_PATH";

const HEADER_POLY_ADDRESS: &str = "POLY_ADDRESS";
const HEADER_POLY_API_KEY: &str = "POLY_API_KEY";
const HEADER_POLY_PASSPHRASE: &str = "POLY_PASSPHRASE";
const HEADER_POLY_SIGNATURE: &str = "POLY_SIGNATURE";
const HEADER_POLY_TIMESTAMP: &str = "POLY_TIMESTAMP";

pub struct OpenOrdersRefresher {
    base_url: String,
    orders_path: String,
    cancel_order_path: String,
    client: Client,
    auth: ClobAuth,
    wallet_key: String,
    interval: Duration,
    limit: usize,
    auto_cancel: bool,
    cancel_grace_ms: u64,
    order_expiry_secs: u64,
    redis: RedisManager,
}

impl OpenOrdersRefresher {
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

        let (Some(api_key), Some(api_secret), Some(api_passphrase)) =
            (api_key, api_secret, api_passphrase)
        else {
            tracing::warn!("polymarket api credentials missing; open orders refresh disabled");
            return Ok(None);
        };

        let chain_id = read_env_u64(ENV_CHAIN_ID)?.unwrap_or(DEFAULT_CHAIN_ID);
        let signer = Eip712Signer::from_secrets(secrets, chain_id)?;
        let wallet_key = format!("{}", signer.address()).to_ascii_lowercase();

        let orders_path =
            read_env_value(ENV_ORDERS_PATH).unwrap_or_else(|| DEFAULT_ORDERS_PATH.to_string());
        let cancel_order_path = read_env_value(ENV_CANCEL_ORDER_PATH)
            .unwrap_or_else(|| DEFAULT_CANCEL_ORDER_PATH.to_string());
        let limit = read_env_usize(ENV_ORDERS_LIMIT)?.unwrap_or(DEFAULT_ORDERS_LIMIT);

        let client = Client::builder()
            .timeout(Duration::from_millis(DEFAULT_REQUEST_TIMEOUT_MS))
            .build()?;

        Ok(Some(Self {
            base_url: config.endpoints.relayer_http.clone(),
            orders_path,
            cancel_order_path,
            client,
            auth: ClobAuth {
                address: wallet_key.clone(),
                api_key,
                api_passphrase,
                api_secret,
            },
            wallet_key,
            interval: Duration::from_secs(DEFAULT_REFRESH_SECS),
            limit,
            auto_cancel: config.execution.auto_cancel_orders,
            cancel_grace_ms: config
                .execution
                .order_cancel_grace_secs
                .saturating_mul(1000),
            order_expiry_secs: config.execution.order_expiry_secs,
            redis,
        }))
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "open orders refresher stopped");
            }
        })
    }

    async fn run(self) -> Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        loop {
            interval.tick().await;
            if let Err(error) = self.refresh().await {
                tracing::warn!(?error, "open orders refresh failed");
            }
        }
    }

    async fn refresh(&self) -> Result<()> {
        let orders = self.fetch_open_orders(None).await?;
        self.reconcile_open_orders(&orders).await?;
        if self.auto_cancel {
            self.maybe_cancel_orders(&orders).await?;
        }
        Ok(())
    }

    async fn fetch_open_orders(&self, asset_id: Option<&str>) -> Result<Vec<OpenOrderSnapshot>> {
        let mut url = Url::parse(&self.base_url)
            .map_err(|err| BankaiError::InvalidArgument(format!("invalid base url: {err}")))?;
        url.set_path(self.orders_path.trim_start_matches('/'));
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("limit", &self.limit.to_string());
            pairs.append_pair("status", "OPEN");
            if let Some(asset_id) = asset_id {
                pairs.append_pair("asset_id", asset_id);
            }
        }
        let path = build_request_path(&url);
        let headers = build_clob_headers(&self.auth, "GET", &path, "")?;
        let response = self.client.get(url).headers(headers).send().await?;
        if !response.status().is_success() {
            return Err(BankaiError::InvalidArgument(format!(
                "clob orders request failed with status {}",
                response.status()
            )));
        }
        Ok(response.json::<Vec<OpenOrderSnapshot>>().await?)
    }

    async fn reconcile_open_orders(&self, orders: &[OpenOrderSnapshot]) -> Result<()> {
        let open_key = open_orders_key(&self.wallet_key);
        let details_key = open_orders_details_key(&self.wallet_key);
        let mut conn = self.redis.connection();
        let mut pipe = redis::pipe();
        pipe.del(&open_key);
        pipe.del(&details_key);

        if !orders.is_empty() {
            let ids: Vec<String> = orders.iter().map(|order| order.id.clone()).collect();
            pipe.sadd(&open_key, ids);
            for order in orders {
                let payload = serde_json::to_string(order)?;
                pipe.hset(&details_key, &order.id, payload);
            }
        }

        pipe.query_async::<_, ()>(&mut conn).await?;
        Ok(())
    }

    async fn maybe_cancel_orders(&self, orders: &[OpenOrderSnapshot]) -> Result<()> {
        if orders.is_empty() {
            return Ok(());
        }
        let now_ms = current_unix_timestamp().saturating_mul(1000);
        for order in orders {
            let Some(asset_id) = order.asset_id.as_ref() else {
                continue;
            };
            let Some(market_id) = self.redis.get_token_market(asset_id).await? else {
                continue;
            };
            let metadata = self.redis.get_market_metadata(&market_id).await?;
            let end_time_ms = match metadata.end_time_ms {
                Some(value) => value,
                None => continue,
            };
            let expiry_ms = order.expiration_ms().or_else(|| {
                order
                    .created_at_ms()
                    .map(|created| created.saturating_add(self.order_expiry_secs * 1000))
            });
            if now_ms <= end_time_ms.saturating_add(self.cancel_grace_ms) {
                if let Some(expiry_ms) = expiry_ms {
                    if now_ms <= expiry_ms.saturating_add(self.cancel_grace_ms) {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            self.cancel_order(&order.id).await?;
        }
        Ok(())
    }

    async fn cancel_order(&self, order_id: &str) -> Result<()> {
        if order_id.trim().is_empty() {
            return Ok(());
        }
        let mut url = Url::parse(&self.base_url)
            .map_err(|err| BankaiError::InvalidArgument(format!("invalid base url: {err}")))?;
        url.set_path(self.cancel_order_path.trim_start_matches('/'));
        let body = serde_json::json!({ "orderID": order_id }).to_string();
        let path = build_request_path(&url);
        let headers = build_clob_headers(&self.auth, "DELETE", &path, &body)?;
        let response = self
            .client
            .delete(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        if response.status().is_success() {
            tracing::info!(order_id = %order_id, "cancelled open order");
            return Ok(());
        }
        tracing::warn!(
            order_id = %order_id,
            status = %response.status(),
            "failed to cancel open order"
        );
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct ClobAuth {
    address: String,
    api_key: String,
    api_passphrase: String,
    api_secret: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct OpenOrderSnapshot {
    pub id: String,
    #[serde(default)]
    pub asset_id: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default, alias = "expiration", alias = "expirationTime")]
    pub expiration: Option<String>,
    #[serde(
        default,
        alias = "created_at",
        alias = "createdAt",
        alias = "timestamp"
    )]
    pub created_at: Option<String>,
}

impl OpenOrderSnapshot {
    fn expiration_ms(&self) -> Option<u64> {
        let raw = self.expiration.as_ref()?;
        parse_timestamp_ms(raw)
    }

    fn created_at_ms(&self) -> Option<u64> {
        let raw = self.created_at.as_ref()?;
        parse_timestamp_ms(raw)
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

fn parse_timestamp_ms(raw: &str) -> Option<u64> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Ok(value) = trimmed.parse::<f64>() {
        if value <= 0.0 {
            return None;
        }
        let rounded = value.round() as u64;
        return Some(if rounded < 10_000_000_000 {
            rounded.saturating_mul(1000)
        } else {
            rounded
        });
    }
    if let Ok(parsed) = DateTime::parse_from_rfc3339(trimmed) {
        return Some(parsed.timestamp_millis() as u64);
    }
    None
}

fn current_unix_timestamp() -> u64 {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    duration.as_secs()
}

fn open_orders_key(address: &str) -> String {
    format!("orders:open:{address}")
}

fn open_orders_details_key(address: &str) -> String {
    format!("orders:details:{address}")
}

fn read_env_value(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

fn read_env_u64(key: &str) -> Result<Option<u64>> {
    match std::env::var(key) {
        Ok(value) => value
            .parse::<u64>()
            .map(Some)
            .map_err(|_| BankaiError::InvalidArgument(format!("{key} must be numeric"))),
        Err(_) => Ok(None),
    }
}

fn read_env_usize(key: &str) -> Result<Option<usize>> {
    match std::env::var(key) {
        Ok(value) => value
            .parse::<usize>()
            .map(Some)
            .map_err(|_| BankaiError::InvalidArgument(format!("{key} must be numeric"))),
        Err(_) => Ok(None),
    }
}
