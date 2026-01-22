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
 * - Builder headers are computed from the exact request body string when credentials are provided.
 */
use base64::engine::general_purpose;
use base64::Engine as _;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::time::{Duration, Instant};

use crate::error::{BankaiError, Result as BankaiResult};
use crate::telemetry::metrics;

const DEFAULT_ORDER_PATH: &str = "/order";
const DEFAULT_TIMEOUT_MS: u64 = 500;

const HEADER_POLY_ADDRESS: &str = "POLY_ADDRESS";
const HEADER_POLY_API_KEY: &str = "POLY_API_KEY";
const HEADER_POLY_PASSPHRASE: &str = "POLY_PASSPHRASE";
const HEADER_POLY_SIGNATURE: &str = "POLY_SIGNATURE";
const HEADER_POLY_TIMESTAMP: &str = "POLY_TIMESTAMP";

const HEADER_BUILDER_API_KEY: &str = "POLY_BUILDER_API_KEY";
const HEADER_BUILDER_TIMESTAMP: &str = "POLY_BUILDER_TIMESTAMP";
const HEADER_BUILDER_PASSPHRASE: &str = "POLY_BUILDER_PASSPHRASE";
const HEADER_BUILDER_SIGNATURE: &str = "POLY_BUILDER_SIGNATURE";

type HmacSha256 = Hmac<sha2::Sha256>;

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
    pub address: String,
    pub api_key: String,
    pub passphrase: String,
    pub signature: String,
    pub timestamp: String,
    pub builder: Option<RelayerBuilderAuth>,
}

#[derive(Debug, Clone)]
pub enum RelayerBuilderAuth {
    Credentials(RelayerBuilderCredentials),
    Headers(RelayerBuilderHeaders),
}

#[derive(Debug, Clone)]
pub struct RelayerBuilderCredentials {
    pub api_key: String,
    pub secret: String,
    pub passphrase: String,
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RelayerBuilderHeaders {
    pub api_key: String,
    pub timestamp: String,
    pub passphrase: String,
    pub signature: String,
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
        let body = serde_json::to_string(payload)
            .map_err(|err| RelayerError::invalid_request(format!("payload json error: {err}")))?;
        let mut request = self
            .client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.clone());
        if let Some(auth) = auth {
            request = apply_auth_headers(request, auth, &self.order_path(), &body)?;
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

    fn order_path(&self) -> String {
        let path = self.config.order_path.trim();
        if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        }
    }
}

fn apply_auth_headers(
    request: reqwest::RequestBuilder,
    auth: &RelayerAuth,
    path: &str,
    body: &str,
) -> RelayerResult<reqwest::RequestBuilder> {
    let mut headers = HeaderMap::new();
    headers.insert(HEADER_POLY_ADDRESS, header_value(&auth.address)?);
    headers.insert(HEADER_POLY_API_KEY, header_value(&auth.api_key)?);
    headers.insert(HEADER_POLY_PASSPHRASE, header_value(&auth.passphrase)?);
    headers.insert(HEADER_POLY_SIGNATURE, header_value(&auth.signature)?);
    headers.insert(HEADER_POLY_TIMESTAMP, header_value(&auth.timestamp)?);

    apply_builder_headers(&mut headers, auth, path, body)?;

    Ok(request.headers(headers))
}

fn apply_builder_headers(
    headers: &mut HeaderMap,
    auth: &RelayerAuth,
    path: &str,
    body: &str,
) -> RelayerResult<()> {
    let Some(builder) = auth.builder.as_ref() else {
        return Ok(());
    };

    let payload = match builder {
        RelayerBuilderAuth::Headers(payload) => payload.clone(),
        RelayerBuilderAuth::Credentials(credentials) => {
            build_builder_headers(credentials, "POST", path, body)?
        }
    };

    headers.insert(HEADER_BUILDER_API_KEY, header_value(&payload.api_key)?);
    headers.insert(
        HEADER_BUILDER_PASSPHRASE,
        header_value(&payload.passphrase)?,
    );
    headers.insert(HEADER_BUILDER_SIGNATURE, header_value(&payload.signature)?);
    headers.insert(HEADER_BUILDER_TIMESTAMP, header_value(&payload.timestamp)?);

    Ok(())
}

fn build_builder_headers(
    credentials: &RelayerBuilderCredentials,
    method: &str,
    path: &str,
    body: &str,
) -> RelayerResult<RelayerBuilderHeaders> {
    let timestamp = credentials.timestamp.unwrap_or_else(current_unix_timestamp);
    let signature = build_builder_signature(&credentials.secret, timestamp, method, path, body)?;
    Ok(RelayerBuilderHeaders {
        api_key: credentials.api_key.clone(),
        passphrase: credentials.passphrase.clone(),
        signature,
        timestamp: timestamp.to_string(),
    })
}

fn build_builder_signature(
    secret: &str,
    timestamp: u64,
    method: &str,
    path: &str,
    body: &str,
) -> RelayerResult<String> {
    let key = general_purpose::STANDARD.decode(secret).map_err(|err| {
        RelayerError::invalid_request(format!("builder secret decode error: {err}"))
    })?;
    let message = format!("{timestamp}{method}{path}{body}");
    let mut mac = HmacSha256::new_from_slice(&key)
        .map_err(|_| RelayerError::invalid_request("builder secret is invalid for hmac"))?;
    mac.update(message.as_bytes());
    let result = mac.finalize().into_bytes();
    let signature = general_purpose::STANDARD.encode(result);
    Ok(signature.replace('+', "-").replace('/', "_"))
}

fn current_unix_timestamp() -> u64 {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    duration.as_secs()
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

async fn read_response_body(response: reqwest::Response, latency_ms: u64) -> RelayerResult<Value> {
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
