/**
 * @purpose
 * Terminal UI runtime and state aggregation for system visibility.
 *
 * @dependencies
 * - ratatui: terminal rendering
 * - crossterm: terminal input/output
 *
 * @notes
 * - Runs in a dedicated thread to avoid blocking the trading engine.
 */
use arc_swap::ArcSwap;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::{self, IsTerminal};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;

use crate::accounting::keys::PNL_24H_KEY;
use crate::config::{Config, ExecutionConfig, StrategyConfig};
use crate::engine::analysis::snipe_threshold_bps;
use crate::engine::risk::{HaltReason, RiskState};
use crate::engine::types::{AlloraMarketUpdate, ChainlinkMarketUpdate, MarketUpdate, MarketWindow};
use crate::error::Result;
use crate::storage::orderbook::{BookSide, OrderBookStore};
use crate::storage::redis::RedisManager;
use chrono::{TimeZone, Utc};
use chrono_tz::America::New_York;

mod widgets;

const BANKROLL_KEY: &str = "sys:bankroll:usdc";
const DEFAULT_MARKET_LIMIT: usize = 12;
const DEFAULT_REFRESH_MS: u64 = 250;
const DEFAULT_BANKROLL_REFRESH_MS: u64 = 2_000;
const DEFAULT_POLYMARKET_REFRESH_MS: u64 = 5_000;
const DEFAULT_ACTIVITY_LOG_LIMIT: usize = 8;
const DEFAULT_INTENT_LOG_LIMIT: usize = 6;
const DEFAULT_ORDER_LOG_LIMIT: usize = 6;
const POLYMARKET_STALE_MS: u64 = 120_000;
const ORACLE_ONLINE_MULTIPLIER: u64 = 3;
const SIGNAL_HORIZON_MS: u64 = 5 * 60 * 1_000;
const SQRT_5: f64 = 2.236_067_977_5;
const SIGNAL_DIR_UP: i8 = 1;
const SIGNAL_DIR_DOWN: i8 = -1;
const PAPER_STATS_WINS_KEY: &str = "paper:stats:wins";
const PAPER_STATS_LOSSES_KEY: &str = "paper:stats:losses";
const PAPER_STATS_TOTAL_KEY: &str = "paper:stats:total";
const PAPER_STATS_ACCURACY_KEY: &str = "paper:stats:accuracy_pct";
const PAPER_STATS_MISSED_KEY: &str = "paper:stats:missed";
const PAPER_STATS_MISSED_REASON_KEY: &str = "paper:stats:missed_reason";
const PAPER_BANKROLL_KEY: &str = "paper:bankroll:usdc";
const PAPER_BANKROLL_START_KEY: &str = "paper:bankroll:start_usdc";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExecutionModelVersion {
    V1,
    V2,
}

impl ExecutionModelVersion {
    fn as_str(self) -> &'static str {
        match self {
            ExecutionModelVersion::V1 => "v1",
            ExecutionModelVersion::V2 => "v2",
        }
    }
}

type UiResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct TuiConfig {
    pub refresh_interval: Duration,
    pub bankroll_interval: Duration,
    pub market_limit: usize,
    pub polymarket_interval: Duration,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_millis(DEFAULT_REFRESH_MS),
            bankroll_interval: Duration::from_millis(DEFAULT_BANKROLL_REFRESH_MS),
            market_limit: DEFAULT_MARKET_LIMIT,
            polymarket_interval: Duration::from_millis(DEFAULT_POLYMARKET_REFRESH_MS),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusBarData {
    pub uptime: Duration,
    pub halted: bool,
    pub halt_reason: HaltReason,
    pub chainlink_online: bool,
    pub allora_online: bool,
    pub polymarket_online: bool,
    pub no_money_mode: bool,
    pub model_version: String,
}

#[derive(Debug, Clone)]
pub struct HealthPanelData {
    pub halted: bool,
    pub halt_reason: HaltReason,
    pub latency_ms: u64,
    pub clock_drift_ms: i64,
    pub consecutive_losses: u32,
    pub chainlink_window_anchor: bool,
}

#[derive(Debug, Clone)]
pub struct FinancialPanelData {
    pub bankroll_usdc: Option<f64>,
    pub pnl_24h: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct PaperStatsData {
    pub wins: f64,
    pub losses: f64,
    pub total: f64,
    pub accuracy_pct: f64,
    pub missed: f64,
    pub missed_reason: Option<String>,
    pub bankroll_usdc: f64,
    pub roi_pct: f64,
    pub win_rate_ci_low: f64,
    pub win_rate_ci_high: f64,
}

#[derive(Debug, Clone)]
pub struct PolymarketPanelData {
    pub online: bool,
    pub asset_count: Option<usize>,
    pub last_refresh: Option<Duration>,
}

#[derive(Debug, Clone)]
pub struct ActiveWindowRow {
    pub asset: String,
    pub market_id: String,
    pub window_et: String,
    pub status: String,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
}

#[derive(Debug, Clone)]
pub enum MarketMode {
    NoSignal,
    Wait,
    Hold,
    Ladder,
    Snipe,
}

impl MarketMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NoSignal => "NO_SIGNAL",
            Self::Wait => "WAIT",
            Self::Hold => "HOLD",
            Self::Ladder => "LADDER",
            Self::Snipe => "SNIPE",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarketRow {
    pub asset: String,
    pub price: Option<f64>,
    pub implied_up: Option<f64>,
    pub min_order_size: Option<f64>,
    pub start_price: Option<f64>,
    pub inference_5m: Option<f64>,
    pub edge_bps: Option<f64>,
    pub side: Option<String>,
    pub fee_bps: Option<f64>,
    pub mode: MarketMode,
    pub last_update_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct UiSnapshot {
    pub status: StatusBarData,
    pub health: HealthPanelData,
    pub financials: FinancialPanelData,
    pub polymarket: PolymarketPanelData,
    pub paper_stats: Option<PaperStatsData>,
    pub no_money_mode: bool,
    pub markets: Vec<MarketRow>,
    pub activity_log: Vec<String>,
    pub intent_log: Vec<String>,
    pub order_log: Vec<String>,
    pub execution_status: Option<String>,
    pub orders_ok: usize,
    pub orders_fail: usize,
    pub open_orders: Option<usize>,
    pub last_order_state: Option<String>,
    pub active_windows: Vec<ActiveWindowRow>,
}

#[derive(Debug)]
pub struct TuiHandle {
    shutdown: mpsc::Sender<UiCommand>,
}

impl TuiHandle {
    pub fn shutdown(&self) {
        let _ = self.shutdown.send(UiCommand::Shutdown);
    }
}

#[derive(Debug)]
enum UiCommand {
    Snapshot(UiSnapshot),
    Shutdown,
}

#[derive(Debug, Clone)]
struct MarketSnapshot {
    asset: String,
    price: Option<f64>,
    implied_up: Option<f64>,
    implied_down: Option<f64>,
    implied_up_mid: Option<f64>,
    implied_down_mid: Option<f64>,
    implied_up_vwap: Option<f64>,
    implied_down_vwap: Option<f64>,
    last_trade_up: Option<f64>,
    last_trade_down: Option<f64>,
    up_token_id: Option<String>,
    down_token_id: Option<String>,
    market_id: Option<String>,
    window: Option<MarketWindow>,
    start_price: Option<f64>,
    inference_5m: Option<f64>,
    signal_timestamp_ms: Option<u64>,
    last_signature: Option<String>,
    last_request_id: Option<String>,
    volatility_1m: Option<f64>,
    fee_rate_up_bps: Option<f64>,
    fee_rate_down_bps: Option<f64>,
    min_order_size: Option<f64>,
    last_chainlink_ms: Option<u64>,
    last_allora_ms: Option<u64>,
}

impl MarketSnapshot {
    fn new(asset: String) -> Self {
        Self {
            asset,
            price: None,
            implied_up: None,
            implied_down: None,
            implied_up_mid: None,
            implied_down_mid: None,
            implied_up_vwap: None,
            implied_down_vwap: None,
            last_trade_up: None,
            last_trade_down: None,
            up_token_id: None,
            down_token_id: None,
            market_id: None,
            window: None,
            start_price: None,
            inference_5m: None,
            signal_timestamp_ms: None,
            last_signature: None,
            last_request_id: None,
            volatility_1m: None,
            fee_rate_up_bps: None,
            fee_rate_down_bps: None,
            min_order_size: None,
            last_chainlink_ms: None,
            last_allora_ms: None,
        }
    }

    fn apply_chainlink(&mut self, update: &ChainlinkMarketUpdate) {
        self.price = resolve_price(update);
        self.volatility_1m = update.volatility_1m;
        let event_time = if update.event_time_ms > 0 {
            update.event_time_ms
        } else {
            now_ms().unwrap_or(0)
        };
        self.last_chainlink_ms = Some(event_time);
    }

    fn apply_allora(&mut self, update: &AlloraMarketUpdate, horizon_ms: u64) {
        match update.timeframe.to_ascii_lowercase().as_str() {
            "5m" => {
                let same_value = self
                    .inference_5m
                    .map(|value| value == update.inference_value)
                    .unwrap_or(false);
                self.inference_5m = Some(update.inference_value);
                let prev_ts = self.signal_timestamp_ms.unwrap_or(0);
                if same_value && prev_ts > 0 {
                    let diff = update.signal_timestamp_ms.saturating_sub(prev_ts);
                    if diff >= horizon_ms {
                        self.signal_timestamp_ms = Some(update.signal_timestamp_ms);
                    }
                } else {
                    self.signal_timestamp_ms = Some(update.signal_timestamp_ms);
                }
                if update.signature.is_some() {
                    self.last_signature = update.signature.clone();
                }
                if update.request_id.is_some() {
                    self.last_request_id = update.request_id.clone();
                }
            }
            _ => {}
        }
        let received_at = if update.received_at_ms > 0 {
            update.received_at_ms
        } else if update.signal_timestamp_ms > 0 {
            update.signal_timestamp_ms
        } else {
            now_ms().unwrap_or(0)
        };
        self.last_allora_ms = Some(received_at);
    }
}

pub fn is_tui_enabled() -> bool {
    if let Ok(value) = std::env::var("BANKAI_TUI") {
        if let Some(parsed) = parse_env_bool(&value) {
            return parsed;
        }
    }
    io::stdout().is_terminal()
}

pub fn spawn_tui(
    config: Arc<ArcSwap<Config>>,
    risk: Arc<RiskState>,
    receiver: broadcast::Receiver<MarketUpdate>,
    redis: Option<RedisManager>,
    wallet_key: Option<String>,
) -> Result<TuiHandle> {
    let (tx, rx) = mpsc::channel();
    let ui_config = TuiConfig::default();
    let ui_thread_config = ui_config.clone();

    let _thread_handle = thread::spawn(move || {
        if let Err(error) = ui_loop(rx, ui_thread_config) {
            tracing::error!(?error, "tui loop stopped");
        }
    });

    let snapshot_sender = tx.clone();
    tokio::spawn(async move {
        if let Err(error) = snapshot_loop(
            config,
            risk,
            receiver,
            redis,
            wallet_key,
            snapshot_sender,
            ui_config,
        )
        .await
        {
            tracing::error!(?error, "tui snapshot loop stopped");
        }
    });

    Ok(TuiHandle { shutdown: tx })
}

async fn snapshot_loop(
    config: Arc<ArcSwap<Config>>,
    risk: Arc<RiskState>,
    mut receiver: broadcast::Receiver<MarketUpdate>,
    redis: Option<RedisManager>,
    wallet_key: Option<String>,
    sender: mpsc::Sender<UiCommand>,
    ui_config: TuiConfig,
) -> Result<()> {
    let mut market_state: HashMap<String, MarketSnapshot> = HashMap::new();
    let mut refresh = tokio::time::interval(ui_config.refresh_interval);
    let mut bankroll_interval = tokio::time::interval(ui_config.bankroll_interval);
    let mut polymarket_interval = tokio::time::interval(ui_config.polymarket_interval);
    let start_time = Instant::now();
    let mut bankroll_usdc: Option<f64> = None;
    let mut pnl_24h: Option<f64> = None;
    let mut polymarket_asset_count: Option<usize> = None;
    let mut polymarket_last_refresh: Option<Instant> = None;
    let mut activity_log: Vec<String> = Vec::new();
    let mut intent_log: Vec<String> = Vec::new();
    let mut order_log: Vec<String> = Vec::new();
    let mut open_orders: Option<usize> = None;
    let mut last_order_state: Option<String> = None;
    let mut chainlink_window_anchor = false;
    let mut active_windows: Vec<ActiveWindowRow> = Vec::new();
    let mut paper_stats: Option<PaperStatsData> = None;
    let orderbook = redis
        .as_ref()
        .map(|manager| OrderBookStore::new(manager.clone()));

    loop {
        tokio::select! {
            message = receiver.recv() => {
                match message {
                    Ok(update) => match update {
                        MarketUpdate::Chainlink(update) => {
                            let key = canonical_asset(&update.asset);
                            let entry = market_state
                                .entry(key.clone())
                                .or_insert_with(|| MarketSnapshot::new(key.clone()));
                            entry.apply_chainlink(&update);
                        }
                        MarketUpdate::Allora(update) => {
                            let key = canonical_asset(&update.asset);
                            let entry = market_state
                                .entry(key.clone())
                                .or_insert_with(|| MarketSnapshot::new(key.clone()));
                            let config_snapshot = config.load();
                            let horizon_ms = alignment_horizon_ms(&config_snapshot, &key);
                            entry.apply_allora(&update, horizon_ms);
                        }
                    },
                    Err(broadcast::error::RecvError::Closed) => break,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                }
            }
            _ = bankroll_interval.tick() => {
                if let Some(redis) = redis.as_ref() {
                    match redis.get_float(BANKROLL_KEY).await {
                        Ok(value) => bankroll_usdc = value,
                        Err(error) => tracing::warn!(?error, "failed to read bankroll from redis"),
                    }
                    match redis.get_float(PNL_24H_KEY).await {
                        Ok(value) => pnl_24h = value,
                        Err(error) => tracing::warn!(?error, "failed to read pnl from redis"),
                    }
                }
            }
            _ = polymarket_interval.tick() => {
                if let Some(redis) = redis.as_ref() {
                    match redis.get_polymarket_asset_ids().await {
                        Ok(ids) => {
                            polymarket_asset_count = Some(ids.len());
                            polymarket_last_refresh = Some(Instant::now());
                        }
                        Err(error) => tracing::warn!(?error, "failed to read polymarket asset ids from redis"),
                    }
                    chainlink_window_anchor = redis.get_asset_window("BTC").await?.is_some()
                        || redis.get_asset_window("ETH").await?.is_some()
                        || redis.get_asset_window("SOL").await?.is_some();
                    match load_active_windows(redis).await {
                        Ok(rows) => active_windows = rows,
                        Err(error) => tracing::warn!(?error, "failed to read active windows from redis"),
                    }
                    match redis.get_activity_log(DEFAULT_ACTIVITY_LOG_LIMIT).await {
                        Ok(entries) => activity_log = entries,
                        Err(error) => tracing::warn!(?error, "failed to read activity log from redis"),
                    }
                    match redis.get_intent_log(DEFAULT_INTENT_LOG_LIMIT).await {
                        Ok(entries) => intent_log = entries,
                        Err(error) => tracing::warn!(?error, "failed to read intent log from redis"),
                    }
                    match redis.get_order_log(DEFAULT_ORDER_LOG_LIMIT).await {
                        Ok(entries) => order_log = entries,
                        Err(error) => tracing::warn!(?error, "failed to read order log from redis"),
                    }
                    if config.load().execution.no_money_mode {
                        let wins = redis
                            .get_float(PAPER_STATS_WINS_KEY)
                            .await
                            .unwrap_or(None)
                            .unwrap_or(0.0);
                        let losses = redis
                            .get_float(PAPER_STATS_LOSSES_KEY)
                            .await
                            .unwrap_or(None)
                            .unwrap_or(0.0);
                        let total = redis
                            .get_float(PAPER_STATS_TOTAL_KEY)
                            .await
                            .unwrap_or(None)
                            .unwrap_or(0.0);
                        let accuracy = redis
                            .get_float(PAPER_STATS_ACCURACY_KEY)
                            .await
                            .unwrap_or(None)
                            .unwrap_or(0.0);
                        let missed = redis
                            .get_float(PAPER_STATS_MISSED_KEY)
                            .await
                            .unwrap_or(None)
                            .unwrap_or(0.0);
                        let missed_reason = redis
                            .get_string(PAPER_STATS_MISSED_REASON_KEY)
                            .await
                            .unwrap_or(None);
                        let bankroll = redis
                            .get_float(PAPER_BANKROLL_KEY)
                            .await
                            .unwrap_or(None)
                            .unwrap_or(0.0);
                        let start = redis
                            .get_float(PAPER_BANKROLL_START_KEY)
                            .await
                            .unwrap_or(None)
                            .unwrap_or(0.0);
                        let roi_pct = if start > 0.0 {
                            ((bankroll - start) / start) * 100.0
                        } else {
                            0.0
                        };
                        let (ci_low, ci_high) = win_rate_confidence_interval(wins, losses);
                        paper_stats = Some(PaperStatsData {
                            wins,
                            losses,
                            total,
                            accuracy_pct: accuracy,
                            missed,
                            missed_reason,
                            bankroll_usdc: bankroll,
                            roi_pct,
                            win_rate_ci_low: ci_low,
                            win_rate_ci_high: ci_high,
                        });
                    } else {
                        paper_stats = None;
                    }
                    if let Some(wallet_key) = wallet_key.as_ref() {
                        let key = open_orders_key(wallet_key);
                        match redis.scard(&key).await {
                            Ok(count) => open_orders = Some(count),
                            Err(error) => tracing::warn!(?error, "failed to read open orders"),
                        }
                        match redis.get_last_order_state(wallet_key).await {
                            Ok(value) => last_order_state = value,
                            Err(error) => tracing::warn!(?error, "failed to read last order state"),
                        }
                    }
                    if let Some(orderbook) = orderbook.as_ref() {
                        if let Err(error) =
                            refresh_market_snapshots(redis, orderbook, &config, bankroll_usdc, &mut market_state).await
                        {
                            tracing::warn!(?error, "failed to refresh market snapshots");
                        }
                    }
                }
            }
            _ = refresh.tick() => {
                if let (Some(redis), Some(orderbook)) = (redis.as_ref(), orderbook.as_ref()) {
                    if let Err(error) =
                        refresh_market_snapshots(redis, orderbook, &config, bankroll_usdc, &mut market_state).await
                    {
                        tracing::warn!(?error, "failed to refresh market snapshots");
                    }
                }
                let snapshot = build_snapshot(
                    &config,
                    &risk,
                    &market_state,
                    start_time,
                    bankroll_usdc,
                    pnl_24h,
                    polymarket_asset_count,
                    polymarket_last_refresh,
                    ui_config.market_limit,
                    activity_log.clone(),
                    intent_log.clone(),
                    order_log.clone(),
                    open_orders,
                    last_order_state.clone(),
                    chainlink_window_anchor,
                    active_windows.clone(),
                    paper_stats.clone(),
                );
                if sender.send(UiCommand::Snapshot(snapshot)).is_err() {
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn refresh_market_snapshots(
    redis: &RedisManager,
    orderbook: &OrderBookStore,
    config: &Arc<ArcSwap<Config>>,
    bankroll_usdc: Option<f64>,
    market_state: &mut HashMap<String, MarketSnapshot>,
) -> Result<()> {
    let config = config.load_full();
    let now_ms = now_ms().unwrap_or(0);
    for snapshot in market_state.values_mut() {
        let Some(window) = redis.get_asset_window(&snapshot.asset).await? else {
            snapshot.market_id = None;
            snapshot.window = None;
            snapshot.implied_up = None;
            snapshot.implied_down = None;
            snapshot.implied_up_mid = None;
            snapshot.implied_down_mid = None;
            snapshot.implied_up_vwap = None;
            snapshot.implied_down_vwap = None;
            snapshot.last_trade_up = None;
            snapshot.last_trade_down = None;
            snapshot.up_token_id = None;
            snapshot.down_token_id = None;
            snapshot.start_price = None;
            snapshot.fee_rate_up_bps = None;
            snapshot.fee_rate_down_bps = None;
            snapshot.min_order_size = None;
            continue;
        };
        snapshot.market_id = Some(window.market_id.clone());
        snapshot.window = Some(MarketWindow {
            start_time_ms: window.start_time_ms,
            end_time_ms: window.end_time_ms,
        });
        snapshot.start_price = match redis.get_asset_start_price(&snapshot.asset).await? {
            Some((start_ms, price)) if start_ms == window.start_time_ms => Some(price),
            _ => None,
        };
        let metadata = redis.get_market_metadata(&window.market_id).await?;
        snapshot.min_order_size = metadata.min_order_size;
        if let Some(up_token) = metadata.outcome_up_token_id {
            snapshot.up_token_id = Some(up_token.clone());
            let mid = orderbook.mid_price(&up_token).await?;
            snapshot.implied_up_mid = mid;
            snapshot.last_trade_up = orderbook.last_trade_price(&up_token).await?;
            let best = orderbook.best_bid_ask(&up_token).await?;
            snapshot.implied_up = resolve_display_price(mid, best, snapshot.last_trade_up);
            snapshot.fee_rate_up_bps = redis.get_fee_rate_bps(&up_token).await?;
            snapshot.implied_up_vwap = estimate_vwap_price(
                orderbook,
                &up_token,
                snapshot,
                snapshot.implied_up_mid,
                &config,
                bankroll_usdc,
                now_ms,
            )
            .await?;
        }
        if let Some(down_token) = metadata.outcome_down_token_id {
            snapshot.down_token_id = Some(down_token.clone());
            let mid = orderbook.mid_price(&down_token).await?;
            snapshot.implied_down_mid = mid;
            snapshot.last_trade_down = orderbook.last_trade_price(&down_token).await?;
            let best = orderbook.best_bid_ask(&down_token).await?;
            snapshot.implied_down = resolve_display_price(mid, best, snapshot.last_trade_down);
            snapshot.fee_rate_down_bps = redis.get_fee_rate_bps(&down_token).await?;
            snapshot.implied_down_vwap = estimate_vwap_price(
                orderbook,
                &down_token,
                snapshot,
                snapshot.implied_down_mid,
                &config,
                bankroll_usdc,
                now_ms,
            )
            .await?;
        }
        if let Ok(Some(price)) = redis.get_chainlink_price(&snapshot.asset).await {
            snapshot.price = Some(price);
            if let Ok(updated_ms) = redis.get_chainlink_updated_ms().await {
                snapshot.last_chainlink_ms = updated_ms;
            }
        }
    }
    Ok(())
}

fn resolve_display_price(
    mid: Option<f64>,
    best: Option<(f64, f64)>,
    last_trade: Option<f64>,
) -> Option<f64> {
    let mid = mid?;
    if let Some((bid, ask)) = best {
        let spread = (ask - bid).abs();
        if spread > 0.10 {
            if let Some(last) = last_trade {
                if last > 0.0 {
                    return Some(last);
                }
            }
        }
    }
    Some(mid)
}

async fn estimate_vwap_price(
    orderbook: &OrderBookStore,
    token_id: &str,
    snapshot: &MarketSnapshot,
    implied_mid: Option<f64>,
    config: &Config,
    bankroll_usdc: Option<f64>,
    now_ms: u64,
) -> Result<Option<f64>> {
    let Some(implied_mid) = implied_mid else {
        return Ok(None);
    };
    let horizon_ms = alignment_horizon_ms(config, &snapshot.asset);
    let max_align_ms = horizon_ms;
    let alignment = alignment_factor(snapshot, now_ms, horizon_ms, max_align_ms);
    let volatility_1m = snapshot
        .volatility_1m
        .unwrap_or(config.execution.min_volatility)
        .max(config.execution.min_volatility);
    let true_up = match (snapshot.start_price, snapshot.inference_5m, alignment) {
        (Some(start_price), Some(predicted), Some(alignment)) => Some(compute_true_probability_5m(
            start_price,
            predicted,
            volatility_1m,
            config.execution.probability_scale,
            config.execution.probability_max_offset,
            alignment,
        )),
        _ => None,
    };
    let Some(true_up) = true_up else {
        return Ok(None);
    };
    let Some(size) = estimate_order_size(
        bankroll_usdc,
        true_up,
        implied_mid,
        &config.execution,
        &config.strategy,
    ) else {
        return Ok(None);
    };
    let vwap = orderbook
        .vwap_for_size(token_id, BookSide::Ask, size, 50)
        .await?;
    Ok(vwap.map(|value| value.avg_price))
}

fn estimate_order_size(
    bankroll_usdc: Option<f64>,
    true_prob: f64,
    price: f64,
    execution: &ExecutionConfig,
    strategy: &StrategyConfig,
) -> Option<f64> {
    if price <= 0.0 {
        return None;
    }
    let odds = 1.0 / price;
    let kelly = calculate_kelly(true_prob, odds);
    let target = bankroll_usdc
        .map(|bankroll| bankroll * strategy.kelly_fraction * kelly)
        .unwrap_or(execution.default_order_usdc);
    let notional = target.clamp(execution.min_order_usdc, execution.max_order_usdc);
    if notional < execution.min_order_usdc {
        return None;
    }
    Some(notional / price)
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

fn build_snapshot(
    config: &Arc<ArcSwap<Config>>,
    risk: &Arc<RiskState>,
    market_state: &HashMap<String, MarketSnapshot>,
    start_time: Instant,
    bankroll_usdc: Option<f64>,
    pnl_24h: Option<f64>,
    polymarket_asset_count: Option<usize>,
    polymarket_last_refresh: Option<Instant>,
    market_limit: usize,
    activity_log: Vec<String>,
    intent_log: Vec<String>,
    order_log: Vec<String>,
    open_orders: Option<usize>,
    last_order_state: Option<String>,
    chainlink_window_anchor: bool,
    active_windows: Vec<ActiveWindowRow>,
    paper_stats: Option<PaperStatsData>,
) -> UiSnapshot {
    let config = config.load_full();
    let risk_snapshot = risk.snapshot();
    let now_ms = now_ms().unwrap_or(0);

    let status = StatusBarData {
        uptime: start_time.elapsed(),
        halted: risk_snapshot.halted,
        halt_reason: risk_snapshot.reason,
        chainlink_online: is_oracle_online(
            now_ms,
            last_chainlink_update_ms(market_state),
            chainlink_online_window(&config),
        ),
        allora_online: is_oracle_online(
            now_ms,
            last_allora_update_ms(market_state),
            allora_online_window(&config),
        ),
        polymarket_online: is_polymarket_online(polymarket_last_refresh),
        no_money_mode: config.execution.no_money_mode,
        model_version: resolve_model_version(&config.execution)
            .as_str()
            .to_ascii_uppercase(),
    };

    let health = HealthPanelData {
        halted: risk_snapshot.halted,
        halt_reason: risk_snapshot.reason,
        latency_ms: risk_snapshot.last_latency_ms,
        clock_drift_ms: risk_snapshot.clock_drift_ms,
        consecutive_losses: risk_snapshot.consecutive_losses,
        chainlink_window_anchor,
    };

    let financials = FinancialPanelData {
        bankroll_usdc,
        pnl_24h,
    };

    let polymarket_panel = PolymarketPanelData {
        online: status.polymarket_online,
        asset_count: polymarket_asset_count,
        last_refresh: polymarket_last_refresh.map(|ts| ts.elapsed()),
    };

    let snipe_floor = snipe_threshold_bps(&config.strategy, &config.fees);
    let mut markets: Vec<MarketRow> = market_state
        .values()
        .map(|snapshot| build_market_row(snapshot, snipe_floor, &config, now_ms))
        .collect();
    markets.sort_by(|a, b| {
        let left = a.edge_bps.unwrap_or(f64::NEG_INFINITY);
        let right = b.edge_bps.unwrap_or(f64::NEG_INFINITY);
        right
            .partial_cmp(&left)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    if markets.len() > market_limit {
        markets.truncate(market_limit);
    }

    let exec_status = execution_status(&intent_log, &order_log);
    let orders_ok = count_orders(&order_log, "[ORDER] OK");
    let orders_fail = count_orders(&order_log, "[ORDER] FAIL");

    UiSnapshot {
        status,
        health,
        financials,
        polymarket: polymarket_panel,
        paper_stats,
        no_money_mode: config.execution.no_money_mode,
        markets,
        activity_log,
        intent_log,
        order_log,
        execution_status: exec_status,
        orders_ok,
        orders_fail,
        open_orders,
        last_order_state,
        active_windows,
    }
}

fn build_market_row(
    snapshot: &MarketSnapshot,
    snipe_floor_bps: f64,
    config: &Config,
    now_ms: u64,
) -> MarketRow {
    let last_update_ms = snapshot.last_chainlink_ms.max(snapshot.last_allora_ms);

    let implied_up = snapshot.implied_up;
    let _implied_down = snapshot
        .implied_down
        .or_else(|| implied_up.map(|v| (1.0 - v).max(0.0)));
    let implied_up_mid = snapshot.implied_up_mid;
    let implied_down_mid = snapshot
        .implied_down_mid
        .or_else(|| implied_up_mid.map(|v| (1.0 - v).max(0.0)));
    let implied_up_vwap = snapshot.implied_up_vwap;
    let implied_down_vwap = snapshot
        .implied_down_vwap
        .or_else(|| implied_up_vwap.map(|v| (1.0 - v).max(0.0)));

    let horizon_ms = alignment_horizon_ms(config, &snapshot.asset);
    let max_align_ms = horizon_ms;
    let alignment = alignment_factor(snapshot, now_ms, horizon_ms, max_align_ms);
    let volatility_1m = snapshot
        .volatility_1m
        .unwrap_or(config.execution.min_volatility)
        .max(config.execution.min_volatility);
    let model_version = resolve_model_version(&config.execution);
    let model_output = compute_model_output(
        model_version,
        implied_up,
        snapshot.start_price,
        snapshot.price,
        snapshot.inference_5m,
        volatility_1m,
        &config.execution,
        alignment,
    );
    let signal_context = model_output.and_then(|value| value.signal_context);

    // The trading engine intentionally does not act until the target timestamp
    // (`window.end - horizon`). Showing an "EV" before that point is misleading because the
    // same signal is down-weighted by alignment and will almost always look like HOLD.
    let pre_target = snapshot
        .window
        .map(|window| now_ms < window.end_time_ms.saturating_sub(horizon_ms))
        .unwrap_or(false);

    if pre_target {
        let side = signal_context
            .as_ref()
            .and_then(|signal| match signal.direction {
                SIGNAL_DIR_UP => Some("UP".to_string()),
                SIGNAL_DIR_DOWN => Some("DOWN".to_string()),
                _ => None,
            });
        let has_side = side.is_some();

        return MarketRow {
            asset: snapshot.asset.clone(),
            price: snapshot.price,
            implied_up,
            min_order_size: snapshot.min_order_size,
            start_price: snapshot.start_price,
            inference_5m: snapshot.inference_5m,
            edge_bps: None,
            side,
            fee_bps: snapshot.fee_rate_up_bps,
            mode: if has_side {
                MarketMode::Wait
            } else {
                MarketMode::NoSignal
            },
            last_update_ms,
        };
    }

    let mut true_up = model_output.map(|value| value.true_up);
    if model_version == ExecutionModelVersion::V2
        && config.execution.model_v2_z_min > 0.0
        && model_output
            .map(|value| value.z_score.abs() < config.execution.model_v2_z_min)
            .unwrap_or(false)
    {
        true_up = None;
    }
    let true_down = true_up.map(|value| (1.0 - value).clamp(0.0, 1.0));

    let decision_up = decide_edge(true_up, implied_up_mid, implied_up_vwap, snipe_floor_bps)
        .and_then(|edge| {
            if model_version == ExecutionModelVersion::V2
                && edge.edge_bps < config.execution.model_v2_edge_floor_bps
            {
                return None;
            }
            if signal_allows_direction(
                &config.execution,
                signal_context.as_ref(),
                SIGNAL_DIR_UP,
                edge.edge_bps,
            ) {
                Some(edge)
            } else {
                None
            }
        });
    let decision_down = decide_edge(
        true_down,
        implied_down_mid,
        implied_down_vwap,
        snipe_floor_bps,
    )
    .and_then(|edge| {
        if model_version == ExecutionModelVersion::V2
            && edge.edge_bps < config.execution.model_v2_edge_floor_bps
        {
            return None;
        }
        if signal_allows_direction(
            &config.execution,
            signal_context.as_ref(),
            SIGNAL_DIR_DOWN,
            edge.edge_bps,
        ) {
            Some(edge)
        } else {
            None
        }
    });

    let (edge_bps, side, mode) = match (decision_up.clone(), decision_down.clone()) {
        (None, None) => (None, None, MarketMode::NoSignal),
        _ => {
            let mut best = decision_up.map(|edge| ("UP", edge));
            if let Some(edge) = decision_down {
                let replace = match best {
                    Some((_, ref current)) => edge.edge_bps > current.edge_bps,
                    None => true,
                };
                if replace {
                    best = Some(("DOWN", edge));
                }
            }
            match best {
                Some((label, edge)) => (
                    Some(edge.edge_bps),
                    Some(label.to_string()),
                    edge.mode.clone(),
                ),
                None => (None, None, MarketMode::NoSignal),
            }
        }
    };

    let fee_bps = snapshot.fee_rate_up_bps;

    MarketRow {
        asset: snapshot.asset.clone(),
        price: snapshot.price,
        implied_up,
        min_order_size: snapshot.min_order_size,
        start_price: snapshot.start_price,
        inference_5m: snapshot.inference_5m,
        edge_bps,
        side,
        fee_bps,
        mode,
        last_update_ms,
    }
}

#[derive(Clone)]
struct EdgeDecision {
    edge_bps: f64,
    mode: MarketMode,
}

fn decide_edge(
    true_prob: Option<f64>,
    implied_mid: Option<f64>,
    implied_vwap: Option<f64>,
    snipe_floor_bps: f64,
) -> Option<EdgeDecision> {
    let true_prob = true_prob?;
    let implied_mid = implied_mid?;
    let edge_mid = (true_prob - implied_mid) * 10_000.0;
    if edge_mid <= 0.0 {
        return Some(EdgeDecision {
            edge_bps: edge_mid,
            mode: MarketMode::Hold,
        });
    }
    if edge_mid >= snipe_floor_bps {
        if let Some(vwap) = implied_vwap {
            let edge_vwap = (true_prob - vwap) * 10_000.0;
            if edge_vwap >= snipe_floor_bps {
                return Some(EdgeDecision {
                    edge_bps: edge_vwap,
                    mode: MarketMode::Snipe,
                });
            }
        }
    }
    Some(EdgeDecision {
        edge_bps: edge_mid,
        mode: MarketMode::Ladder,
    })
}

fn execution_status(intent_log: &[String], order_log: &[String]) -> Option<String> {
    if order_log.iter().any(|entry| entry.contains("[ORDER]")) {
        return Some("ORDER_SENT".to_string());
    }
    if intent_log.iter().any(|entry| entry.contains("[INTENT]")) {
        return Some("SIGNAL_ONLY".to_string());
    }
    None
}

fn count_orders(entries: &[String], needle: &str) -> usize {
    entries
        .iter()
        .filter(|entry| entry.contains(needle))
        .count()
}

fn open_orders_key(wallet_key: &str) -> String {
    format!("orders:open:{wallet_key}")
}

fn last_chainlink_update_ms(state: &HashMap<String, MarketSnapshot>) -> Option<u64> {
    state
        .values()
        .filter_map(|snapshot| snapshot.last_chainlink_ms)
        .max()
}

fn last_allora_update_ms(state: &HashMap<String, MarketSnapshot>) -> Option<u64> {
    state
        .values()
        .filter_map(|snapshot| snapshot.last_allora_ms)
        .max()
}

fn is_oracle_online(now_ms: u64, last_update_ms: Option<u64>, window: Duration) -> bool {
    let last_update_ms = match last_update_ms {
        Some(value) => value,
        None => return false,
    };
    let window_ms = window.as_millis() as u64;
    now_ms.saturating_sub(last_update_ms) <= window_ms
}

fn allora_online_window(config: &Config) -> Duration {
    if let Some(allora) = config.allora_consumer.as_ref() {
        let window = allora
            .poll_interval_secs
            .saturating_mul(ORACLE_ONLINE_MULTIPLIER);
        Duration::from_secs(window.max(5))
    } else {
        Duration::from_secs(5)
    }
}

fn chainlink_online_window(_config: &Config) -> Duration {
    Duration::from_secs(10)
}

fn win_rate_confidence_interval(wins: f64, losses: f64) -> (f64, f64) {
    let n = wins + losses;
    if n <= 0.0 {
        return (0.0, 0.0);
    }
    let p = wins / n;
    let z = 1.96;
    let z2 = z * z;
    let denom = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denom;
    let margin = (z / denom) * ((p * (1.0 - p) / n + z2 / (4.0 * n * n)).max(0.0)).sqrt();
    let low = (center - margin).clamp(0.0, 1.0);
    let high = (center + margin).clamp(0.0, 1.0);
    (low * 100.0, high * 100.0)
}

fn ui_loop(receiver: mpsc::Receiver<UiCommand>, config: TuiConfig) -> UiResult<()> {
    let mut stdout = io::stdout();
    let _guard = TerminalGuard::enter(&mut stdout)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut snapshot = UiSnapshot {
        status: StatusBarData {
            uptime: Duration::from_secs(0),
            halted: false,
            halt_reason: HaltReason::None,
            chainlink_online: false,
            allora_online: false,
            polymarket_online: false,
            no_money_mode: false,
            model_version: "V1".to_string(),
        },
        health: HealthPanelData {
            halted: false,
            halt_reason: HaltReason::None,
            latency_ms: 0,
            clock_drift_ms: 0,
            consecutive_losses: 0,
            chainlink_window_anchor: false,
        },
        financials: FinancialPanelData {
            bankroll_usdc: None,
            pnl_24h: None,
        },
        polymarket: PolymarketPanelData {
            online: false,
            asset_count: None,
            last_refresh: None,
        },
        paper_stats: None,
        no_money_mode: false,
        markets: Vec::new(),
        activity_log: Vec::new(),
        intent_log: Vec::new(),
        order_log: Vec::new(),
        execution_status: None,
        orders_ok: 0,
        orders_fail: 0,
        open_orders: None,
        last_order_state: None,
        active_windows: Vec::new(),
    };

    loop {
        while let Ok(command) = receiver.try_recv() {
            match command {
                UiCommand::Snapshot(next) => snapshot = next,
                UiCommand::Shutdown => return Ok(()),
            }
        }

        terminal.draw(|frame| {
            widgets::render_dashboard(frame, &snapshot);
        })?;

        if event::poll(config.refresh_interval)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter(stdout: &mut io::Stdout) -> UiResult<Self> {
        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

fn parse_env_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn canonical_asset(raw: &str) -> String {
    let upper = raw.trim().to_ascii_uppercase();
    if let Some((base, _)) = upper.split_once('/') {
        return base.trim().to_string();
    }
    upper
        .strip_suffix("USDT")
        .or_else(|| upper.strip_suffix("USD"))
        .map(|value| value.to_string())
        .unwrap_or(upper)
}

fn resolve_price(update: &ChainlinkMarketUpdate) -> Option<f64> {
    match update.last_price {
        Some(value) if value > 0.0 => Some(value),
        _ => None,
    }
}

fn now_ms() -> Option<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    Some(now.as_millis() as u64)
}

fn alignment_factor(
    snapshot: &MarketSnapshot,
    now_ms: u64,
    horizon_ms: u64,
    max_alignment_ms: u64,
) -> Option<f64> {
    let signal_ts = snapshot.signal_timestamp_ms?;
    let window = snapshot.window?;
    if signal_ts > window.end_time_ms {
        return None;
    }
    if signal_ts < window.start_time_ms {
        let age = window.start_time_ms.saturating_sub(signal_ts);
        if age > horizon_ms {
            return None;
        }
    }
    if now_ms.saturating_sub(signal_ts) > horizon_ms {
        return None;
    }
    let target_ts = window.end_time_ms.saturating_sub(horizon_ms);
    let diff = signal_ts.abs_diff(target_ts);
    let denom = if max_alignment_ms > 0 {
        max_alignment_ms
    } else {
        horizon_ms
    }
    .max(1) as f64;
    let alignment = 1.0 - (diff as f64 / denom);
    Some(alignment.clamp(0.0, 1.0))
}

fn alignment_horizon_ms(config: &Config, asset: &str) -> u64 {
    let base_ms = config
        .execution
        .signal_alignment_max_secs
        .saturating_mul(1000);
    let sol_ms = config
        .execution
        .signal_alignment_max_secs_sol
        .saturating_mul(1000);
    let selected = if asset.eq_ignore_ascii_case("SOL") && sol_ms > 0 {
        sol_ms
    } else {
        base_ms
    };
    if selected > 0 {
        selected
    } else {
        SIGNAL_HORIZON_MS
    }
}

#[derive(Debug, Clone, Copy)]
struct SignalContext {
    direction: i8,
    confidence: f64,
}

#[derive(Debug, Clone, Copy)]
struct ModelOutput {
    true_up: f64,
    signal_context: Option<SignalContext>,
    z_score: f64,
}

fn resolve_model_version(execution: &ExecutionConfig) -> ExecutionModelVersion {
    match execution.model_version.trim().to_ascii_lowercase().as_str() {
        "v2" => ExecutionModelVersion::V2,
        _ => ExecutionModelVersion::V1,
    }
}

#[allow(clippy::too_many_arguments)]
fn compute_model_output(
    model: ExecutionModelVersion,
    market_up_prob: Option<f64>,
    start_price: Option<f64>,
    current_price: Option<f64>,
    predicted_price: Option<f64>,
    volatility_1m: f64,
    execution: &ExecutionConfig,
    alignment: Option<f64>,
) -> Option<ModelOutput> {
    let predicted_price = predicted_price?;
    let alignment = alignment?;
    match model {
        ExecutionModelVersion::V1 => {
            let start_price = start_price?;
            let z = compute_signal_z_score(
                start_price,
                predicted_price,
                volatility_1m,
                execution.probability_scale,
            )
            .unwrap_or(0.0);
            Some(ModelOutput {
                true_up: compute_true_probability_5m(
                    start_price,
                    predicted_price,
                    volatility_1m,
                    execution.probability_scale,
                    execution.probability_max_offset,
                    alignment,
                ),
                signal_context: compute_signal_context(
                    start_price,
                    predicted_price,
                    volatility_1m,
                    execution.probability_scale,
                    alignment,
                ),
                z_score: z,
            })
        }
        ExecutionModelVersion::V2 => {
            let current_price = current_price?;
            let market_up_prob = market_up_prob?;
            let z = compute_signal_z_score(
                current_price,
                predicted_price,
                volatility_1m,
                execution.probability_scale,
            )?;
            Some(ModelOutput {
                true_up: compute_true_probability_v2(
                    market_up_prob,
                    z,
                    execution.model_v2_k,
                    alignment,
                ),
                signal_context: compute_signal_context(
                    current_price,
                    predicted_price,
                    volatility_1m,
                    execution.probability_scale,
                    alignment,
                ),
                z_score: z,
            })
        }
    }
}

fn compute_signal_context(
    current_price: f64,
    predicted_price: f64,
    volatility_1m: f64,
    scale: f64,
    alignment: f64,
) -> Option<SignalContext> {
    let z_scaled = compute_signal_z_score(current_price, predicted_price, volatility_1m, scale)?;
    let confidence = z_scaled.abs().tanh() * alignment;
    let direction = if z_scaled > 0.0 {
        SIGNAL_DIR_UP
    } else if z_scaled < 0.0 {
        SIGNAL_DIR_DOWN
    } else {
        0
    };
    Some(SignalContext {
        direction,
        confidence: confidence.clamp(0.0, 1.0),
    })
}

fn compute_signal_z_score(
    current_price: f64,
    predicted_price: f64,
    volatility_1m: f64,
    scale: f64,
) -> Option<f64> {
    if current_price <= 0.0 {
        return None;
    }
    let delta = (predicted_price - current_price) / current_price;
    let volatility_5m = (volatility_1m * SQRT_5).max(1e-9);
    let z = delta / volatility_5m;
    Some(z * scale)
}

fn signal_allows_direction(
    execution: &ExecutionConfig,
    signal: Option<&SignalContext>,
    expected_direction: i8,
    edge_bps: f64,
) -> bool {
    if !execution.signal_direction_gate {
        return true;
    }
    let Some(signal) = signal else {
        return false;
    };
    if signal.direction == 0 || signal.direction == expected_direction {
        return true;
    }
    if execution.contrarian_min_edge_bps <= 0.0 {
        return false;
    }
    edge_bps >= execution.contrarian_min_edge_bps
        && signal.confidence >= execution.contrarian_confidence_min
}

fn compute_true_probability_5m(
    current_price: f64,
    predicted_price: f64,
    volatility_1m: f64,
    scale: f64,
    max_offset: f64,
    alignment: f64,
) -> f64 {
    if current_price <= 0.0 {
        return 0.5;
    }
    let delta = (predicted_price - current_price) / current_price;
    let volatility_5m = (volatility_1m * SQRT_5).max(1e-9);
    let z = delta / volatility_5m;
    let offset = (z * scale).tanh() * max_offset * alignment;
    (0.5 + offset).clamp(0.01, 0.99)
}

fn compute_true_probability_v2(market_up_prob: f64, z_score: f64, k: f64, alignment: f64) -> f64 {
    let prior = clamp_probability(market_up_prob);
    let update = z_score * k * alignment;
    let posterior_logit = logit(prior) + update;
    sigmoid(posterior_logit).clamp(0.01, 0.99)
}

fn clamp_probability(value: f64) -> f64 {
    value.clamp(0.01, 0.99)
}

fn logit(value: f64) -> f64 {
    let p = clamp_probability(value);
    (p / (1.0 - p)).ln()
}

fn sigmoid(value: f64) -> f64 {
    if value >= 0.0 {
        let exp = (-value).exp();
        1.0 / (1.0 + exp)
    } else {
        let exp = value.exp();
        exp / (1.0 + exp)
    }
}

fn is_polymarket_online(last_refresh: Option<Instant>) -> bool {
    last_refresh
        .map(|ts| ts.elapsed().as_millis() as u64 <= POLYMARKET_STALE_MS)
        .unwrap_or(false)
}

async fn load_active_windows(redis: &RedisManager) -> Result<Vec<ActiveWindowRow>> {
    let mut rows = Vec::new();
    for asset in ["BTC", "ETH", "SOL"] {
        match redis.get_asset_window(asset).await? {
            Some(window) => {
                let now = now_ms().unwrap_or(0);
                let status = if now >= window.start_time_ms && now < window.end_time_ms {
                    "ACTIVE"
                } else if now < window.start_time_ms {
                    "UPCOMING"
                } else {
                    "PAST"
                };
                rows.push(ActiveWindowRow {
                    asset: asset.to_string(),
                    market_id: window.market_id,
                    window_et: format_window_et(window.start_time_ms, window.end_time_ms),
                    status: status.to_string(),
                    start_time_ms: window.start_time_ms,
                    end_time_ms: window.end_time_ms,
                });
            }
            None => rows.push(ActiveWindowRow {
                asset: asset.to_string(),
                market_id: "--".to_string(),
                window_et: "--".to_string(),
                status: "NONE".to_string(),
                start_time_ms: 0,
                end_time_ms: 0,
            }),
        }
    }
    Ok(rows)
}

fn format_window_et(start_ms: u64, end_ms: u64) -> String {
    let start = Utc.timestamp_millis_opt(start_ms as i64).single();
    let end = Utc.timestamp_millis_opt(end_ms as i64).single();
    match (start, end) {
        (Some(start), Some(end)) => {
            let start_et = start.with_timezone(&New_York);
            let end_et = end.with_timezone(&New_York);
            format!(
                "{}-{} ET",
                start_et.format("%b %d %I:%M%p"),
                end_et.format("%I:%M%p")
            )
        }
        _ => "--".to_string(),
    }
}
