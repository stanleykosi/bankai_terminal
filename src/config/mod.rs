/**
 * @description
 * Configuration loading and hot-reload support for runtime parameters.
 *
 * @dependencies
 * - arc-swap: atomic swap of Arc<Config>
 * - notify: filesystem watcher for config changes
 * - serde: configuration deserialization
 *
 * @notes
 * - Supports a leading JSDoc-style header in the JSON config file.
 */
use arc_swap::ArcSwap;
use notify::{EventKind, RecursiveMode, Watcher};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::{mpsc, Arc},
};

use crate::error::{BankaiError, Result};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub endpoints: EndpointConfig,
    pub trading: TradingConfig,
    pub strategy: StrategyConfig,
    pub fees: FeeConfig,
    #[serde(default)]
    pub polymarket: PolymarketConfig,
    #[serde(default)]
    pub health: HealthConfig,
    #[serde(default)]
    pub preflight: PreflightConfig,
    #[serde(default)]
    pub allora_consumer: Option<AlloraConsumerConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EndpointConfig {
    pub binance_ws: String,
    pub polymarket_ws: String,
    pub polymarket_gamma: String,
    pub allora_rpc: String,
    pub relayer_http: String,
    pub polygon_rpc: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TradingConfig {
    pub max_volatility: f64,
    pub kill_switch_latency_ms: u64,
    pub kill_switch_clock_drift_ms: u64,
    pub kill_switch_consecutive_losses: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StrategyConfig {
    pub kelly_fraction: f64,
    pub snipe_min_edge_bps: f64,
    pub spread_offset_bps: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeeConfig {
    pub taker_fee_bps: f64,
    pub estimated_gas_bps: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolymarketConfig {
    #[serde(default)]
    pub asset_ids: Vec<String>,
    #[serde(default = "default_polymarket_asset_refresh_interval_secs")]
    pub asset_refresh_interval_secs: u64,
    #[serde(default = "default_polymarket_ping_interval_secs")]
    pub ping_interval_secs: u64,
    #[serde(default = "default_polymarket_reconnect_delay_secs")]
    pub reconnect_delay_secs: u64,
    #[serde(default = "default_polymarket_snapshot_timeout_ms")]
    pub snapshot_timeout_ms: u64,
}

impl Default for PolymarketConfig {
    fn default() -> Self {
        Self {
            asset_ids: Vec::new(),
            asset_refresh_interval_secs: default_polymarket_asset_refresh_interval_secs(),
            ping_interval_secs: default_polymarket_ping_interval_secs(),
            reconnect_delay_secs: default_polymarket_reconnect_delay_secs(),
            snapshot_timeout_ms: default_polymarket_snapshot_timeout_ms(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlloraConsumerConfig {
    pub base_url: String,
    pub chain: String,
    pub poll_interval_secs: u64,
    pub timeout_ms: u64,
    pub topics: Vec<AlloraTopicConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlloraTopicConfig {
    pub asset: String,
    pub timeframe: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthConfig {
    #[serde(default = "default_clock_drift_interval_secs")]
    pub clock_drift_check_interval_secs: u64,
    #[serde(default)]
    pub time_api_url: Option<String>,
    #[serde(default = "default_time_api_timeout_ms")]
    pub time_api_timeout_ms: u64,
    #[serde(default)]
    pub chrony_command: Option<String>,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            clock_drift_check_interval_secs: default_clock_drift_interval_secs(),
            time_api_url: None,
            time_api_timeout_ms: default_time_api_timeout_ms(),
            chrony_command: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PreflightConfig {
    #[serde(default = "default_preflight_enabled")]
    pub enabled: bool,
    #[serde(default = "default_preflight_fail_fast")]
    pub fail_fast: bool,
    #[serde(default = "default_preflight_timeout_ms")]
    pub timeout_ms: u64,
}

impl Default for PreflightConfig {
    fn default() -> Self {
        Self {
            enabled: default_preflight_enabled(),
            fail_fast: default_preflight_fail_fast(),
            timeout_ms: default_preflight_timeout_ms(),
        }
    }
}

fn default_clock_drift_interval_secs() -> u64 {
    30
}

fn default_time_api_timeout_ms() -> u64 {
    1500
}

fn default_preflight_enabled() -> bool {
    true
}

fn default_preflight_fail_fast() -> bool {
    true
}

fn default_preflight_timeout_ms() -> u64 {
    3_000
}

fn default_polymarket_ping_interval_secs() -> u64 {
    10
}

fn default_polymarket_asset_refresh_interval_secs() -> u64 {
    5
}

fn default_polymarket_reconnect_delay_secs() -> u64 {
    3
}

fn default_polymarket_snapshot_timeout_ms() -> u64 {
    10_000
}

pub struct ConfigManager {
    path: PathBuf,
    state: Arc<ArcSwap<Config>>,
}

impl ConfigManager {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let config = load_config(&path)?;
        Ok(Self {
            path,
            state: Arc::new(ArcSwap::from_pointee(config)),
        })
    }

    pub fn current(&self) -> Arc<Config> {
        self.state.load_full()
    }

    pub fn spawn_watcher(&self) -> Result<tokio::task::JoinHandle<()>> {
        let path = self.path.clone();
        let state = Arc::clone(&self.state);

        Ok(tokio::task::spawn_blocking(move || {
            if let Err(error) = watch_loop(&path, state) {
                tracing::error!(?error, "config watcher exited");
            }
        }))
    }
}

fn load_config(path: &Path) -> Result<Config> {
    let raw = fs::read_to_string(path)?;
    let stripped = strip_jsdoc_header(&raw)?;
    let trimmed = stripped.trim();
    Ok(serde_json::from_str(trimmed)?)
}

fn watch_loop(path: &Path, state: Arc<ArcSwap<Config>>) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;

    loop {
        let event = rx.recv()?;
        match event {
            Ok(event) => {
                if !is_relevant_event(&event.kind) {
                    continue;
                }
                match load_config(path) {
                    Ok(config) => {
                        state.store(Arc::new(config));
                        tracing::info!("config reloaded");
                    }
                    Err(error) => {
                        tracing::error!(?error, "failed to reload config");
                    }
                }
            }
            Err(error) => {
                tracing::error!(?error, "config watcher error");
            }
        }
    }
}

fn is_relevant_event(kind: &EventKind) -> bool {
    matches!(kind, EventKind::Modify(_) | EventKind::Create(_))
}

fn strip_jsdoc_header(contents: &str) -> Result<&str> {
    let start_index = contents
        .char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    if !contents[start_index..].starts_with("/**") {
        return Ok(contents);
    }

    let header_start = start_index + 3;
    let header_end = contents[header_start..]
        .find("*/")
        .ok_or(BankaiError::InvalidHeader)?;
    let content_start = header_start + header_end + 2;

    Ok(&contents[content_start..])
}
