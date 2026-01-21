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
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::collections::HashMap;
use std::io::{self, IsTerminal};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;

use crate::config::Config;
use crate::engine::analysis::snipe_threshold_bps;
use crate::engine::risk::{HaltReason, RiskState};
use crate::engine::types::{AlloraMarketUpdate, BinanceMarketUpdate, MarketUpdate};
use crate::error::Result;
use crate::storage::redis::RedisManager;

mod widgets;

const BANKROLL_KEY: &str = "sys:bankroll:usdc";
const DEFAULT_MARKET_LIMIT: usize = 12;
const DEFAULT_REFRESH_MS: u64 = 250;
const DEFAULT_BANKROLL_REFRESH_MS: u64 = 2_000;
const ORACLE_ONLINE_MULTIPLIER: u64 = 3;

type UiResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct TuiConfig {
    pub refresh_interval: Duration,
    pub bankroll_interval: Duration,
    pub market_limit: usize,
}

impl Default for TuiConfig {
    fn default() -> Self {
        Self {
            refresh_interval: Duration::from_millis(DEFAULT_REFRESH_MS),
            bankroll_interval: Duration::from_millis(DEFAULT_BANKROLL_REFRESH_MS),
            market_limit: DEFAULT_MARKET_LIMIT,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StatusBarData {
    pub uptime: Duration,
    pub halted: bool,
    pub halt_reason: HaltReason,
    pub binance_online: bool,
    pub allora_online: bool,
}

#[derive(Debug, Clone)]
pub struct HealthPanelData {
    pub halted: bool,
    pub halt_reason: HaltReason,
    pub latency_ms: u64,
    pub clock_drift_ms: i64,
    pub consecutive_losses: u32,
}

#[derive(Debug, Clone)]
pub struct FinancialPanelData {
    pub bankroll_usdc: Option<f64>,
    pub pnl_24h: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum MarketMode {
    NoSignal,
    Hold,
    Ladder,
    Snipe,
}

impl MarketMode {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NoSignal => "NO_SIGNAL",
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
    pub inference: Option<f64>,
    pub edge_bps: Option<f64>,
    pub fee_bps: Option<f64>,
    pub mode: MarketMode,
    pub last_update_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct UiSnapshot {
    pub status: StatusBarData,
    pub health: HealthPanelData,
    pub financials: FinancialPanelData,
    pub markets: Vec<MarketRow>,
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
    inference: Option<f64>,
    last_binance_ms: Option<u64>,
    last_allora_ms: Option<u64>,
}

impl MarketSnapshot {
    fn new(asset: String) -> Self {
        Self {
            asset,
            price: None,
            inference: None,
            last_binance_ms: None,
            last_allora_ms: None,
        }
    }

    fn apply_binance(&mut self, update: &BinanceMarketUpdate) {
        self.price = resolve_price(update);
        let event_time = if update.event_time_ms > 0 {
            update.event_time_ms
        } else {
            now_ms().unwrap_or(0)
        };
        self.last_binance_ms = Some(event_time);
    }

    fn apply_allora(&mut self, update: &AlloraMarketUpdate) {
        self.inference = Some(update.inference_value);
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
        if let Err(error) =
            snapshot_loop(config, risk, receiver, redis, snapshot_sender, ui_config).await
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
    sender: mpsc::Sender<UiCommand>,
    ui_config: TuiConfig,
) -> Result<()> {
    let mut market_state: HashMap<String, MarketSnapshot> = HashMap::new();
    let mut refresh = tokio::time::interval(ui_config.refresh_interval);
    let mut bankroll_interval = tokio::time::interval(ui_config.bankroll_interval);
    let start_time = Instant::now();
    let mut bankroll_usdc: Option<f64> = None;
    let pnl_24h: Option<f64> = None;

    loop {
        tokio::select! {
            message = receiver.recv() => {
                match message {
                    Ok(update) => match update {
                        MarketUpdate::Binance(update) => {
                            let entry = market_state.entry(update.asset.clone())
                                .or_insert_with(|| MarketSnapshot::new(update.asset.clone()));
                            entry.apply_binance(&update);
                        }
                        MarketUpdate::Allora(update) => {
                            let entry = market_state.entry(update.asset.clone())
                                .or_insert_with(|| MarketSnapshot::new(update.asset.clone()));
                            entry.apply_allora(&update);
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
                }
            }
            _ = refresh.tick() => {
                let snapshot = build_snapshot(
                    &config,
                    &risk,
                    &market_state,
                    start_time,
                    bankroll_usdc,
                    pnl_24h,
                    ui_config.market_limit,
                );
                if sender.send(UiCommand::Snapshot(snapshot)).is_err() {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn build_snapshot(
    config: &Arc<ArcSwap<Config>>,
    risk: &Arc<RiskState>,
    market_state: &HashMap<String, MarketSnapshot>,
    start_time: Instant,
    bankroll_usdc: Option<f64>,
    pnl_24h: Option<f64>,
    market_limit: usize,
) -> UiSnapshot {
    let config = config.load_full();
    let risk_snapshot = risk.snapshot();
    let now_ms = now_ms().unwrap_or(0);

    let status = StatusBarData {
        uptime: start_time.elapsed(),
        halted: risk_snapshot.halted,
        halt_reason: risk_snapshot.reason,
        binance_online: is_oracle_online(
            now_ms,
            last_binance_update_ms(market_state),
            Duration::from_secs(5),
        ),
        allora_online: is_oracle_online(
            now_ms,
            last_allora_update_ms(market_state),
            allora_online_window(&config),
        ),
    };

    let health = HealthPanelData {
        halted: risk_snapshot.halted,
        halt_reason: risk_snapshot.reason,
        latency_ms: risk_snapshot.last_latency_ms,
        clock_drift_ms: risk_snapshot.clock_drift_ms,
        consecutive_losses: risk_snapshot.consecutive_losses,
    };

    let financials = FinancialPanelData {
        bankroll_usdc,
        pnl_24h,
    };

    let snipe_floor = snipe_threshold_bps(&config.strategy, &config.fees);
    let mut markets: Vec<MarketRow> = market_state
        .values()
        .map(|snapshot| build_market_row(snapshot, snipe_floor, config.fees.taker_fee_bps))
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

    UiSnapshot {
        status,
        health,
        financials,
        markets,
    }
}

fn build_market_row(
    snapshot: &MarketSnapshot,
    snipe_floor_bps: f64,
    taker_fee_bps: f64,
) -> MarketRow {
    let edge_bps = match (snapshot.price, snapshot.inference) {
        (Some(price), Some(inference)) if price > 0.0 => Some((inference - price) / price * 10_000.0),
        _ => None,
    };

    let mode = match edge_bps {
        None => MarketMode::NoSignal,
        Some(value) if value > 0.0 && value >= snipe_floor_bps => MarketMode::Snipe,
        Some(value) if value > 0.0 => MarketMode::Ladder,
        _ => MarketMode::Hold,
    };

    let fee_bps = match mode {
        MarketMode::Snipe => Some(taker_fee_bps),
        MarketMode::Ladder => Some(0.0),
        _ => None,
    };

    let last_update_ms = snapshot
        .last_binance_ms
        .max(snapshot.last_allora_ms);

    MarketRow {
        asset: snapshot.asset.clone(),
        price: snapshot.price,
        inference: snapshot.inference,
        edge_bps,
        fee_bps,
        mode,
        last_update_ms,
    }
}

fn last_binance_update_ms(state: &HashMap<String, MarketSnapshot>) -> Option<u64> {
    state.values().filter_map(|snapshot| snapshot.last_binance_ms).max()
}

fn last_allora_update_ms(state: &HashMap<String, MarketSnapshot>) -> Option<u64> {
    state.values().filter_map(|snapshot| snapshot.last_allora_ms).max()
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
            binance_online: false,
            allora_online: false,
        },
        health: HealthPanelData {
            halted: false,
            halt_reason: HaltReason::None,
            latency_ms: 0,
            clock_drift_ms: 0,
            consecutive_losses: 0,
        },
        financials: FinancialPanelData {
            bankroll_usdc: None,
            pnl_24h: None,
        },
        markets: Vec::new(),
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

fn resolve_price(update: &BinanceMarketUpdate) -> Option<f64> {
    if let Some(value) = update.last_price {
        if value > 0.0 {
            return Some(value);
        }
    }
    match (update.best_bid, update.best_ask) {
        (Some(bid), Some(ask)) if bid > 0.0 && ask > 0.0 => Some((bid + ask) / 2.0),
        (Some(bid), None) if bid > 0.0 => Some(bid),
        (None, Some(ask)) if ask > 0.0 => Some(ask),
        _ => None,
    }
}

fn now_ms() -> Option<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    Some(now.as_millis() as u64)
}
