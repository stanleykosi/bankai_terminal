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
use std::time::Duration;
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

use crate::engine::types::{BinanceMarketUpdate, MarketUpdate};
use crate::error::{BankaiError, Result};

const STREAM_ID: u64 = 1;

#[derive(Debug, Clone)]
pub struct BinanceOracleConfig {
    pub endpoint: String,
    pub symbols: Vec<String>,
    pub candle_interval: Duration,
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
        let (ws_stream, _) = tokio_tungstenite::connect_async(&self.config.endpoint).await?;
        let (mut writer, mut reader) = ws_stream.split();
        let payload = subscribe_payload(&self.config.symbols)?;
        writer.send(Message::Text(payload)).await?;

        let mut states = HashMap::new();

        while let Some(message) = reader.next().await {
            let message = message?;
            match message {
                Message::Text(text) => {
                    if let Some(event) = parse_event(&text)? {
                        let asset = event.symbol().to_string();
                        let state = states
                            .entry(asset.clone())
                            .or_insert_with(|| AssetState::new(self.config.candle_interval));
                        if let Some(update) = state.apply_event(event)? {
                            let _ = sender.send(MarketUpdate::Binance(update));
                        }
                    }
                }
                Message::Ping(payload) => {
                    writer.send(Message::Pong(payload)).await?;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }

        Ok(())
    }
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
}

impl AssetState {
    fn new(candle_interval: Duration) -> Self {
        Self {
            last_price: None,
            best_bid: None,
            best_ask: None,
            volatility: RollingVolatility::new(Duration::from_secs(60)),
            last_volatility: None,
            candle_start_ms: None,
            candle_open_price: None,
            candle_interval_ms: candle_interval.as_millis() as u64,
        }
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
                let dfo = self
                    .candle_open_price
                    .map(|open| (price - open) / open);

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
        let candle_start = timestamp_ms - (timestamp_ms % self.candle_interval_ms);
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
        .flat_map(|symbol| {
            vec![
                format!("{symbol}@aggTrade"),
                format!("{symbol}@bookTicker"),
            ]
        })
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
