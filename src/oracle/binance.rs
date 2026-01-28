/**
 * @description
 * Binance oracle for aggTrade and bookTicker streams with rolling volatility + DFO.
 *
 * @dependencies
 * - tokio-tungstenite: websocket client
 * - futures-util: stream utilities
 * - serde_json: message parsing
 *
 * @notes
 * - Volatility uses 1-minute rolling stddev of returns.
 * - DFO aligns candle start to epoch-based interval boundaries.
 */
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

use crate::engine::types::{BinanceMarketUpdate, MarketUpdate};
use crate::error::{BankaiError, Result};
use crate::storage::redis::{AssetWindow, RedisManager};

const STREAM_ID: u64 = 1;

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
pub struct BinanceOracleConfig {
    pub endpoint: String,
    pub symbols: Vec<String>,
    pub candle_interval: Duration,
    pub window_refresh_interval: Duration,
    pub redis: Option<RedisManager>,
}

pub struct BinanceOracle {
    config: BinanceOracleConfig,
}

impl BinanceOracle {
    pub fn new(config: BinanceOracleConfig) -> Self {
        Self { config }
    }

    pub fn spawn(self, sender: broadcast::Sender<MarketUpdate>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run(sender).await {
                tracing::error!(?error, "binance oracle stopped");
            }
        })
    }

    async fn run(self, sender: broadcast::Sender<MarketUpdate>) -> Result<()> {
        let mut states = HashMap::new();
        let mut asset_windows: HashMap<String, AssetWindow> = HashMap::new();
        let mut window_refresh = tokio::time::interval(self.config.window_refresh_interval);
        let mut backoff = ReconnectBackoff::new();
        let endpoints = build_ws_endpoints(&self.config.endpoint);

        let mut endpoint_index = 0usize;
        let mut primary_failures = 0u32;

        loop {
            let endpoint = endpoints
                .get(endpoint_index)
                .cloned()
                .unwrap_or_else(|| self.config.endpoint.clone());
            tracing::warn!(endpoint = %endpoint, "binance ws connecting");

            let ws_stream = match tokio_tungstenite::connect_async(&endpoint).await {
                Ok((stream, _)) => {
                    tracing::info!(endpoint = %endpoint, "binance ws connected");
                    backoff.reset();
                    primary_failures = 0;
                    stream
                }
                Err(error) => {
                    tracing::warn!(?error, endpoint = %endpoint, "binance ws connect failed");
                    if endpoint_index == 0 {
                        primary_failures += 1;
                        if primary_failures >= 10 && endpoints.len() > 1 {
                            endpoint_index = 1;
                            tracing::warn!(
                                "binance ws primary failed 10 times; switching to fallback endpoint"
                            );
                        }
                    } else {
                        endpoint_index = 0;
                    }
                    let delay = backoff.next_delay()?;
                    tokio::time::sleep(delay).await;
                    continue;
                }
            };

            let (mut writer, mut reader) = ws_stream.split();
            let payload = subscribe_payload(&self.config.symbols)?;
            if let Err(error) = writer.send(Message::Text(payload)).await {
                tracing::warn!(?error, endpoint = %endpoint, "binance ws subscribe failed");
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
                                if let Some(event) = parse_event(&text)? {
                                    let asset = event.symbol().to_string();
                                    let asset_key = canonical_asset(&asset);
                                    let state = states
                                        .entry(asset.clone())
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
                                    }
                                    if let Some(update) = state.apply_event(event)? {
                                        let _ = sender.send(MarketUpdate::Binance(update));
                                    }
                                }
                            }
                            Some(Ok(Message::Ping(payload))) => {
                                if let Err(error) = writer.send(Message::Pong(payload)).await {
                                    tracing::warn!(?error, "binance ws pong failed");
                                    break;
                                }
                            }
                            Some(Ok(Message::Close(frame))) => {
                                tracing::warn!(?frame, endpoint = %endpoint, "binance ws closed");
                                break;
                            }
                            Some(Ok(_)) => {}
                            Some(Err(error)) => {
                                tracing::warn!(?error, endpoint = %endpoint, "binance ws error");
                                break;
                            }
                            None => {
                                tracing::warn!(endpoint = %endpoint, "binance ws stream ended");
                                break;
                            }
                        }
                    }
                }
            }

            if endpoint_index != 0 {
                endpoint_index = 0;
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
        let exp = self.base_ms.saturating_mul(2u64.saturating_pow(self.attempts.min(6)));
        let capped = exp.min(self.max_ms);
        let jitter = compute_jitter(1_000)?;
        Ok(Duration::from_millis(capped).saturating_add(jitter))
    }
}

fn build_ws_endpoints(primary: &str) -> Vec<String> {
    let mut endpoints = Vec::new();
    let primary_trim = primary.trim().trim_end_matches('/');
    if !primary_trim.is_empty() {
        endpoints.push(primary_trim.to_string());
    }
    for fallback in [
        "wss://stream.binance.com:443/ws",
        "wss://data-stream.binance.vision/ws",
    ] {
        if !endpoints.iter().any(|entry| entry == fallback) {
            endpoints.push(fallback.to_string());
        }
    }
    endpoints
}

#[derive(Debug)]
enum BinanceEvent {
    AggTrade {
        symbol: String,
        price: f64,
        event_time_ms: u64,
    },
    BookTicker {
        symbol: String,
        best_bid: f64,
        best_ask: f64,
        event_time_ms: u64,
    },
}

impl BinanceEvent {
    fn symbol(&self) -> &str {
        match self {
            Self::AggTrade { symbol, .. } => symbol,
            Self::BookTicker { symbol, .. } => symbol,
        }
    }
}

struct AssetState {
    last_price: Option<f64>,
    best_bid: Option<f64>,
    best_ask: Option<f64>,
    volatility: RollingVolatility,
    last_volatility: Option<f64>,
    candle_start_ms: Option<u64>,
    candle_open_price: Option<f64>,
    candle_interval_ms: u64,
    anchor_start_ms: Option<u64>,
    recorded_start_ms: Option<u64>,
}

impl AssetState {
    fn new(candle_interval: Duration) -> Self {
        Self {
            last_price: None,
            best_bid: None,
            best_ask: None,
            volatility: RollingVolatility::new(candle_interval),
            last_volatility: None,
            candle_start_ms: None,
            candle_open_price: None,
            candle_interval_ms: candle_interval.as_millis() as u64,
            anchor_start_ms: None,
            recorded_start_ms: None,
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

    fn apply_event(&mut self, event: BinanceEvent) -> Result<Option<BinanceMarketUpdate>> {
        match event {
            BinanceEvent::AggTrade {
                symbol,
                price,
                event_time_ms,
            } => {
                self.update_candle(event_time_ms, price);
                self.last_price = Some(price);
                if let Some(vol) = self.volatility.update(price, event_time_ms) {
                    self.last_volatility = Some(vol);
                }
                let dfo = self.candle_open_price.map(|open| (price - open) / open);

                Ok(Some(BinanceMarketUpdate {
                    asset: symbol,
                    best_bid: self.best_bid,
                    best_ask: self.best_ask,
                    last_price: Some(price),
                    volatility_1m: self.last_volatility,
                    dfo,
                    event_time_ms,
                }))
            }
            BinanceEvent::BookTicker {
                symbol,
                best_bid,
                best_ask,
                event_time_ms,
            } => {
                self.best_bid = Some(best_bid);
                self.best_ask = Some(best_ask);

                Ok(Some(BinanceMarketUpdate {
                    asset: symbol,
                    best_bid: self.best_bid,
                    best_ask: self.best_ask,
                    last_price: self.last_price,
                    volatility_1m: self.last_volatility,
                    dfo: self
                        .last_price
                        .and_then(|price| self.candle_open_price.map(|open| (price - open) / open)),
                    event_time_ms,
                }))
            }
        }
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
    upper
        .strip_suffix("USDT")
        .map(|value| value.to_string())
        .unwrap_or(upper.to_string())
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
    let params: Vec<String> = symbols
        .iter()
        .map(|symbol| symbol.to_ascii_lowercase())
        .flat_map(|symbol| vec![format!("{symbol}@aggTrade"), format!("{symbol}@bookTicker")])
        .collect();

    if params.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "binance oracle requires at least one symbol".to_string(),
        ));
    }

    let payload = serde_json::json!({
        "method": "SUBSCRIBE",
        "params": params,
        "id": STREAM_ID
    });
    Ok(payload.to_string())
}

fn parse_event(text: &str) -> Result<Option<BinanceEvent>> {
    let raw: Value = serde_json::from_str(text)?;
    let payload = match raw.get("data") {
        Some(data) => data,
        None => &raw,
    };
    let event_type = payload.get("e").and_then(|value| value.as_str());
    match event_type {
        Some("aggTrade") => parse_agg_trade(payload).map(Some),
        Some("bookTicker") => parse_book_ticker(payload).map(Some),
        _ => Ok(None),
    }
}

fn parse_agg_trade(payload: &Value) -> Result<BinanceEvent> {
    let symbol = payload
        .get("s")
        .and_then(|value| value.as_str())
        .ok_or_else(|| BankaiError::InvalidArgument("aggTrade missing symbol".to_string()))?;
    let price_str = payload
        .get("p")
        .and_then(|value| value.as_str())
        .ok_or_else(|| BankaiError::InvalidArgument("aggTrade missing price".to_string()))?;
    let event_time_ms = payload
        .get("T")
        .or_else(|| payload.get("E"))
        .and_then(|value| value.as_u64())
        .ok_or_else(|| BankaiError::InvalidArgument("aggTrade missing timestamp".to_string()))?;
    let price = price_str
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("aggTrade price not a float".to_string()))?;

    Ok(BinanceEvent::AggTrade {
        symbol: symbol.to_string(),
        price,
        event_time_ms,
    })
}

fn parse_book_ticker(payload: &Value) -> Result<BinanceEvent> {
    let symbol = payload
        .get("s")
        .and_then(|value| value.as_str())
        .ok_or_else(|| BankaiError::InvalidArgument("bookTicker missing symbol".to_string()))?;
    let best_bid_str = payload
        .get("b")
        .and_then(|value| value.as_str())
        .ok_or_else(|| BankaiError::InvalidArgument("bookTicker missing bid".to_string()))?;
    let best_ask_str = payload
        .get("a")
        .and_then(|value| value.as_str())
        .ok_or_else(|| BankaiError::InvalidArgument("bookTicker missing ask".to_string()))?;
    let event_time_ms = payload
        .get("E")
        .and_then(|value| value.as_u64())
        .ok_or_else(|| BankaiError::InvalidArgument("bookTicker missing timestamp".to_string()))?;

    let best_bid = best_bid_str
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("bookTicker bid not a float".to_string()))?;
    let best_ask = best_ask_str
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("bookTicker ask not a float".to_string()))?;

    Ok(BinanceEvent::BookTicker {
        symbol: symbol.to_string(),
        best_bid,
        best_ask,
        event_time_ms,
    })
}
