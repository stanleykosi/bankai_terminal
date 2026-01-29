/**
 * @description
 * Core engine event loop that consumes oracle updates and applies risk gates.
 *
 * @dependencies
 * - tokio: async event loop and select
 *
 * @notes
 * - Volatility halts are enforced when signals are neutral or missing.
 * - Uses RiskState to respect kill switch conditions.
 */
use arc_swap::ArcSwap;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::broadcast;

use crate::config::Config;
use crate::engine::risk::RiskState;
use crate::engine::types::{AlloraMarketUpdate, ChainlinkMarketUpdate, MarketUpdate};
use crate::error::{BankaiError, Result};
use crate::telemetry::metrics;

const ENGINE_TICK_INTERVAL: Duration = Duration::from_secs(5);
const NEUTRAL_SIGNAL_THRESHOLD_PCT: f64 = 0.001;

pub struct EngineCore {
    config: Arc<ArcSwap<Config>>,
    risk: Arc<RiskState>,
}

impl EngineCore {
    pub fn new(config: Arc<ArcSwap<Config>>, risk: Arc<RiskState>) -> Self {
        Self { config, risk }
    }

    pub fn spawn(self, receiver: broadcast::Receiver<MarketUpdate>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run(receiver).await {
                tracing::error!(?error, "engine core stopped");
            }
        })
    }

    async fn run(self, mut receiver: broadcast::Receiver<MarketUpdate>) -> Result<()> {
        let mut state = EngineState::new();
        let mut tick = tokio::time::interval(ENGINE_TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = tick.tick() => {
                    let snapshot = self.risk.snapshot();
                    if snapshot.halted {
                        tracing::warn!(?snapshot, "engine halted");
                    }
                }
                message = receiver.recv() => {
                    match message {
                        Ok(update) => {
                            self.handle_update(&mut state, update).await?;
                        }
                        Err(broadcast::error::RecvError::Lagged(skipped)) => {
                            tracing::warn!(skipped, "engine receiver lagged");
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            return Err(BankaiError::InvalidArgument("oracle channel closed".to_string()));
                        }
                    }
                }
            }
        }
    }

    async fn handle_update(&self, state: &mut EngineState, update: MarketUpdate) -> Result<()> {
        match update {
            MarketUpdate::Chainlink(update) => self.handle_chainlink_update(state, update).await,
            MarketUpdate::Allora(update) => self.handle_allora_update(state, update).await,
        }
    }

    async fn handle_chainlink_update(
        &self,
        state: &mut EngineState,
        update: ChainlinkMarketUpdate,
    ) -> Result<()> {
        let config = self.config.load_full();
        self.record_latency(update.event_time_ms)?;
        state
            .last_chainlink
            .insert(update.asset.clone(), update.clone());

        if self.risk.is_halted() {
            tracing::warn!(asset = %update.asset, "risk halt active; skipping chainlink update");
            return Ok(());
        }

        if let Some(volatility) = update.volatility_1m {
            if volatility > config.trading.max_volatility {
                let is_neutral =
                    is_neutral_signal(state.last_allora.get(&update.asset), resolve_price(&update));
                if is_neutral {
                    tracing::warn!(
                        asset = %update.asset,
                        volatility,
                        "volatility guard triggered with neutral signal"
                    );
                    self.risk.manual_halt();
                }
            }
        }

        Ok(())
    }

    async fn handle_allora_update(
        &self,
        state: &mut EngineState,
        update: AlloraMarketUpdate,
    ) -> Result<()> {
        if !is_five_min_signal(&update) {
            return Ok(());
        }
        state
            .last_allora
            .insert(update.asset.clone(), update.clone());
        if self.risk.is_halted() {
            tracing::warn!(asset = %update.asset, "risk halt active; skipping allora update");
        }
        Ok(())
    }

    fn record_latency(&self, event_time_ms: u64) -> Result<()> {
        if event_time_ms == 0 {
            return Ok(());
        }
        let now = now_ms()?;
        let latency_ms = now.saturating_sub(event_time_ms);
        if latency_ms > 0 {
            self.risk.record_latency_ms(latency_ms);
            metrics::record_latency_ms(latency_ms as f64);
        }
        Ok(())
    }
}

struct EngineState {
    last_chainlink: HashMap<String, ChainlinkMarketUpdate>,
    last_allora: HashMap<String, AlloraMarketUpdate>,
}

impl EngineState {
    fn new() -> Self {
        Self {
            last_chainlink: HashMap::new(),
            last_allora: HashMap::new(),
        }
    }
}

fn is_neutral_signal(allora: Option<&AlloraMarketUpdate>, price: Option<f64>) -> bool {
    let price = match price {
        Some(value) if value > 0.0 => value,
        _ => return true,
    };
    let allora = match allora {
        Some(value) => value,
        None => return true,
    };
    if allora.inference_value <= 0.0 {
        return true;
    }
    let delta = (allora.inference_value - price) / price;
    delta.abs() < NEUTRAL_SIGNAL_THRESHOLD_PCT
}

fn is_five_min_signal(update: &AlloraMarketUpdate) -> bool {
    update.timeframe.trim().eq_ignore_ascii_case("5m")
}

fn resolve_price(update: &ChainlinkMarketUpdate) -> Option<f64> {
    match update.last_price {
        Some(value) if value > 0.0 => Some(value),
        _ => None,
    }
}

fn now_ms() -> Result<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now.as_millis() as u64)
}
