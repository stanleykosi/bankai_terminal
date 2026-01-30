/**
 * @description
 * Trading engine that turns oracle updates into TradeIntents.
 *
 * @dependencies
 * - tokio: async runtime and channel handling
 * - arc-swap: live config access
 *
 * @notes
 * - Requires Redis market metadata and Polymarket order books to be available.
 * - Emits TradeIntent only when signals are fresh and within the market window.
 */
use arc_swap::ArcSwap;
use chrono::Utc;
use chrono_tz::America::New_York;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{broadcast, mpsc};

use crate::config::{Config, FeeConfig};
use crate::engine::analysis::{analyze_opportunity, AnalysisInput, TradeDecision};
use crate::engine::risk::RiskState;
use crate::engine::types::{
    AlloraMarketUpdate, ChainlinkMarketUpdate, MarketUpdate, MarketWindow, TradeIntent, TradeMode,
};
use crate::error::{BankaiError, Result};
use crate::storage::orderbook::{BookSide, OrderBookStore};
use crate::storage::redis::RedisManager;

const DEFAULT_TICK_INTERVAL: Duration = Duration::from_secs(5);
const ACTIVITY_LOG_LIMIT: usize = 50;
const SIGNAL_HORIZON_MS: u64 = 5 * 60 * 1_000;
const SQRT_5: f64 = 2.236_067_977_5;
const ORDERBOOK_STALE_MS: u64 = 5_000;

pub struct TradingEngine {
    config: Arc<ArcSwap<Config>>,
    risk: Arc<RiskState>,
    redis: RedisManager,
    orderbook: OrderBookStore,
    intent_tx: mpsc::Sender<TradeIntent>,
    wallet_key: Option<String>,
    gamma_client: Client,
}

impl TradingEngine {
    pub fn new(
        config: Arc<ArcSwap<Config>>,
        risk: Arc<RiskState>,
        redis: RedisManager,
        orderbook: OrderBookStore,
        intent_tx: mpsc::Sender<TradeIntent>,
        wallet_key: Option<String>,
    ) -> Self {
        let gamma_client = Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .expect("gamma client build");
        Self {
            config,
            risk,
            redis,
            orderbook,
            intent_tx,
            wallet_key,
            gamma_client,
        }
    }

    pub fn spawn(self, receiver: broadcast::Receiver<MarketUpdate>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run(receiver).await {
                tracing::error!(?error, "trading engine stopped");
            }
        })
    }

    async fn run(self, mut receiver: broadcast::Receiver<MarketUpdate>) -> Result<()> {
        let mut state = TraderState::new();
        let mut tick = tokio::time::interval(DEFAULT_TICK_INTERVAL);

        loop {
            tokio::select! {
                _ = tick.tick() => {
                    if self.risk.is_halted() {
                        tracing::warn!("trading engine halted");
                    }
                }
                message = receiver.recv() => {
                    match message {
                        Ok(update) => {
                            self.handle_update(&mut state, update).await?;
                        }
                        Err(broadcast::error::RecvError::Lagged(skipped)) => {
                            tracing::warn!(skipped, "trading engine receiver lagged");
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            return Err(BankaiError::InvalidArgument("oracle channel closed".to_string()));
                        }
                    }
                }
            }
        }
    }

    async fn handle_update(&self, state: &mut TraderState, update: MarketUpdate) -> Result<()> {
        match update {
            MarketUpdate::Chainlink(update) => {
                state
                    .last_chainlink
                    .insert(update.asset.clone(), update.clone());
                self.evaluate_asset(state, &update.asset).await?;
            }
            MarketUpdate::Allora(update) => {
                let key = format!("{}:{}", update.asset, update.timeframe);
                state.last_allora.insert(key, update.clone());
                self.evaluate_asset(state, &update.asset).await?;
            }
        }
        Ok(())
    }

    async fn evaluate_asset(&self, state: &mut TraderState, asset: &str) -> Result<()> {
        if self.risk.is_halted() {
            return Ok(());
        }
        let config = self.config.load_full();
        if !config.execution.enable_trading && !config.execution.no_money_mode {
            return Ok(());
        }

        let now = now_ms()?;
        if let Some(last_intent) = state.last_intent_ms.get(asset) {
            let cooldown = config.execution.order_cooldown_secs * 1000;
            if now.saturating_sub(*last_intent) < cooldown {
                return Ok(());
            }
        }

        let Some(chainlink) = state.last_chainlink.get(asset) else {
            return Ok(());
        };

        let _current_price = resolve_chainlink_price(chainlink)
            .ok_or_else(|| BankaiError::InvalidArgument("chainlink price missing".to_string()))?;
        let volatility = chainlink
            .volatility_1m
            .unwrap_or(config.execution.min_volatility)
            .max(config.execution.min_volatility);

        let asset_window = self
            .redis
            .get_asset_window(asset)
            .await?
            .ok_or_else(|| BankaiError::InvalidArgument("asset window missing".to_string()))?;
        let window = MarketWindow {
            start_time_ms: asset_window.start_time_ms,
            end_time_ms: asset_window.end_time_ms,
        };

        let max_align_ms = config
            .execution
            .signal_alignment_max_secs
            .saturating_mul(1000);
        let Some(aligned) =
            select_aligned_5m_signal(&state.last_allora, asset, window, now, max_align_ms)
        else {
            let last = state.last_signal_miss_ms.get(asset).copied().unwrap_or(0);
            if now.saturating_sub(last) > 30_000 {
                self.log_alert(asset, "missing aligned 10m signal; skipping window")
                    .await;
                state.last_signal_miss_ms.insert(asset.to_string(), now);
            }
            return Ok(());
        };
        let allora = aligned.update;
        let alignment = aligned.alignment;
        if now.saturating_sub(allora.signal_timestamp_ms) > SIGNAL_HORIZON_MS {
            return Ok(());
        }

        let Some((start_time_ms, start_price)) = self.redis.get_asset_start_price(asset).await?
        else {
            let last = state
                .last_start_price_alert_ms
                .get(asset)
                .copied()
                .unwrap_or(0);
            if now.saturating_sub(last) > 30_000 {
                self.log_alert(asset, "start price missing for active window")
                    .await;
                state
                    .last_start_price_alert_ms
                    .insert(asset.to_string(), now);
            }
            return Ok(());
        };
        if start_time_ms != window.start_time_ms || start_price <= 0.0 {
            let last = state
                .last_start_price_alert_ms
                .get(asset)
                .copied()
                .unwrap_or(0);
            if now.saturating_sub(last) > 30_000 {
                self.log_alert(asset, "start price mismatch for active window")
                    .await;
                state
                    .last_start_price_alert_ms
                    .insert(asset.to_string(), now);
            }
            return Ok(());
        }

        let metadata = self
            .redis
            .get_market_metadata(&asset_window.market_id)
            .await?;
        let up_token = metadata
            .outcome_up_token_id
            .ok_or_else(|| BankaiError::InvalidArgument("up token id missing".to_string()))?;
        let down_token = metadata
            .outcome_down_token_id
            .ok_or_else(|| BankaiError::InvalidArgument("down token id missing".to_string()))?;
        let up_fee_bps = self.redis.get_fee_rate_bps(&up_token).await?;
        let down_fee_bps = self.redis.get_fee_rate_bps(&down_token).await?;
        if up_fee_bps.is_none() || down_fee_bps.is_none() {
            let last_fee = state.last_fee_alert_ms.get(asset).copied().unwrap_or(0);
            if now.saturating_sub(last_fee) > 60_000 {
                if up_fee_bps.is_none() {
                    self.log_alert(asset, "missing fee rate for UP token").await;
                }
                if down_fee_bps.is_none() {
                    self.log_alert(asset, "missing fee rate for DOWN token")
                        .await;
                }
                state.last_fee_alert_ms.insert(asset.to_string(), now);
            }
        }

        if let Some(exit_intent) = self
            .check_exit_intent(asset, &asset_window.market_id, &up_token)
            .await?
        {
            self.log_intent(asset, &exit_intent).await;
            let _ = self.intent_tx.send(exit_intent).await;
            state.last_intent_ms.insert(asset.to_string(), now);
            return Ok(());
        }
        if let Some(exit_intent) = self
            .check_exit_intent(asset, &asset_window.market_id, &down_token)
            .await?
        {
            self.log_intent(asset, &exit_intent).await;
            let _ = self.intent_tx.send(exit_intent).await;
            state.last_intent_ms.insert(asset.to_string(), now);
            return Ok(());
        }

        let mut implied_up = self.orderbook.mid_price(&up_token).await?;
        let mut implied_down = self.orderbook.mid_price(&down_token).await?;
        if is_orderbook_stale(&self.orderbook, &up_token, now).await? {
            self.log_alert(asset, "orderbook stale for UP token; using gamma fallback")
                .await;
            if let Some((up, down)) = fetch_outcomes_from_gamma(
                &self.gamma_client,
                &config.endpoints.polymarket_gamma,
                &asset_window.market_id,
            )
            .await
            {
                implied_up = implied_up.or(Some(up));
                implied_down = implied_down.or(Some(down));
            }
        }
        if is_orderbook_stale(&self.orderbook, &down_token, now).await? {
            self.log_alert(
                asset,
                "orderbook stale for DOWN token; using gamma fallback",
            )
            .await;
            if let Some((up, down)) = fetch_outcomes_from_gamma(
                &self.gamma_client,
                &config.endpoints.polymarket_gamma,
                &asset_window.market_id,
            )
            .await
            {
                implied_up = implied_up.or(Some(up));
                implied_down = implied_down.or(Some(down));
            }
        }
        let implied_up = implied_up
            .ok_or_else(|| BankaiError::InvalidArgument("up book mid missing".to_string()))?;
        let implied_down = implied_down.unwrap_or_else(|| (1.0 - implied_up).max(0.0));

        let true_up = compute_true_probability_5m(
            start_price,
            allora.inference_value,
            volatility,
            config.execution.probability_scale,
            config.execution.probability_max_offset,
            alignment,
        );
        let true_down = (1.0 - true_up).clamp(0.0, 1.0);

        let mut best_intent: Option<TradeIntent> = None;
        let (up_fees, up_fee_missing) = fee_config_for_token(&config, up_fee_bps);
        if let Ok(result) = analyze_opportunity(
            AnalysisInput {
                market_id: asset_window.market_id.clone(),
                asset_id: up_token.clone(),
                implied_prob: implied_up,
                true_prob: true_up,
                timestamp_ms: now,
                market_window: Some(window),
            },
            &config.strategy,
            &up_fees,
        ) {
            if let Some(mut intent) = result.intent {
                if up_fee_missing {
                    match result.decision {
                        TradeDecision::Snipe => {
                            self.log_alert(asset, "fee rate missing; snipe intent blocked")
                                .await;
                        }
                        TradeDecision::Ladder => {
                            self.log_alert(asset, "fee rate missing; ladder intent allowed")
                                .await;
                        }
                        _ => {}
                    }
                }
                if !(up_fee_missing && result.decision == TradeDecision::Snipe) {
                    if result.decision == TradeDecision::Snipe {
                        if let Some(vwap) = estimate_snipe_vwap(
                            &self.orderbook,
                            &self.redis,
                            &up_token,
                            true_up,
                            implied_up,
                            &config,
                        )
                        .await?
                        {
                            if let Ok(vwap_result) = analyze_opportunity(
                                AnalysisInput {
                                    market_id: asset_window.market_id.clone(),
                                    asset_id: up_token.clone(),
                                    implied_prob: vwap,
                                    true_prob: true_up,
                                    timestamp_ms: now,
                                    market_window: Some(window),
                                },
                                &config.strategy,
                                &up_fees,
                            ) {
                                if vwap_result.decision == TradeDecision::Snipe {
                                    if let Some(vwap_intent) = vwap_result.intent {
                                        intent = vwap_intent;
                                    }
                                }
                            }
                        }
                    }
                    best_intent = Some(intent);
                }
            }
        }

        let (down_fees, down_fee_missing) = fee_config_for_token(&config, down_fee_bps);
        if let Ok(result) = analyze_opportunity(
            AnalysisInput {
                market_id: asset_window.market_id.clone(),
                asset_id: down_token.clone(),
                implied_prob: implied_down,
                true_prob: true_down,
                timestamp_ms: now,
                market_window: Some(window),
            },
            &config.strategy,
            &down_fees,
        ) {
            if let Some(mut intent) = result.intent {
                if down_fee_missing {
                    match result.decision {
                        TradeDecision::Snipe => {
                            self.log_alert(asset, "fee rate missing; snipe intent blocked")
                                .await;
                        }
                        TradeDecision::Ladder => {
                            self.log_alert(asset, "fee rate missing; ladder intent allowed")
                                .await;
                        }
                        _ => {}
                    }
                }
                if !(down_fee_missing && result.decision == TradeDecision::Snipe) {
                    if result.decision == TradeDecision::Snipe {
                        if let Some(vwap) = estimate_snipe_vwap(
                            &self.orderbook,
                            &self.redis,
                            &down_token,
                            true_down,
                            implied_down,
                            &config,
                        )
                        .await?
                        {
                            if let Ok(vwap_result) = analyze_opportunity(
                                AnalysisInput {
                                    market_id: asset_window.market_id.clone(),
                                    asset_id: down_token.clone(),
                                    implied_prob: vwap,
                                    true_prob: true_down,
                                    timestamp_ms: now,
                                    market_window: Some(window),
                                },
                                &config.strategy,
                                &down_fees,
                            ) {
                                if vwap_result.decision == TradeDecision::Snipe {
                                    if let Some(vwap_intent) = vwap_result.intent {
                                        intent = vwap_intent;
                                    }
                                }
                            }
                        }
                    }
                    let replace = match best_intent.as_ref() {
                        Some(current) => intent.edge_bps > current.edge_bps,
                        None => true,
                    };
                    if replace {
                        best_intent = Some(intent);
                    }
                }
            }
        }

        if let Some(intent) = best_intent {
            if !config.execution.no_money_mode {
                if let Some(min_size) = metadata.min_order_size {
                    if let Ok(Some(bankroll)) = self.redis.get_float("sys:bankroll:usdc").await {
                        let required = min_size * intent.implied_prob.max(0.0);
                        if required > bankroll {
                            let last = state
                                .last_min_order_alert_ms
                                .get(asset)
                                .copied()
                                .unwrap_or(0);
                            if now.saturating_sub(last) > 30_000 {
                                self.log_alert(
                                    asset,
                                    "bankroll below min order notional; skipping intent",
                                )
                                .await;
                                state.last_min_order_alert_ms.insert(asset.to_string(), now);
                            }
                            return Ok(());
                        }
                    }
                }
            }
            self.log_intent(asset, &intent).await;
            let _ = self.intent_tx.send(intent).await;
            state.last_intent_ms.insert(asset.to_string(), now);
        }

        Ok(())
    }
}

impl TradingEngine {
    async fn log_intent(&self, asset: &str, intent: &TradeIntent) {
        let prefix = log_prefix();
        let side = match intent.side {
            crate::engine::types::TradeSide::Buy => "BUY",
            crate::engine::types::TradeSide::Sell => "SELL",
        };
        let message = format!(
            "{prefix} [INTENT] {asset} {side} mode={:?} edge_bps={:.1} implied={:.4} true={:.4} market={} token={}",
            intent.mode,
            intent.edge_bps,
            intent.implied_prob,
            intent.true_prob,
            intent.market_id,
            intent.asset_id
        );
        let _ = self
            .redis
            .push_activity_log(&message, ACTIVITY_LOG_LIMIT)
            .await;
        let _ = self
            .redis
            .push_intent_log(&message, ACTIVITY_LOG_LIMIT)
            .await;
    }
}

impl TradingEngine {
    async fn log_alert(&self, asset: &str, message: &str) {
        let prefix = log_prefix();
        let entry = format!("{prefix} [ALERT] {asset} {message}");
        let _ = self
            .redis
            .push_activity_log(&entry, ACTIVITY_LOG_LIMIT)
            .await;
    }

    async fn check_exit_intent(
        &self,
        _asset: &str,
        market_id: &str,
        token_id: &str,
    ) -> Result<Option<TradeIntent>> {
        let Some(wallet_key) = self.wallet_key.as_ref() else {
            return Ok(None);
        };
        let position = self.position_size(wallet_key, token_id).await?;
        if position <= 0.0 {
            return Ok(None);
        }
        let Some(entry_price) = self.redis.get_entry_price(wallet_key, token_id).await? else {
            return Ok(None);
        };
        if entry_price <= 0.0 {
            return Ok(None);
        }
        let current_price = self
            .orderbook
            .mid_price(token_id)
            .await?
            .unwrap_or(entry_price);

        let config = self.config.load_full();
        let execution = &config.execution;
        let mut peak_price = self
            .redis
            .get_peak_price(wallet_key, token_id)
            .await?
            .unwrap_or(entry_price);
        if current_price > peak_price {
            peak_price = current_price;
            let _ = self
                .redis
                .set_peak_price(wallet_key, token_id, peak_price)
                .await;
        }

        let take_profit = entry_price * (1.0 + execution.take_profit_bps / 10_000.0);
        let stop_loss = entry_price * (1.0 - execution.stop_loss_bps / 10_000.0);
        let trailing_stop = if execution.trailing_stop_bps > 0.0 {
            peak_price * (1.0 - execution.trailing_stop_bps / 10_000.0)
        } else {
            0.0
        };

        let should_exit = current_price >= take_profit
            || current_price <= stop_loss
            || (execution.trailing_stop_bps > 0.0 && current_price <= trailing_stop);
        if !should_exit {
            return Ok(None);
        }

        let close_size = (position * execution.close_fraction).max(0.0);
        if close_size <= 0.0 {
            return Ok(None);
        }

        let edge = (current_price - entry_price) / entry_price;
        let edge_bps = edge * 10_000.0;
        let now = now_ms()?;

        Ok(Some(TradeIntent {
            market_id: market_id.to_string(),
            asset_id: token_id.to_string(),
            side: crate::engine::types::TradeSide::Sell,
            mode: TradeMode::Snipe,
            implied_prob: current_price,
            true_prob: current_price,
            edge,
            edge_bps,
            spread_offset_bps: config.strategy.spread_offset_bps,
            timestamp_ms: now,
            market_window: None,
            requested_size: Some(close_size),
        }))
    }

    async fn position_size(&self, wallet_key: &str, token_id: &str) -> Result<f64> {
        let tracked = self
            .redis
            .get_tracked_position(wallet_key, token_id)
            .await?;
        if tracked > 0.0 {
            return Ok(tracked);
        }
        let onchain_key = format!("positions:ctf:{wallet_key}");
        Ok(self
            .redis
            .hget_float(&onchain_key, token_id)
            .await?
            .unwrap_or(0.0))
    }
}

struct TraderState {
    last_chainlink: HashMap<String, ChainlinkMarketUpdate>,
    last_allora: HashMap<String, AlloraMarketUpdate>,
    last_intent_ms: HashMap<String, u64>,
    last_signal_miss_ms: HashMap<String, u64>,
    last_fee_alert_ms: HashMap<String, u64>,
    last_start_price_alert_ms: HashMap<String, u64>,
    last_min_order_alert_ms: HashMap<String, u64>,
}

impl TraderState {
    fn new() -> Self {
        Self {
            last_chainlink: HashMap::new(),
            last_allora: HashMap::new(),
            last_intent_ms: HashMap::new(),
            last_signal_miss_ms: HashMap::new(),
            last_fee_alert_ms: HashMap::new(),
            last_start_price_alert_ms: HashMap::new(),
            last_min_order_alert_ms: HashMap::new(),
        }
    }
}

struct AlignedSignal {
    update: AlloraMarketUpdate,
    alignment: f64,
}

fn select_aligned_5m_signal(
    updates: &HashMap<String, AlloraMarketUpdate>,
    asset: &str,
    window: MarketWindow,
    now_ms: u64,
    max_alignment_ms: u64,
) -> Option<AlignedSignal> {
    let target_ts = window.end_time_ms.saturating_sub(SIGNAL_HORIZON_MS);
    let mut best: Option<(AlloraMarketUpdate, u64)> = None;

    for (key, update) in updates.iter() {
        if !key.starts_with(&format!("{asset}:")) {
            continue;
        }
        if !update.timeframe.trim().eq_ignore_ascii_case("5m") {
            continue;
        }
        if update.signal_timestamp_ms > now_ms {
            continue;
        }
        if update.signal_timestamp_ms < window.start_time_ms
            || update.signal_timestamp_ms > window.end_time_ms
        {
            continue;
        }

        let diff = update.signal_timestamp_ms.abs_diff(target_ts);
        if max_alignment_ms > 0 && diff > max_alignment_ms {
            continue;
        }

        match best.as_ref() {
            Some((_, best_diff)) if diff >= *best_diff => {}
            _ => best = Some((update.clone(), diff)),
        }
    }

    best.map(|(update, diff)| {
        let denom = max_alignment_ms.max(1) as f64;
        let alignment = 1.0 - (diff as f64 / denom);
        AlignedSignal {
            update,
            alignment: alignment.clamp(0.0, 1.0),
        }
    })
}

fn resolve_chainlink_price(update: &ChainlinkMarketUpdate) -> Option<f64> {
    match update.last_price {
        Some(price) if price > 0.0 => Some(price),
        _ => None,
    }
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

fn fee_config_for_token(config: &Config, fee_rate_bps: Option<f64>) -> (FeeConfig, bool) {
    let mut fees = config.fees.clone();
    if let Some(value) = fee_rate_bps {
        fees.taker_fee_bps = value;
        return (fees, false);
    }
    (fees, true)
}

async fn estimate_snipe_vwap(
    orderbook: &OrderBookStore,
    redis: &RedisManager,
    token_id: &str,
    true_prob: f64,
    mid_price: f64,
    config: &Config,
) -> Result<Option<f64>> {
    if mid_price <= 0.0 {
        return Ok(None);
    }
    let size = estimate_order_size(
        redis,
        true_prob,
        mid_price,
        &config.execution,
        &config.strategy,
    )
    .await?;
    let vwap = orderbook
        .vwap_for_size(token_id, BookSide::Ask, size, 50)
        .await?;
    Ok(vwap.map(|value| value.avg_price))
}

async fn estimate_order_size(
    redis: &RedisManager,
    true_prob: f64,
    price: f64,
    execution: &crate::config::ExecutionConfig,
    strategy: &crate::config::StrategyConfig,
) -> Result<f64> {
    let bankroll = redis.get_float("sys:bankroll:usdc").await?;
    let odds = if price > 0.0 { 1.0 / price } else { 0.0 };
    let kelly = calculate_kelly(true_prob, odds);
    let target = if let Some(bankroll) = bankroll {
        bankroll * strategy.kelly_fraction * kelly
    } else {
        execution.default_order_usdc
    };
    let mut notional = target.clamp(execution.min_order_usdc, execution.max_order_usdc);
    if notional < execution.min_order_usdc {
        return Err(BankaiError::InvalidArgument(
            "order notional below min".to_string(),
        ));
    }
    if notional > execution.max_order_usdc {
        notional = execution.max_order_usdc;
    }
    Ok(notional / price)
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

async fn is_orderbook_stale(
    orderbook: &OrderBookStore,
    token_id: &str,
    now_ms: u64,
) -> Result<bool> {
    let Some(last_update) = orderbook.last_update_ms(token_id).await? else {
        return Ok(true);
    };
    Ok(now_ms.saturating_sub(last_update) > ORDERBOOK_STALE_MS)
}

async fn fetch_outcomes_from_gamma(
    client: &Client,
    base_url: &str,
    market_id: &str,
) -> Option<(f64, f64)> {
    if base_url.trim().is_empty() || market_id.trim().is_empty() {
        return None;
    }
    let url = format!("{}/markets/{}", base_url.trim_end_matches('/'), market_id);
    let response = client.get(url).send().await.ok()?;
    if !response.status().is_success() {
        return None;
    }
    let body: serde_json::Value = response.json().await.ok()?;
    let outcomes = body.get("outcomes")?;
    let prices = body.get("outcomePrices")?;

    let outcomes: Vec<String> = match outcomes {
        serde_json::Value::String(value) => serde_json::from_str(value).ok()?,
        serde_json::Value::Array(values) => values
            .iter()
            .filter_map(|value| value.as_str().map(|s| s.to_string()))
            .collect(),
        _ => return None,
    };
    let prices: Vec<f64> = match prices {
        serde_json::Value::String(value) => serde_json::from_str(value).ok()?,
        serde_json::Value::Array(values) => values
            .iter()
            .filter_map(|value| value.as_str().and_then(|s| s.parse::<f64>().ok()))
            .collect(),
        _ => return None,
    };

    if outcomes.len() != prices.len() {
        return None;
    }
    let mut up = None;
    let mut down = None;
    for (outcome, price) in outcomes.iter().zip(prices.iter()) {
        if outcome.eq_ignore_ascii_case("up") {
            up = Some(*price);
        } else if outcome.eq_ignore_ascii_case("down") {
            down = Some(*price);
        }
    }
    match (up, down) {
        (Some(up), Some(down)) => Some((up, down)),
        (Some(up), None) => Some((up, (1.0 - up).max(0.0))),
        _ => None,
    }
}

fn now_ms() -> Result<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now.as_millis() as u64)
}

fn log_prefix() -> String {
    let now = Utc::now();
    let et = now.with_timezone(&New_York);
    format!("[{}]", et.format("%H:%M:%S"))
}
