/**
 * @description
 * Chainlink oracle via Polymarket RTDS crypto_prices_chainlink stream.
 *
 * @dependencies
 * - tokio-tungstenite: websocket client
 * - futures-util: stream utilities
 * - serde_json: message parsing
 *
 * @notes
 * - Volatility uses a rolling window of returns.
 * - Start price snapshots align to the active market window in Redis.
 */
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

use crate::engine::types::{ChainlinkMarketUpdate, MarketUpdate};
use crate::error::{BankaiError, Result};
use crate::storage::redis::{AssetWindow, RedisManager};

const TOPIC: &str = "crypto_prices_chainlink";

fn compute_jitter(max_ms: u64) -> Result<Duration> {
    if max_ms == 0 {
        return Ok(Duration::from_millis(0));
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let jitter = now.subsec_millis() as u64 % (max_ms + 1);
    Ok(Duration::from_millis(jitter))
}

fn now_ms() -> Result<u64> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(duration.as_millis() as u64)
}

#[derive(Debug, Clone)]
pub struct ChainlinkOracleConfig {
    pub endpoint: String,
    pub symbols: Vec<String>,
    pub candle_interval: Duration,
    pub window_refresh_interval: Duration,
    pub redis: Option<RedisManager>,
}

pub struct ChainlinkOracle {
    config: ChainlinkOracleConfig,
}

impl ChainlinkOracle {
    pub fn new(config: ChainlinkOracleConfig) -> Self {
        Self { config }
    }

    pub fn spawn(self, sender: broadcast::Sender<MarketUpdate>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run(sender).await {
                tracing::error!(?error, "chainlink oracle stopped");
            }
        })
    }

    async fn run(self, sender: broadcast::Sender<MarketUpdate>) -> Result<()> {
        let mut states = HashMap::new();
        let mut asset_windows: HashMap<String, AssetWindow> = HashMap::new();
        let mut window_refresh = tokio::time::interval(self.config.window_refresh_interval);
        let mut backoff = ReconnectBackoff::new();

        loop {
            let endpoint = self.config.endpoint.clone();
            tracing::warn!(endpoint = %endpoint, "chainlink ws connecting");

            let ws_stream = match tokio_tungstenite::connect_async(&endpoint).await {
                Ok((stream, _)) => {
                    tracing::info!(endpoint = %endpoint, "chainlink ws connected");
                    backoff.reset();
                    stream
                }
                Err(error) => {
                    tracing::warn!(?error, endpoint = %endpoint, "chainlink ws connect failed");
                    let delay = backoff.next_delay()?;
                    tokio::time::sleep(delay).await;
                    continue;
                }
            };

            let (mut writer, mut reader) = ws_stream.split();
            let payload = subscribe_payload(&self.config.symbols)?;
            if let Err(error) = writer.send(Message::Text(payload)).await {
                tracing::warn!(?error, endpoint = %endpoint, "chainlink ws subscribe failed");
                let delay = backoff.next_delay()?;
                tokio::time::sleep(delay).await;
                continue;
            }

            loop {
                tokio::select! {
                    _ = window_refresh.tick() => {
                        if let Some(redis) = self.config.redis.as_ref() {
                            for symbol in &self.config.symbols {
                                let asset_key = canonical_asset(symbol);
                                if let Ok(Some(window)) = redis.get_asset_window(&asset_key).await {
                                    asset_windows.insert(asset_key, window);
                                }
                            }
                        }
                    }
                    message = reader.next() => {
                        match message {
                            Some(Ok(Message::Text(text))) => {
                                if let Some(mut event) = parse_event(&text)? {
                                    if event.event_time_ms == 0 {
                                        event.event_time_ms = now_ms().unwrap_or(0);
                                    }
                                    let asset_key = canonical_asset(&event.symbol);
                                    let state = states
                                        .entry(asset_key.clone())
                                        .or_insert_with(|| AssetState::new(self.config.candle_interval));
                                    if let Some(window) = asset_windows.get(&asset_key) {
                                        state.set_window(window);
                                        if let (Some(redis), Some((start_ms, price))) =
                                            (self.config.redis.as_ref(), state.take_start_price_if_ready(window))
                                        {
                                            let now = now_ms().unwrap_or(0);
                                            let _ = redis
                                                .set_asset_start_price(&asset_key, start_ms, price, now)
                                                .await;
                                        }
                                        if let (Some(redis), Some((end_ms, price))) = (
                                            self.config.redis.as_ref(),
                                            state.take_end_price_if_ready(
                                                window,
                                                event.event_time_ms,
                                                event.price,
                                            ),
                                        ) {
                                            let now = now_ms().unwrap_or(0);
                                            let _ = redis
                                                .set_asset_end_price(&asset_key, end_ms, price, now)
                                                .await;
                                        }
                                    }
                                    if let Some(update) = state.apply_event(&asset_key, event)? {
                                        let _ = sender.send(MarketUpdate::Chainlink(update));
                                    }
                                }
                            }
                            Some(Ok(Message::Ping(payload))) => {
                                if let Err(error) = writer.send(Message::Pong(payload)).await {
                                    tracing::warn!(?error, "chainlink ws pong failed");
                                    break;
                                }
                            }
                            Some(Ok(Message::Close(frame))) => {
                                tracing::warn!(?frame, endpoint = %endpoint, "chainlink ws closed");
                                break;
                            }
                            Some(Ok(_)) => {}
                            Some(Err(error)) => {
                                tracing::warn!(?error, endpoint = %endpoint, "chainlink ws error");
                                break;
                            }
                            None => {
                                tracing::warn!(endpoint = %endpoint, "chainlink ws stream ended");
                                break;
                            }
                        }
                    }
                }
            }

            let delay = backoff.next_delay()?;
            tokio::time::sleep(delay).await;
        }
    }
}

struct ReconnectBackoff {
    attempts: u32,
    base_ms: u64,
    max_ms: u64,
}

impl ReconnectBackoff {
    fn new() -> Self {
        Self {
            attempts: 0,
            base_ms: 500,
            max_ms: 30_000,
        }
    }

    fn reset(&mut self) {
        self.attempts = 0;
    }

    fn next_delay(&mut self) -> Result<Duration> {
        self.attempts = self.attempts.saturating_add(1);
        let exp = self
            .base_ms
            .saturating_mul(2u64.saturating_pow(self.attempts.min(6)));
        let capped = exp.min(self.max_ms);
        let jitter = compute_jitter(1_000)?;
        Ok(Duration::from_millis(capped).saturating_add(jitter))
    }
}

#[derive(Debug)]
struct ChainlinkEvent {
    symbol: String,
    price: f64,
    event_time_ms: u64,
}

struct AssetState {
    last_price: Option<f64>,
    volatility: RollingVolatility,
    last_volatility: Option<f64>,
    candle_start_ms: Option<u64>,
    candle_open_price: Option<f64>,
    candle_interval_ms: u64,
    anchor_start_ms: Option<u64>,
    recorded_start_ms: Option<u64>,
    recorded_end_ms: Option<u64>,
}

impl AssetState {
    fn new(candle_interval: Duration) -> Self {
        Self {
            last_price: None,
            volatility: RollingVolatility::new(candle_interval),
            last_volatility: None,
            candle_start_ms: None,
            candle_open_price: None,
            candle_interval_ms: candle_interval.as_millis() as u64,
            anchor_start_ms: None,
            recorded_start_ms: None,
            recorded_end_ms: None,
        }
    }

    fn set_window(&mut self, window: &AssetWindow) {
        let interval_ms = window.end_time_ms.saturating_sub(window.start_time_ms);
        if interval_ms == 0 {
            return;
        }
        let changed = self.anchor_start_ms != Some(window.start_time_ms)
            || self.candle_interval_ms != interval_ms;
        if changed {
            self.anchor_start_ms = Some(window.start_time_ms);
            self.candle_interval_ms = interval_ms;
            self.candle_start_ms = None;
            self.candle_open_price = None;
            self.volatility.reset(Duration::from_millis(interval_ms));
            self.recorded_start_ms = None;
            self.recorded_end_ms = None;
        }
    }

    fn take_start_price_if_ready(&mut self, window: &AssetWindow) -> Option<(u64, f64)> {
        let start_ms = window.start_time_ms;
        if self.recorded_start_ms == Some(start_ms) {
            return None;
        }
        if self.candle_start_ms != Some(start_ms) {
            return None;
        }
        let price = self.candle_open_price?;
        if price <= 0.0 {
            return None;
        }
        self.recorded_start_ms = Some(start_ms);
        Some((start_ms, price))
    }

    fn take_end_price_if_ready(
        &mut self,
        window: &AssetWindow,
        event_time_ms: u64,
        price: f64,
    ) -> Option<(u64, f64)> {
        if price <= 0.0 {
            return None;
        }
        let end_ms = window.end_time_ms;
        if self.recorded_end_ms == Some(end_ms) {
            return None;
        }
        if event_time_ms < end_ms {
            return None;
        }
        self.recorded_end_ms = Some(end_ms);
        Some((end_ms, price))
    }

    fn apply_event(
        &mut self,
        asset: &str,
        event: ChainlinkEvent,
    ) -> Result<Option<ChainlinkMarketUpdate>> {
        self.update_candle(event.event_time_ms, event.price);
        self.last_price = Some(event.price);
        if let Some(vol) = self.volatility.update(event.price, event.event_time_ms) {
            self.last_volatility = Some(vol);
        }
        let dfo = self
            .candle_open_price
            .map(|open| (event.price - open) / open);

        Ok(Some(ChainlinkMarketUpdate {
            asset: asset.to_string(),
            last_price: Some(event.price),
            volatility_1m: self.last_volatility,
            dfo,
            event_time_ms: event.event_time_ms,
        }))
    }

    fn update_candle(&mut self, timestamp_ms: u64, price: f64) {
        if self.candle_interval_ms == 0 {
            return;
        }
        let candle_start = if let Some(anchor) = self.anchor_start_ms {
            if timestamp_ms < anchor {
                return;
            }
            let offset = timestamp_ms.saturating_sub(anchor);
            anchor + (offset / self.candle_interval_ms) * self.candle_interval_ms
        } else {
            timestamp_ms - (timestamp_ms % self.candle_interval_ms)
        };
        if self.candle_start_ms != Some(candle_start) {
            self.candle_start_ms = Some(candle_start);
            self.candle_open_price = Some(price);
        }
    }
}

struct RollingVolatility {
    window: Duration,
    samples: VecDeque<(u64, f64)>,
    last_price: Option<f64>,
}

impl RollingVolatility {
    fn new(window: Duration) -> Self {
        Self {
            window,
            samples: VecDeque::new(),
            last_price: None,
        }
    }

    fn reset(&mut self, window: Duration) {
        self.window = window;
        self.samples.clear();
        self.last_price = None;
    }

    fn update(&mut self, price: f64, timestamp_ms: u64) -> Option<f64> {
        if let Some(prev) = self.last_price {
            if prev > 0.0 {
                let ret = (price - prev) / prev;
                self.samples.push_back((timestamp_ms, ret));
            }
        }
        self.last_price = Some(price);
        self.prune(timestamp_ms);
        compute_stddev(&self.samples)
    }

    fn prune(&mut self, now_ms: u64) {
        let window_ms = self.window.as_millis() as u64;
        while let Some((timestamp, _)) = self.samples.front() {
            if now_ms.saturating_sub(*timestamp) > window_ms {
                self.samples.pop_front();
            } else {
                break;
            }
        }
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

fn compute_stddev(samples: &VecDeque<(u64, f64)>) -> Option<f64> {
    let count = samples.len();
    if count < 2 {
        return None;
    }
    let sum: f64 = samples.iter().map(|(_, value)| value).sum();
    let mean = sum / count as f64;
    let variance = samples
        .iter()
        .map(|(_, value)| {
            let diff = value - mean;
            diff * diff
        })
        .sum::<f64>()
        / count as f64;
    Some(variance.sqrt())
}

fn subscribe_payload(symbols: &[String]) -> Result<String> {
    if symbols.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "chainlink oracle requires at least one symbol".to_string(),
        ));
    }
    let subscriptions: Vec<Value> = symbols
        .iter()
        .map(|symbol| {
            let normalized = symbol.trim().to_ascii_lowercase();
            let filters = format!(r#"{{"symbol":"{normalized}"}}"#);
            serde_json::json!({
                "topic": TOPIC,
                "type": "update",
                "filters": filters
            })
        })
        .collect();
    let payload = serde_json::json!({
        "action": "subscribe",
        "subscriptions": subscriptions
    });
    Ok(payload.to_string())
}

fn parse_event(text: &str) -> Result<Option<ChainlinkEvent>> {
    let raw: Value = serde_json::from_str(text)?;
    let topic = raw
        .get("topic")
        .and_then(|value| value.as_str())
        .unwrap_or_default();
    if topic != TOPIC {
        return Ok(None);
    }
    let event_type = raw.get("type").and_then(|value| value.as_str());
    if !matches!(event_type, Some("update") | Some("*") | None) {
        return Ok(None);
    }
    let payload = raw
        .get("payload")
        .ok_or_else(|| BankaiError::InvalidArgument("chainlink payload missing".to_string()))?;
    let symbol = payload
        .get("symbol")
        .and_then(|value| value.as_str())
        .ok_or_else(|| BankaiError::InvalidArgument("chainlink symbol missing".to_string()))?;
    let price = match payload.get("value") {
        Some(value) if value.is_number() => value.as_f64().unwrap_or(0.0),
        Some(value) if value.is_string() => value
            .as_str()
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap_or(0.0),
        _ => 0.0,
    };
    if price <= 0.0 {
        return Ok(None);
    }
    let event_time_ms = payload
        .get("timestamp")
        .or_else(|| raw.get("timestamp"))
        .and_then(|value| value.as_u64())
        .unwrap_or(0);

    Ok(Some(ChainlinkEvent {
        symbol: symbol.to_string(),
        price,
        event_time_ms,
    }))
}
