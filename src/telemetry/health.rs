/**
 * @description
 * System health monitor for clock drift checks and kill switch updates.
 *
 * @dependencies
 * - reqwest: external time API polling
 * - tokio: async scheduling
 *
 * @notes
 * - Clock drift can be sourced from a time API or chrony tracking output.
 */
use reqwest::Client;
use serde::Deserialize;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config::HealthConfig;
use crate::engine::risk::RiskState;
use crate::error::{BankaiError, Result};

#[derive(Debug, Clone)]
pub struct HealthMonitorConfig {
    pub check_interval: Duration,
    pub time_api_url: Option<String>,
    pub time_api_timeout: Duration,
    pub chrony_command: Option<String>,
}

impl From<&HealthConfig> for HealthMonitorConfig {
    fn from(config: &HealthConfig) -> Self {
        Self {
            check_interval: Duration::from_secs(config.clock_drift_check_interval_secs),
            time_api_url: config.time_api_url.clone(),
            time_api_timeout: Duration::from_millis(config.time_api_timeout_ms),
            chrony_command: config.chrony_command.clone(),
        }
    }
}

#[derive(Debug, Clone)]
enum ClockDriftSource {
    TimeApi { url: String },
    Chrony { command: String },
    Disabled,
}

pub struct HealthMonitor {
    risk: Arc<RiskState>,
    config: HealthMonitorConfig,
    client: Client,
    drift_source: ClockDriftSource,
}

impl HealthMonitor {
    pub fn from_config(risk: Arc<RiskState>, config: &HealthConfig) -> Result<Self> {
        Self::new(risk, HealthMonitorConfig::from(config))
    }

    pub fn new(risk: Arc<RiskState>, config: HealthMonitorConfig) -> Result<Self> {
        let client = Client::builder().timeout(config.time_api_timeout).build()?;
        let drift_source = select_drift_source(&config);
        if matches!(drift_source, ClockDriftSource::Disabled) {
            tracing::warn!("clock drift checks disabled (no time API or chrony configured)");
        }
        Ok(Self {
            risk,
            config,
            client,
            drift_source,
        })
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            self.run().await;
        })
    }

    async fn run(self) {
        let interval = if self.config.check_interval.is_zero() {
            Duration::from_secs(1)
        } else {
            self.config.check_interval
        };

        loop {
            if let Err(error) = self.check_clock_drift().await {
                tracing::warn!(?error, "clock drift check failed");
            }
            tokio::time::sleep(interval).await;
        }
    }

    async fn check_clock_drift(&self) -> Result<()> {
        match &self.drift_source {
            ClockDriftSource::TimeApi { url } => {
                let drift_ms = self.check_time_api(url).await?;
                self.risk.record_clock_drift_ms(drift_ms);
                tracing::info!(drift_ms, "clock drift updated from time API");
            }
            ClockDriftSource::Chrony { command } => {
                let drift_ms = self.check_chrony(command)?;
                self.risk.record_clock_drift_ms(drift_ms);
                tracing::info!(drift_ms, "clock drift updated from chrony");
            }
            ClockDriftSource::Disabled => {}
        }
        Ok(())
    }

    async fn check_time_api(&self, url: &str) -> Result<i64> {
        let response = self
            .client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<TimeApiResponse>()
            .await?;
        if response.unixtime < 0 {
            return Err(BankaiError::InvalidArgument(
                "time API returned negative unix time".to_string(),
            ));
        }
        let remote = UNIX_EPOCH + Duration::from_secs(response.unixtime as u64);
        Ok(calculate_drift_ms(SystemTime::now(), remote))
    }

    fn check_chrony(&self, command: &str) -> Result<i64> {
        let output = Command::new(command).arg("tracking").output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BankaiError::InvalidArgument(format!(
                "chrony command failed: {stderr}"
            )));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let drift_seconds = parse_chrony_offset_seconds(&stdout).ok_or_else(|| {
            BankaiError::InvalidArgument("unable to parse chrony tracking output".to_string())
        })?;
        Ok((drift_seconds * 1000.0).round() as i64)
    }
}

#[derive(Debug, Deserialize)]
struct TimeApiResponse {
    unixtime: i64,
}

fn select_drift_source(config: &HealthMonitorConfig) -> ClockDriftSource {
    if let Some(url) = config.time_api_url.as_ref() {
        let trimmed = url.trim();
        if !trimmed.is_empty() {
            return ClockDriftSource::TimeApi {
                url: trimmed.to_string(),
            };
        }
    }

    if let Some(command) = config.chrony_command.as_ref() {
        let trimmed = command.trim();
        if !trimmed.is_empty() {
            return ClockDriftSource::Chrony {
                command: trimmed.to_string(),
            };
        }
    }

    ClockDriftSource::Disabled
}

fn calculate_drift_ms(local: SystemTime, remote: SystemTime) -> i64 {
    match local.duration_since(remote) {
        Ok(delta) => delta.as_millis() as i64,
        Err(error) => -(error.duration().as_millis() as i64),
    }
}

fn parse_chrony_offset_seconds(output: &str) -> Option<f64> {
    for line in output.lines() {
        let lower = line.to_ascii_lowercase();
        if lower.contains("system time") || lower.contains("last offset") {
            if let Some(value) = extract_first_float(line) {
                let mut drift = value;
                if drift >= 0.0 && lower.contains("slow") {
                    drift = -drift;
                }
                return Some(drift);
            }
        }
    }
    None
}

fn extract_first_float(line: &str) -> Option<f64> {
    for token in line.split_whitespace() {
        let cleaned = token.trim_matches(|c: char| {
            !c.is_ascii_digit() && c != '.' && c != '-' && c != '+'
        });
        if cleaned.is_empty() || cleaned == "-" || cleaned == "+" {
            continue;
        }
        if let Ok(value) = cleaned.parse::<f64>() {
            return Some(value);
        }
    }
    None
}
