/**
 * @purpose
 * Polymarket CLOB relayer client for posting signed orders with latency tracking.
 *
 * @dependencies
 * - reqwest: HTTP client for relayer calls
 * - serde_json: payload serialization and response parsing
 * - metrics: latency histogram recording
 *
 * @notes
 * - Congestion and 5xx errors are classified for rail failover decisions.
 */
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::time::{Duration, Instant};

use crate::error::{BankaiError, Result as BankaiResult};
use crate::telemetry::metrics;

const DEFAULT_ORDER_PATH: &str = "/order";
const DEFAULT_TIMEOUT_MS: u64 = 500;

const HEADER_BUILDER_API_KEY: &str = "POLY_BUILDER_API_KEY";
const HEADER_BUILDER_TIMESTAMP: &str = "POLY_BUILDER_TIMESTAMP";
const HEADER_BUILDER_PASSPHRASE: &str = "POLY_BUILDER_PASSPHRASE";
const HEADER_BUILDER_SIGNATURE: &str = "POLY_BUILDER_SIGNATURE";

#[derive(Debug, Clone)]
pub struct RelayerConfig {
    pub base_url: String,
    pub order_path: String,
    pub timeout: Duration,
}

impl RelayerConfig {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            order_path: DEFAULT_ORDER_PATH.to_string(),
            timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RelayerAuth {
    pub builder_api_key: String,
    pub builder_timestamp: String,
    pub builder_passphrase: String,
    pub builder_signature: String,
}

#[derive(Debug, Clone)]
pub struct RelayerResponse {
    pub status: StatusCode,
    pub body: Value,
    pub latency_ms: u64,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayerErrorKind {
    Timeout,
    Transport,
    Congestion,
    Server,
    Client,
    InvalidRequest,
    InvalidResponse,
}

#[derive(Debug, Clone)]
pub struct RelayerError {
    pub kind: RelayerErrorKind,
    pub message: String,
    pub status: Option<StatusCode>,
    pub body: Option<Value>,
    pub latency_ms: Option<u64>,
}

pub type RelayerResult<T> = std::result::Result<T, RelayerError>;

impl RelayerError {
    pub fn should_failover(&self) -> bool {
        matches!(
            self.kind,
            RelayerErrorKind::Timeout
                | RelayerErrorKind::Transport
                | RelayerErrorKind::Congestion
                | RelayerErrorKind::Server
        )
    }

    fn from_status(status: StatusCode, body: Value, latency_ms: u64) -> Self {
        let kind = classify_status(status);
        let message = format!("relayer returned status {}", status.as_u16());
        Self {
            kind,
            message,
            status: Some(status),
            body: Some(body),
            latency_ms: Some(latency_ms),
        }
    }

    fn from_request_error(error: reqwest::Error, latency_ms: u64) -> Self {
        let kind = if error.is_timeout() {
            RelayerErrorKind::Timeout
        } else {
            RelayerErrorKind::Transport
        };
        Self {
            kind,
            message: format!("relayer request failed: {error}"),
            status: None,
            body: None,
            latency_ms: Some(latency_ms),
        }
    }

    fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            kind: RelayerErrorKind::InvalidRequest,
            message: message.into(),
            status: None,
            body: None,
            latency_ms: None,
        }
    }

    fn invalid_response(message: impl Into<String>, latency_ms: u64) -> Self {
        Self {
            kind: RelayerErrorKind::InvalidResponse,
            message: message.into(),
            status: None,
            body: None,
            latency_ms: Some(latency_ms),
        }
    }
}

pub struct RelayerClient {
    config: RelayerConfig,
    client: Client,
}

impl RelayerClient {
    pub fn new(config: RelayerConfig) -> BankaiResult<Self> {
        if config.base_url.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "relayer base_url is required".to_string(),
            ));
        }
        if config.order_path.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "relayer order_path is required".to_string(),
            ));
        }
        let client = Client::builder().timeout(config.timeout).build()?;
        Ok(Self { config, client })
    }

    pub fn config(&self) -> &RelayerConfig {
        &self.config
    }

    pub async fn post_order(
        &self,
        payload: &Value,
        auth: Option<&RelayerAuth>,
    ) -> RelayerResult<RelayerResponse> {
        let url = self.order_url();
        let mut request = self.client.post(url).json(payload);
        if let Some(auth) = auth {
            request = apply_auth_headers(request, auth)?;
        }

        let start = Instant::now();
        let response = request.send().await;
        let latency_ms = duration_to_ms(start.elapsed());
        metrics::record_latency_ms(latency_ms as f64);

        let response = match response {
            Ok(response) => response,
            Err(error) => return Err(RelayerError::from_request_error(error, latency_ms)),
        };

        let status = response.status();
        let request_id = extract_request_id(response.headers());
        let body = read_response_body(response, latency_ms).await?;

        if status.is_success() {
            Ok(RelayerResponse {
                status,
                body,
                latency_ms,
                request_id,
            })
        } else {
            Err(RelayerError::from_status(status, body, latency_ms))
        }
    }

    fn order_url(&self) -> String {
        let base = self.config.base_url.trim_end_matches('/');
        let path = self.config.order_path.trim_start_matches('/');
        format!("{base}/{path}")
    }
}

fn apply_auth_headers(
    request: reqwest::RequestBuilder,
    auth: &RelayerAuth,
) -> RelayerResult<reqwest::RequestBuilder> {
    let mut headers = HeaderMap::new();
    headers.insert(
        HEADER_BUILDER_API_KEY,
        header_value(&auth.builder_api_key)?,
    );
    headers.insert(
        HEADER_BUILDER_TIMESTAMP,
        header_value(&auth.builder_timestamp)?,
    );
    headers.insert(
        HEADER_BUILDER_PASSPHRASE,
        header_value(&auth.builder_passphrase)?,
    );
    headers.insert(
        HEADER_BUILDER_SIGNATURE,
        header_value(&auth.builder_signature)?,
    );
    Ok(request.headers(headers))
}

fn header_value(value: &str) -> RelayerResult<HeaderValue> {
    HeaderValue::from_str(value).map_err(|_| {
        RelayerError::invalid_request("relayer auth header contains invalid characters")
    })
}

fn extract_request_id(headers: &HeaderMap) -> Option<String> {
    let candidates = ["x-request-id", "x-correlation-id", "x-trace-id"];
    for key in candidates {
        if let Some(value) = headers.get(key) {
            if let Ok(text) = value.to_str() {
                if !text.trim().is_empty() {
                    return Some(text.trim().to_string());
                }
            }
        }
    }
    None
}

async fn read_response_body(
    response: reqwest::Response,
    latency_ms: u64,
) -> RelayerResult<Value> {
    let bytes = response
        .bytes()
        .await
        .map_err(|error| RelayerError::invalid_response(error.to_string(), latency_ms))?;
    if bytes.is_empty() {
        return Ok(Value::Null);
    }
    if let Ok(parsed) = serde_json::from_slice::<Value>(&bytes) {
        return Ok(parsed);
    }
    Ok(Value::String(String::from_utf8_lossy(&bytes).to_string()))
}

fn classify_status(status: StatusCode) -> RelayerErrorKind {
    match status {
        StatusCode::TOO_MANY_REQUESTS
        | StatusCode::REQUEST_TIMEOUT
        | StatusCode::BAD_GATEWAY
        | StatusCode::SERVICE_UNAVAILABLE
        | StatusCode::GATEWAY_TIMEOUT => RelayerErrorKind::Congestion,
        status if status.is_server_error() => RelayerErrorKind::Server,
        status if status.is_client_error() => RelayerErrorKind::Client,
        _ => RelayerErrorKind::InvalidResponse,
    }
}

fn duration_to_ms(duration: Duration) -> u64 {
    let ms = duration.as_millis();
    if ms > u128::from(u64::MAX) {
        u64::MAX
    } else {
        ms as u64
    }
}
