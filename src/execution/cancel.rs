/**
 * @purpose
 * Polymarket CLOB cancel client for market/asset-level order cancellations.
 *
 * @dependencies
 * - reqwest: HTTP client
 * - hmac/sha2: CLOB auth headers
 *
 * @notes
 * - Uses L2 headers for authenticated DELETE requests.
 */
use base64::engine::general_purpose;
use base64::Engine as _;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use secrecy::ExposeSecret;
use serde::Deserialize;
use std::time::Duration;

use crate::error::{BankaiError, Result};
use crate::security::Secrets;

type HmacSha256 = Hmac<sha2::Sha256>;

const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 4_000;
const DEFAULT_CANCEL_MARKET_PATH: &str = "/cancel-market-orders";

const HEADER_POLY_ADDRESS: &str = "POLY_ADDRESS";
const HEADER_POLY_API_KEY: &str = "POLY_API_KEY";
const HEADER_POLY_PASSPHRASE: &str = "POLY_PASSPHRASE";
const HEADER_POLY_SIGNATURE: &str = "POLY_SIGNATURE";
const HEADER_POLY_TIMESTAMP: &str = "POLY_TIMESTAMP";

const ENV_CANCEL_MARKET_PATH: &str = "POLYMARKET_CANCEL_MARKET_PATH";

#[derive(Debug, Clone)]
pub struct CancelClientConfig {
    pub base_url: String,
    pub cancel_market_path: String,
}

impl CancelClientConfig {
    pub fn from_env(base_url: String) -> Self {
        let cancel_market_path = std::env::var(ENV_CANCEL_MARKET_PATH)
            .unwrap_or_else(|_| DEFAULT_CANCEL_MARKET_PATH.to_string());
        Self {
            base_url,
            cancel_market_path,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CancelClient {
    config: CancelClientConfig,
    client: Client,
    auth: ClobAuth,
}

impl CancelClient {
    pub fn from_env(
        config: CancelClientConfig,
        secrets: &Secrets,
        address: &str,
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
            return Ok(None);
        };

        let client = Client::builder()
            .timeout(Duration::from_millis(DEFAULT_REQUEST_TIMEOUT_MS))
            .build()?;

        Ok(Some(Self {
            config,
            client,
            auth: ClobAuth {
                address: address.to_string(),
                api_key,
                api_passphrase,
                api_secret,
            },
        }))
    }

    pub async fn cancel_market_orders(
        &self,
        market_id: &str,
        asset_id: &str,
    ) -> Result<CancelResponse> {
        let mut url = Url::parse(&self.config.base_url)
            .map_err(|err| BankaiError::InvalidArgument(format!("invalid base url: {err}")))?;
        url.set_path(self.config.cancel_market_path.trim_start_matches('/'));
        let body = serde_json::json!({
            "market": market_id,
            "asset_id": asset_id,
        })
        .to_string();
        let path = build_request_path(&url);
        let headers = build_clob_headers(&self.auth, "DELETE", &path, &body)?;
        let response = self
            .client
            .delete(url)
            .headers(headers)
            .body(body)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(BankaiError::InvalidArgument(format!(
                "cancel market orders failed with status {}",
                response.status()
            )));
        }
        let payload = response.json::<CancelResponse>().await?;
        Ok(payload)
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
pub struct CancelResponse {
    #[serde(default)]
    pub canceled: Vec<String>,
    #[serde(default)]
    pub not_canceled: serde_json::Value,
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

fn current_unix_timestamp() -> u64 {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    duration.as_secs()
}
