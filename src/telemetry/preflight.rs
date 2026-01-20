/**
 * @purpose
 * Startup preflight checks for core endpoints and dependencies.
 *
 * @dependencies
 * - reqwest: HTTP liveness checks
 * - tokio-tungstenite: WebSocket connectivity probes
 * - sqlx: TimescaleDB connectivity
 * - redis: Redis ping
 *
 * @notes
 * - Preflight results are logged before the engine starts.
 * - Fail-fast behavior is controlled by config.preflight.
 */
use futures_util::future::{join_all, BoxFuture};
use futures_util::FutureExt;
use redis::RedisError;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use sqlx::{Connection, PgConnection};
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tokio_tungstenite::connect_async;

use crate::config::Config;
use crate::error::{BankaiError, Result};

const ENV_REDIS_URL: &str = "REDIS_URL";
const ENV_TIMESCALE_URL: &str = "TIMESCALE_URL";
const ENV_COLLATERAL_ADDRESS: &str = "POLYMARKET_COLLATERAL_ADDRESS";
const ENV_CTF_ADDRESS: &str = "POLYMARKET_CTF_ADDRESS";

const DEFAULT_MIN_TIMEOUT_MS: u64 = 250;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightStatus {
    Ok,
    Failed,
    Skipped,
}

impl PreflightStatus {
    fn as_str(self) -> &'static str {
        match self {
            PreflightStatus::Ok => "ok",
            PreflightStatus::Failed => "failed",
            PreflightStatus::Skipped => "skipped",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PreflightCheck {
    pub name: String,
    pub status: PreflightStatus,
    pub required: bool,
    pub detail: String,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct PreflightReport {
    pub checks: Vec<PreflightCheck>,
}

impl PreflightReport {
    pub fn has_failures(&self) -> bool {
        self.checks
            .iter()
            .any(|check| check.required && check.status == PreflightStatus::Failed)
    }

    fn counts(&self) -> (usize, usize, usize) {
        let mut ok = 0;
        let mut failed = 0;
        let mut skipped = 0;
        for check in &self.checks {
            match check.status {
                PreflightStatus::Ok => ok += 1,
                PreflightStatus::Failed => failed += 1,
                PreflightStatus::Skipped => skipped += 1,
            }
        }
        (ok, failed, skipped)
    }
}

pub async fn run(config: &Config) -> Result<PreflightReport> {
    let timeout = resolve_timeout(config.preflight.timeout_ms);
    let client = Client::builder().timeout(timeout).build()?;
    let mut checks: Vec<BoxFuture<'static, PreflightCheck>> = Vec::new();

    checks.push(check_env_address(
        "recovery collateral address",
        ENV_COLLATERAL_ADDRESS,
        true,
    ));
    checks.push(check_env_address(
        "recovery conditional tokens address",
        ENV_CTF_ADDRESS,
        true,
    ));

    checks.push(check_redis(timeout));
    checks.push(check_timescale(timeout));

    let gamma_url = build_gamma_url(&config.endpoints.polymarket_gamma)?;
    let relayer_url = build_relayer_url(&config.endpoints.relayer_http)?;
    let polygon_rpc = config.endpoints.polygon_rpc.clone();
    let binance_ws = config.endpoints.binance_ws.clone();
    let polymarket_ws = config.endpoints.polymarket_ws.clone();

    checks.push(check_http_endpoint(
        "polymarket gamma",
        gamma_url,
        client.clone(),
        true,
        false,
    ));
    checks.push(check_http_endpoint(
        "polymarket relayer",
        relayer_url,
        client.clone(),
        true,
        true,
    ));
    checks.push(check_json_rpc(
        "polygon rpc",
        polygon_rpc,
        client.clone(),
        true,
        timeout,
    ));

    checks.push(check_websocket_endpoint(
        "binance ws",
        binance_ws,
        true,
        timeout,
    ));
    checks.push(check_websocket_endpoint(
        "polymarket ws",
        polymarket_ws,
        true,
        timeout,
    ));

    if let Some(check) = build_allora_check(config, client.clone())? {
        checks.push(check);
    }

    let results = join_all(checks).await;
    Ok(PreflightReport { checks: results })
}

pub fn log_report(report: &PreflightReport) {
    let (ok, failed, skipped) = report.counts();
    tracing::info!(ok, failed, skipped, "preflight summary");

    for check in &report.checks {
        let status = check.status.as_str();
        let latency_ms = check.latency_ms;
        let detail = check.detail.as_str();
        if check.status == PreflightStatus::Failed && check.required {
            tracing::error!(
                name = %check.name,
                status,
                required = check.required,
                latency_ms,
                detail,
                "preflight check failed"
            );
        } else if check.status == PreflightStatus::Failed {
            tracing::warn!(
                name = %check.name,
                status,
                required = check.required,
                latency_ms,
                detail,
                "preflight check failed"
            );
        } else if check.status == PreflightStatus::Skipped {
            tracing::info!(
                name = %check.name,
                status,
                required = check.required,
                detail,
                "preflight check skipped"
            );
        } else {
            tracing::info!(
                name = %check.name,
                status,
                required = check.required,
                latency_ms,
                detail,
                "preflight check ok"
            );
        }
    }
}

fn build_allora_check(
    config: &Config,
    client: Client,
) -> Result<Option<BoxFuture<'static, PreflightCheck>>> {
    let Some(allora) = config.allora_consumer.as_ref() else {
        return Ok(None);
    };
    if allora.topics.is_empty() {
        return Ok(Some(check_skipped(
            "allora consumer",
            "no topics configured",
            true,
        )));
    }
    let topic = &allora.topics[0];
    let base = allora.base_url.trim_end_matches('/');
    let chain = allora.chain.trim_matches('/');
    let asset = topic.asset.trim();
    let timeframe = topic.timeframe.trim();

    if base.is_empty() || chain.is_empty() || asset.is_empty() || timeframe.is_empty() {
        return Ok(Some(check_failed(
            "allora consumer",
            "invalid consumer config",
            true,
            None,
        )));
    }

    let url = format!("{}/{}/{}/{}", base, chain, asset, timeframe);
    Ok(Some(check_http_endpoint(
        "allora consumer",
        url,
        client,
        true,
        false,
    )))
}

fn check_env_address(
    name: &str,
    env_key: &str,
    required: bool,
) -> BoxFuture<'static, PreflightCheck> {
    let name = name.to_string();
    let env_key = env_key.to_string();
    async move {
        let Some(value) = read_env_value(&env_key) else {
            return PreflightCheck {
                name,
                status: if required {
                    PreflightStatus::Failed
                } else {
                    PreflightStatus::Skipped
                },
                required,
                detail: format!("{env_key} missing"),
                latency_ms: None,
            };
        };
        if ethers_core::types::Address::from_str(&value).is_err() {
            return PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: format!("{env_key} is not a valid address"),
                latency_ms: None,
            };
        }
        PreflightCheck {
            name,
            status: PreflightStatus::Ok,
            required,
            detail: "configured".to_string(),
            latency_ms: None,
        }
    }
    .boxed()
}

fn check_redis(timeout_duration: Duration) -> BoxFuture<'static, PreflightCheck> {
    async move {
        let Some(redis_url) = read_env_value(ENV_REDIS_URL) else {
            return PreflightCheck {
                name: "redis".to_string(),
                status: PreflightStatus::Failed,
                required: true,
                detail: format!("{ENV_REDIS_URL} missing"),
                latency_ms: None,
            };
        };

        let start = Instant::now();
        let future = async {
            let client = redis::Client::open(redis_url)?;
            let mut conn = client.get_multiplexed_async_connection().await?;
            let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
            Ok::<String, RedisError>(pong)
        };

        match timeout(timeout_duration, future).await {
            Ok(Ok(pong)) => PreflightCheck {
                name: "redis".to_string(),
                status: if pong.to_ascii_uppercase() == "PONG" {
                    PreflightStatus::Ok
                } else {
                    PreflightStatus::Failed
                },
                required: true,
                detail: format!("response {pong}"),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
            Ok(Err(error)) => PreflightCheck {
                name: "redis".to_string(),
                status: PreflightStatus::Failed,
                required: true,
                detail: format!("error {error}"),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
            Err(_) => PreflightCheck {
                name: "redis".to_string(),
                status: PreflightStatus::Failed,
                required: true,
                detail: "timeout".to_string(),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
        }
    }
    .boxed()
}

fn check_timescale(timeout_duration: Duration) -> BoxFuture<'static, PreflightCheck> {
    async move {
        let Some(timescale_url) = read_env_value(ENV_TIMESCALE_URL) else {
            return PreflightCheck {
                name: "timescale".to_string(),
                status: PreflightStatus::Failed,
                required: true,
                detail: format!("{ENV_TIMESCALE_URL} missing"),
                latency_ms: None,
            };
        };

        let start = Instant::now();
        let future = async {
            let mut conn = PgConnection::connect(&timescale_url).await?;
            sqlx::query("SELECT 1").execute(&mut conn).await?;
            Ok::<(), sqlx::Error>(())
        };

        match timeout(timeout_duration, future).await {
            Ok(Ok(())) => PreflightCheck {
                name: "timescale".to_string(),
                status: PreflightStatus::Ok,
                required: true,
                detail: "connected".to_string(),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
            Ok(Err(error)) => PreflightCheck {
                name: "timescale".to_string(),
                status: PreflightStatus::Failed,
                required: true,
                detail: format!("error {error}"),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
            Err(_) => PreflightCheck {
                name: "timescale".to_string(),
                status: PreflightStatus::Failed,
                required: true,
                detail: "timeout".to_string(),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
        }
    }
    .boxed()
}

fn check_http_endpoint(
    name: &str,
    url: String,
    client: Client,
    required: bool,
    allow_client_error: bool,
) -> BoxFuture<'static, PreflightCheck> {
    let name = name.to_string();
    async move {
        if url.trim().is_empty() {
            return PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: "url missing".to_string(),
                latency_ms: None,
            };
        }
        let start = Instant::now();
        let response = client.get(url.clone()).send().await;
        match response {
            Ok(resp) => {
                let status = resp.status();
                let latency_ms = duration_to_ms(start.elapsed());
                let status_ok = if allow_client_error {
                    !status.is_server_error()
                } else {
                    status.is_success()
                };
                let detail = format!("status {}", status.as_u16());
                PreflightCheck {
                    name,
                    status: if status_ok {
                        PreflightStatus::Ok
                    } else {
                        PreflightStatus::Failed
                    },
                    required,
                    detail,
                    latency_ms: Some(latency_ms),
                }
            }
            Err(error) => PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: format!("error {error}"),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
        }
    }
    .boxed()
}

fn check_json_rpc(
    name: &str,
    url: String,
    client: Client,
    required: bool,
    timeout_duration: Duration,
) -> BoxFuture<'static, PreflightCheck> {
    let name = name.to_string();
    async move {
        if url.trim().is_empty() {
            return PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: "url missing".to_string(),
                latency_ms: None,
            };
        }
        let start = Instant::now();
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_chainId",
            "params": []
        });
        let request = client.post(url).json(&payload);
        match timeout(timeout_duration, request.send()).await {
            Ok(Ok(resp)) => {
                let status = resp.status();
                let latency_ms = duration_to_ms(start.elapsed());
                if !status.is_success() {
                    return PreflightCheck {
                        name,
                        status: PreflightStatus::Failed,
                        required,
                        detail: format!("status {}", status.as_u16()),
                        latency_ms: Some(latency_ms),
                    };
                }
                match resp.json::<Value>().await {
                    Ok(value) => {
                        if value.get("error").is_some() {
                            PreflightCheck {
                                name,
                                status: PreflightStatus::Failed,
                                required,
                                detail: "rpc error response".to_string(),
                                latency_ms: Some(latency_ms),
                            }
                        } else {
                            PreflightCheck {
                                name,
                                status: PreflightStatus::Ok,
                                required,
                                detail: "rpc ok".to_string(),
                                latency_ms: Some(latency_ms),
                            }
                        }
                    }
                    Err(error) => PreflightCheck {
                        name,
                        status: PreflightStatus::Failed,
                        required,
                        detail: format!("decode error {error}"),
                        latency_ms: Some(latency_ms),
                    },
                }
            }
            Ok(Err(error)) => PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: format!("error {error}"),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
            Err(_) => PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: "timeout".to_string(),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
        }
    }
    .boxed()
}

fn check_websocket_endpoint(
    name: &str,
    url: String,
    required: bool,
    timeout_duration: Duration,
) -> BoxFuture<'static, PreflightCheck> {
    let name = name.to_string();
    async move {
        if url.trim().is_empty() {
            return PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: "url missing".to_string(),
                latency_ms: None,
            };
        }
        let start = Instant::now();
        let connect = connect_async(url.clone());
        match timeout(timeout_duration, connect).await {
            Ok(Ok((mut stream, _))) => {
                let _ = stream.close(None).await;
                PreflightCheck {
                    name,
                    status: PreflightStatus::Ok,
                    required,
                    detail: "connected".to_string(),
                    latency_ms: Some(duration_to_ms(start.elapsed())),
                }
            }
            Ok(Err(error)) => PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: format!("error {error}"),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
            Err(_) => PreflightCheck {
                name,
                status: PreflightStatus::Failed,
                required,
                detail: "timeout".to_string(),
                latency_ms: Some(duration_to_ms(start.elapsed())),
            },
        }
    }
    .boxed()
}

fn check_failed(
    name: &str,
    detail: &str,
    required: bool,
    latency_ms: Option<u64>,
) -> BoxFuture<'static, PreflightCheck> {
    let name = name.to_string();
    let detail = detail.to_string();
    async move {
        PreflightCheck {
            name,
            status: PreflightStatus::Failed,
            required,
            detail,
            latency_ms,
        }
    }
    .boxed()
}

fn check_skipped(name: &str, detail: &str, required: bool) -> BoxFuture<'static, PreflightCheck> {
    let name = name.to_string();
    let detail = detail.to_string();
    async move {
        PreflightCheck {
            name,
            status: PreflightStatus::Skipped,
            required,
            detail,
            latency_ms: None,
        }
    }
    .boxed()
}

fn build_gamma_url(base_url: &str) -> Result<String> {
    let base = base_url.trim_end_matches('/');
    if base.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "gamma base url is empty".to_string(),
        ));
    }
    let mut url = reqwest::Url::parse(base).map_err(|_| {
        BankaiError::InvalidArgument("gamma base url is invalid".to_string())
    })?;
    url.set_path("markets");
    url.query_pairs_mut().append_pair("limit", "1");
    Ok(url.to_string())
}

fn build_relayer_url(base_url: &str) -> Result<String> {
    let base = base_url.trim_end_matches('/');
    if base.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "relayer base url is empty".to_string(),
        ));
    }
    let mut url = reqwest::Url::parse(base).map_err(|_| {
        BankaiError::InvalidArgument("relayer base url is invalid".to_string())
    })?;
    url.set_path("order");
    Ok(url.to_string())
}

fn resolve_timeout(timeout_ms: u64) -> Duration {
    let clamped = timeout_ms.max(DEFAULT_MIN_TIMEOUT_MS);
    Duration::from_millis(clamped)
}

fn duration_to_ms(duration: Duration) -> u64 {
    let ms = duration.as_millis();
    if ms > u128::from(u64::MAX) {
        u64::MAX
    } else {
        ms as u64
    }
}

fn read_env_value(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
