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
    pub execution: ExecutionConfig,
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
    pub chainlink_ws: String,
    pub polymarket_ws: String,
    #[serde(default)]
    pub polymarket_user_ws: Option<String>,
    pub polymarket_gamma: String,
    pub allora_rpc: String,
    pub relayer_http: String,
    pub polygon_rpc: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TradingConfig {
    pub max_volatility: f64,
    pub kill_switch_latency_ms: u64,
    pub kill_switch_latency_consecutive: u32,
    pub kill_switch_latency_recovery: u32,
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
#[serde(untagged)]
enum StrategyOverrideFile {
    Wrapped(StrategyOverrideWrapper),
    Direct(StrategyConfig),
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
struct StrategyOverrideWrapper {
    #[serde(default)]
    version: Option<u64>,
    #[serde(default)]
    updated_at: Option<String>,
    strategy: StrategyConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeeConfig {
    pub taker_fee_bps: f64,
    pub estimated_gas_bps: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionConfig {
    #[serde(default = "default_execution_enabled")]
    pub enable_trading: bool,
    #[serde(default = "default_execution_min_order_usdc")]
    pub min_order_usdc: f64,
    #[serde(default = "default_execution_max_order_usdc")]
    pub max_order_usdc: f64,
    #[serde(default = "default_execution_default_order_usdc")]
    pub default_order_usdc: f64,
    #[serde(default = "default_execution_max_slippage_bps")]
    pub max_slippage_bps: f64,
    #[serde(default = "default_execution_max_impact_bps")]
    pub max_impact_bps: f64,
    #[serde(default = "default_execution_probability_scale")]
    pub probability_scale: f64,
    #[serde(default = "default_execution_probability_max_offset")]
    pub probability_max_offset: f64,
    #[serde(default = "default_execution_signal_direction_gate")]
    pub signal_direction_gate: bool,
    #[serde(default = "default_execution_contrarian_min_edge_bps")]
    pub contrarian_min_edge_bps: f64,
    #[serde(default = "default_execution_contrarian_confidence_min")]
    pub contrarian_confidence_min: f64,
    #[serde(default = "default_execution_min_volatility")]
    pub min_volatility: f64,
    #[serde(default = "default_execution_staleness_max_ratio")]
    pub staleness_max_ratio: f64,
    #[serde(default = "default_execution_order_expiry_secs")]
    pub order_expiry_secs: u64,
    #[serde(default = "default_execution_order_cooldown_secs")]
    pub order_cooldown_secs: u64,
    #[serde(default = "default_execution_allowance_target_usdc")]
    pub allowance_target_usdc: f64,
    #[serde(default = "default_execution_allowance_check_interval_secs")]
    pub allowance_check_interval_secs: u64,
    #[serde(default = "default_execution_trade_reconcile_interval_secs")]
    pub trade_reconcile_interval_secs: u64,
    #[serde(default = "default_execution_prefer_ws_reconcile")]
    pub prefer_ws_reconcile: bool,
    #[serde(default = "default_execution_auto_cancel_orders")]
    pub auto_cancel_orders: bool,
    #[serde(default = "default_execution_order_cancel_grace_secs")]
    pub order_cancel_grace_secs: u64,
    #[serde(default = "default_execution_cancel_before_replace")]
    pub cancel_before_replace: bool,
    #[serde(default = "default_execution_ladder_order_type")]
    pub ladder_order_type: String,
    #[serde(default = "default_execution_snipe_order_type")]
    pub snipe_order_type: String,
    #[serde(default = "default_execution_post_only_ladder")]
    pub post_only_ladder: bool,
    #[serde(default = "default_execution_gtd_min_expiry_secs")]
    pub gtd_min_expiry_secs: u64,
    #[serde(default = "default_execution_relayer_max_retries")]
    pub relayer_max_retries: u32,
    #[serde(default = "default_execution_relayer_backoff_ms")]
    pub relayer_backoff_ms: u64,
    #[serde(default = "default_execution_relayer_backoff_max_ms")]
    pub relayer_backoff_max_ms: u64,
    #[serde(default = "default_execution_idempotency_ttl_secs")]
    pub idempotency_ttl_secs: u64,
    #[serde(default = "default_execution_bankroll_refresh_secs")]
    pub bankroll_refresh_secs: u64,
    #[serde(default = "default_execution_no_money_mode")]
    pub no_money_mode: bool,
    #[serde(default = "default_execution_paper_stats_persist")]
    pub paper_stats_persist: bool,
    #[serde(default = "default_execution_paper_start_bankroll_usdc")]
    pub paper_start_bankroll_usdc: f64,
    #[serde(default = "default_execution_paper_latency_ms")]
    pub paper_latency_ms: u64,
    #[serde(default = "default_execution_paper_slippage_bps")]
    pub paper_slippage_bps: f64,
    #[serde(default = "default_execution_signal_alignment_max_secs")]
    pub signal_alignment_max_secs: u64,
    #[serde(default = "default_execution_signal_alignment_max_secs_sol")]
    pub signal_alignment_max_secs_sol: u64,
    #[serde(default = "default_execution_take_profit_bps")]
    pub take_profit_bps: f64,
    #[serde(default = "default_execution_stop_loss_bps")]
    pub stop_loss_bps: f64,
    #[serde(default = "default_execution_trailing_stop_bps")]
    pub trailing_stop_bps: f64,
    #[serde(default = "default_execution_close_fraction")]
    pub close_fraction: f64,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            enable_trading: default_execution_enabled(),
            min_order_usdc: default_execution_min_order_usdc(),
            max_order_usdc: default_execution_max_order_usdc(),
            default_order_usdc: default_execution_default_order_usdc(),
            max_slippage_bps: default_execution_max_slippage_bps(),
            max_impact_bps: default_execution_max_impact_bps(),
            probability_scale: default_execution_probability_scale(),
            probability_max_offset: default_execution_probability_max_offset(),
            signal_direction_gate: default_execution_signal_direction_gate(),
            contrarian_min_edge_bps: default_execution_contrarian_min_edge_bps(),
            contrarian_confidence_min: default_execution_contrarian_confidence_min(),
            min_volatility: default_execution_min_volatility(),
            staleness_max_ratio: default_execution_staleness_max_ratio(),
            order_expiry_secs: default_execution_order_expiry_secs(),
            order_cooldown_secs: default_execution_order_cooldown_secs(),
            allowance_target_usdc: default_execution_allowance_target_usdc(),
            allowance_check_interval_secs: default_execution_allowance_check_interval_secs(),
            trade_reconcile_interval_secs: default_execution_trade_reconcile_interval_secs(),
            prefer_ws_reconcile: default_execution_prefer_ws_reconcile(),
            auto_cancel_orders: default_execution_auto_cancel_orders(),
            order_cancel_grace_secs: default_execution_order_cancel_grace_secs(),
            cancel_before_replace: default_execution_cancel_before_replace(),
            ladder_order_type: default_execution_ladder_order_type(),
            snipe_order_type: default_execution_snipe_order_type(),
            post_only_ladder: default_execution_post_only_ladder(),
            gtd_min_expiry_secs: default_execution_gtd_min_expiry_secs(),
            relayer_max_retries: default_execution_relayer_max_retries(),
            relayer_backoff_ms: default_execution_relayer_backoff_ms(),
            relayer_backoff_max_ms: default_execution_relayer_backoff_max_ms(),
            idempotency_ttl_secs: default_execution_idempotency_ttl_secs(),
            bankroll_refresh_secs: default_execution_bankroll_refresh_secs(),
            no_money_mode: default_execution_no_money_mode(),
            paper_stats_persist: default_execution_paper_stats_persist(),
            paper_start_bankroll_usdc: default_execution_paper_start_bankroll_usdc(),
            paper_latency_ms: default_execution_paper_latency_ms(),
            paper_slippage_bps: default_execution_paper_slippage_bps(),
            signal_alignment_max_secs: default_execution_signal_alignment_max_secs(),
            signal_alignment_max_secs_sol: default_execution_signal_alignment_max_secs_sol(),
            take_profit_bps: default_execution_take_profit_bps(),
            stop_loss_bps: default_execution_stop_loss_bps(),
            trailing_stop_bps: default_execution_trailing_stop_bps(),
            close_fraction: default_execution_close_fraction(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolymarketConfig {
    #[serde(default)]
    pub asset_ids: Vec<String>,
    #[serde(default = "default_polymarket_asset_refresh_interval_secs")]
    pub asset_refresh_interval_secs: u64,
    #[serde(default = "default_polymarket_asset_stale_timeout_secs")]
    pub asset_stale_timeout_secs: u64,
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
            asset_stale_timeout_secs: default_polymarket_asset_stale_timeout_secs(),
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
    #[serde(default)]
    pub api_key: Option<String>,
    pub topics: Vec<AlloraTopicConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlloraTopicConfig {
    pub asset: String,
    pub timeframe: String,
    #[serde(default)]
    pub topic_id: Option<u64>,
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

fn default_polymarket_asset_stale_timeout_secs() -> u64 {
    20
}

fn default_polymarket_reconnect_delay_secs() -> u64 {
    3
}

fn default_polymarket_snapshot_timeout_ms() -> u64 {
    10_000
}

fn default_execution_enabled() -> bool {
    false
}

fn default_execution_min_order_usdc() -> f64 {
    5.0
}

fn default_execution_max_order_usdc() -> f64 {
    50.0
}

fn default_execution_default_order_usdc() -> f64 {
    10.0
}

fn default_execution_max_slippage_bps() -> f64 {
    50.0
}

fn default_execution_max_impact_bps() -> f64 {
    100.0
}

fn default_execution_probability_scale() -> f64 {
    1.0
}

fn default_execution_probability_max_offset() -> f64 {
    0.35
}

fn default_execution_signal_direction_gate() -> bool {
    false
}

fn default_execution_contrarian_min_edge_bps() -> f64 {
    0.0
}

fn default_execution_contrarian_confidence_min() -> f64 {
    0.7
}

fn default_execution_min_volatility() -> f64 {
    0.001
}

fn default_execution_staleness_max_ratio() -> f64 {
    0.05
}

fn default_execution_order_expiry_secs() -> u64 {
    90
}

fn default_execution_order_cooldown_secs() -> u64 {
    5
}

fn default_execution_allowance_target_usdc() -> f64 {
    1000.0
}

fn default_execution_allowance_check_interval_secs() -> u64 {
    600
}

fn default_execution_trade_reconcile_interval_secs() -> u64 {
    10
}

fn default_execution_prefer_ws_reconcile() -> bool {
    true
}

fn default_execution_auto_cancel_orders() -> bool {
    true
}

fn default_execution_order_cancel_grace_secs() -> u64 {
    2
}

fn default_execution_cancel_before_replace() -> bool {
    true
}

fn default_execution_ladder_order_type() -> String {
    "GTD".to_string()
}

fn default_execution_snipe_order_type() -> String {
    "FOK".to_string()
}

fn default_execution_post_only_ladder() -> bool {
    true
}

fn default_execution_gtd_min_expiry_secs() -> u64 {
    60
}

fn default_execution_relayer_max_retries() -> u32 {
    2
}

fn default_execution_relayer_backoff_ms() -> u64 {
    50
}

fn default_execution_relayer_backoff_max_ms() -> u64 {
    500
}

fn default_execution_idempotency_ttl_secs() -> u64 {
    30
}

fn default_execution_bankroll_refresh_secs() -> u64 {
    60
}

fn default_execution_no_money_mode() -> bool {
    false
}

fn default_execution_paper_stats_persist() -> bool {
    false
}

fn default_execution_paper_start_bankroll_usdc() -> f64 {
    50.0
}

fn default_execution_paper_latency_ms() -> u64 {
    1_000
}

fn default_execution_paper_slippage_bps() -> f64 {
    5.0
}

fn default_execution_signal_alignment_max_secs() -> u64 {
    120
}

fn default_execution_signal_alignment_max_secs_sol() -> u64 {
    210
}

fn default_execution_take_profit_bps() -> f64 {
    150.0
}

fn default_execution_stop_loss_bps() -> f64 {
    200.0
}

fn default_execution_trailing_stop_bps() -> f64 {
    120.0
}

fn default_execution_close_fraction() -> f64 {
    1.0
}

pub struct ConfigManager {
    path: PathBuf,
    strategies_path: Option<PathBuf>,
    state: Arc<ArcSwap<Config>>,
}

impl ConfigManager {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let strategies_path = path.parent().map(|parent| parent.join("strategies.json"));
        Self::new_with_strategy_override(path, strategies_path)
    }

    pub fn new_with_strategy_override(
        path: PathBuf,
        strategies_path: Option<PathBuf>,
    ) -> Result<Self> {
        let config = load_config_with_strategy_override(&path, strategies_path.as_deref())?;
        Ok(Self {
            path,
            strategies_path,
            state: Arc::new(ArcSwap::from_pointee(config)),
        })
    }

    pub fn current(&self) -> Arc<Config> {
        self.state.load_full()
    }

    pub fn state(&self) -> Arc<ArcSwap<Config>> {
        Arc::clone(&self.state)
    }

    pub fn spawn_watcher(&self) -> Result<tokio::task::JoinHandle<()>> {
        let path = self.path.clone();
        let strategies_path = self.strategies_path.clone();
        let state = Arc::clone(&self.state);

        Ok(tokio::task::spawn_blocking(move || {
            if let Err(error) = watch_loop(&path, strategies_path.as_deref(), state) {
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

fn load_config_with_strategy_override(
    path: &Path,
    strategies_path: Option<&Path>,
) -> Result<Config> {
    let mut config = load_config(path)?;
    if let Some(strategies_path) = strategies_path {
        if strategies_path.exists() {
            let override_config = load_strategy_override(strategies_path)?;
            config.strategy = override_config;
        }
    }
    Ok(config)
}

fn load_strategy_override(path: &Path) -> Result<StrategyConfig> {
    let raw = fs::read_to_string(path)?;
    let stripped = strip_jsdoc_header(&raw)?;
    let trimmed = stripped.trim();
    let parsed: StrategyOverrideFile = serde_json::from_str(trimmed)?;
    let strategy = match parsed {
        StrategyOverrideFile::Wrapped(wrapper) => wrapper.strategy,
        StrategyOverrideFile::Direct(strategy) => strategy,
    };
    validate_strategy_config(&strategy)?;
    Ok(strategy)
}

fn validate_strategy_config(strategy: &StrategyConfig) -> Result<()> {
    if !(0.0..=1.0).contains(&strategy.kelly_fraction) {
        return Err(BankaiError::InvalidArgument(
            "kelly_fraction must be within [0, 1]".to_string(),
        ));
    }
    if strategy.snipe_min_edge_bps < 0.0 {
        return Err(BankaiError::InvalidArgument(
            "snipe_min_edge_bps must be non-negative".to_string(),
        ));
    }
    if strategy.spread_offset_bps < 0.0 {
        return Err(BankaiError::InvalidArgument(
            "spread_offset_bps must be non-negative".to_string(),
        ));
    }
    Ok(())
}

fn watch_loop(
    path: &Path,
    strategies_path: Option<&Path>,
    state: Arc<ArcSwap<Config>>,
) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(path, RecursiveMode::NonRecursive)?;
    if let Some(strategies_path) = strategies_path {
        if strategies_path.exists() {
            watcher.watch(strategies_path, RecursiveMode::NonRecursive)?;
        }
    }

    loop {
        let event = rx.recv()?;
        match event {
            Ok(event) => {
                if !is_relevant_event(&event.kind) {
                    continue;
                }
                match load_config_with_strategy_override(path, strategies_path) {
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
