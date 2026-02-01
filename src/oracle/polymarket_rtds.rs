/**
 * @description
 * Polymarket RTDS order book ingestion with snapshot + price_change updates.
 *
 * @dependencies
 * - reqwest: REST snapshot fetches
 * - tokio-tungstenite: websocket streaming
 * - serde_json: message parsing
 *
 * @notes
 * - Seeds Redis with REST snapshots before streaming price_change updates.
 * - Uses MARKET channel subscription for asset IDs (token IDs).
 */
use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio_tungstenite::tungstenite::Message;

use crate::error::{BankaiError, Result};
use crate::storage::orderbook::{BookSide, OrderBookLevel, OrderBookStore};

const DEFAULT_PING_INTERVAL: Duration = Duration::from_secs(10);
const DEFAULT_RECONNECT_DELAY: Duration = Duration::from_secs(3);
const DEFAULT_SNAPSHOT_TIMEOUT: Duration = Duration::from_secs(10);
const DEFAULT_STALE_TIMEOUT: Duration = Duration::from_secs(60);
const DEFAULT_ASSET_STALE_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug, Clone)]
pub struct PolymarketAuth {
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: String,
}

#[derive(Debug, Clone)]
pub struct PolymarketRtdsConfig {
    pub ws_endpoint: String,
    pub rest_endpoint: String,
    pub asset_ids: Vec<String>,
    pub asset_refresh_interval: Duration,
    pub asset_stale_timeout: Duration,
    pub ping_interval: Duration,
    pub reconnect_delay: Duration,
    pub snapshot_timeout: Duration,
    pub auth: Option<PolymarketAuth>,
}

impl PolymarketRtdsConfig {
    pub fn new(ws_endpoint: String, rest_endpoint: String, asset_ids: Vec<String>) -> Self {
        Self {
            ws_endpoint,
            rest_endpoint,
            asset_ids,
            asset_refresh_interval: Duration::from_secs(5),
            asset_stale_timeout: DEFAULT_ASSET_STALE_TIMEOUT,
            ping_interval: DEFAULT_PING_INTERVAL,
            reconnect_delay: DEFAULT_RECONNECT_DELAY,
            snapshot_timeout: DEFAULT_SNAPSHOT_TIMEOUT,
            auth: None,
        }
    }
}

pub struct PolymarketRtds {
    config: PolymarketRtdsConfig,
    client: Client,
    orderbook: OrderBookStore,
}

impl PolymarketRtds {
    pub fn new(config: PolymarketRtdsConfig, orderbook: OrderBookStore) -> Result<Self> {
        let client = Client::builder().timeout(config.snapshot_timeout).build()?;
        Ok(Self {
            config,
            client,
            orderbook,
        })
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "polymarket rtds stopped");
            }
        })
    }

    async fn run(self) -> Result<()> {
        loop {
            let asset_ids = self.resolve_asset_ids().await?;
            if asset_ids.is_empty() {
                tracing::warn!("polymarket rtds has no asset ids; retrying");
                tokio::time::sleep(self.config.reconnect_delay).await;
                continue;
            }

            if let Err(error) = self.seed_snapshots(&asset_ids).await {
                tracing::warn!(?error, "polymarket snapshot seed failed");
            }

            match self.connect_and_stream(&asset_ids).await {
                Ok(StreamOutcome::Resubscribe) => {
                    continue;
                }
                Ok(StreamOutcome::Backoff) => {}
                Err(error) => {
                    tracing::warn!(?error, "polymarket rtds stream error");
                }
            }

            tokio::time::sleep(self.config.reconnect_delay).await;
        }
    }

    async fn resolve_asset_ids(&self) -> Result<Vec<String>> {
        let mut merged: HashSet<String> = HashSet::new();
        for id in &self.config.asset_ids {
            merged.insert(id.clone());
        }
        let dynamic = self.orderbook.load_polymarket_asset_ids().await?;
        for id in dynamic {
            merged.insert(id);
        }
        Ok(merged.into_iter().collect())
    }

    async fn seed_snapshots(&self, asset_ids: &[String]) -> Result<()> {
        for asset_id in asset_ids {
            match self.fetch_snapshot(asset_id).await {
                Ok(snapshot) => {
                    self.orderbook
                        .apply_snapshot(asset_id, &snapshot.bids, &snapshot.asks)
                        .await?;
                }
                Err(error) => {
                    tracing::warn!(
                        ?error,
                        asset_id = %asset_id,
                        "failed to fetch polymarket snapshot"
                    );
                }
            }
        }
        Ok(())
    }

    async fn fetch_snapshot(&self, asset_id: &str) -> Result<OrderBookSnapshot> {
        let url = format!(
            "{}/book?token_id={}",
            self.config.rest_endpoint.trim_end_matches('/'),
            asset_id
        );
        let response = self.client.get(url).send().await?.error_for_status()?;
        let parsed: Value = response.json().await?;
        parse_snapshot(&parsed)
    }

    async fn connect_and_stream(&self, asset_ids: &[String]) -> Result<StreamOutcome> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(&self.config.ws_endpoint).await?;
        let (mut writer, mut reader) = ws_stream.split();
        let payload = build_subscription_payload(asset_ids, self.config.auth.as_ref())?;
        writer.send(Message::Text(payload)).await?;

        let mut ping_interval = tokio::time::interval(self.config.ping_interval);
        let mut refresh_interval = tokio::time::interval(self.config.asset_refresh_interval);
        let stale_timeout = DEFAULT_STALE_TIMEOUT;
        let asset_stale_timeout = self.config.asset_stale_timeout;
        let mut last_event = tokio::time::Instant::now();
        let mut current_assets = asset_ids.to_vec();
        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    if last_event.elapsed() > stale_timeout {
                        tracing::warn!("polymarket rtds stale; reconnecting");
                        return Ok(StreamOutcome::Backoff);
                    }
                    writer.send(Message::Text("PING".to_string())).await?;
                }
                _ = refresh_interval.tick() => {
                    let latest = self.resolve_asset_ids().await?;
                    if !latest.is_empty() && asset_ids_changed(&current_assets, &latest) {
                        tracing::info!("polymarket asset ids updated; resubscribing");
                        return Ok(StreamOutcome::Resubscribe);
                    }
                    current_assets = latest;
                    let now_ms = now_ms().unwrap_or(0);
                    for asset_id in &current_assets {
                        let last_update = self.orderbook.last_update_ms(asset_id).await?;
                        let age_ms = match last_update {
                            Some(ts) => now_ms.saturating_sub(ts),
                            None => u64::MAX,
                        };
                        if age_ms > asset_stale_timeout.as_millis() as u64 {
                            tracing::warn!(
                                asset_id = %asset_id,
                                age_ms,
                                "polymarket rtds asset stale; resubscribing"
                            );
                            return Ok(StreamOutcome::Resubscribe);
                        }
                    }
                }
                message = reader.next() => {
                    match message {
                        Some(Ok(Message::Text(text))) => {
                            match self.handle_message(&text).await {
                                Ok(true) => {
                                    last_event = tokio::time::Instant::now();
                                }
                                Ok(false) => {}
                                Err(error) => {
                                    tracing::warn!(?error, "failed to handle polymarket rtds message");
                                }
                            }
                        }
                        Some(Ok(Message::Ping(payload))) => {
                            writer.send(Message::Pong(payload)).await?;
                        }
                        Some(Ok(Message::Close(_))) => break,
                        Some(Ok(_)) => {}
                        Some(Err(error)) => return Err(error.into()),
                        None => break,
                    }
                }
            }
        }

        Ok(StreamOutcome::Backoff)
    }

    async fn handle_message(&self, text: &str) -> Result<bool> {
        let mut updated = false;
        if let Some(changes) = parse_price_change_event(text)? {
            for change in changes {
                self.orderbook
                    .apply_level(&change.asset_id, change.side, &change.price, change.size)
                    .await?;
            }
            updated = true;
        }
        if let Some(trade) = parse_last_trade_event(text)? {
            let _ = self
                .orderbook
                .set_last_trade_price(&trade.asset_id, trade.price, trade.timestamp_ms)
                .await;
            updated = true;
        }
        Ok(updated)
    }
}

#[derive(Debug)]
struct OrderBookSnapshot {
    bids: Vec<OrderBookLevel>,
    asks: Vec<OrderBookLevel>,
}

#[derive(Debug)]
struct PriceChange {
    asset_id: String,
    price: String,
    size: f64,
    side: BookSide,
}

#[derive(Debug)]
struct LastTrade {
    asset_id: String,
    price: f64,
    timestamp_ms: u64,
}

#[derive(Debug, Clone, Copy)]
enum StreamOutcome {
    Resubscribe,
    Backoff,
}

fn now_ms() -> Result<u64> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(duration.as_millis() as u64)
}

fn build_subscription_payload(
    asset_ids: &[String],
    auth: Option<&PolymarketAuth>,
) -> Result<String> {
    let mut payload = json!({
        "asset_ids": asset_ids,
        "type": "MARKET",
    });

    if let Some(auth) = auth {
        payload["auth"] = json!({
            "apiKey": auth.api_key,
            "secret": auth.api_secret,
            "passphrase": auth.api_passphrase,
        });
    }

    Ok(serde_json::to_string(&payload)?)
}

fn parse_snapshot(value: &Value) -> Result<OrderBookSnapshot> {
    let bids = parse_snapshot_levels(value.get("bids"), "snapshot bids")?;
    let asks = parse_snapshot_levels(value.get("asks"), "snapshot asks")?;
    Ok(OrderBookSnapshot { bids, asks })
}

fn parse_snapshot_levels(value: Option<&Value>, field: &str) -> Result<Vec<OrderBookLevel>> {
    let levels =
        value.ok_or_else(|| BankaiError::InvalidArgument(format!("snapshot missing {field}")))?;
    let entries = levels
        .as_array()
        .ok_or_else(|| BankaiError::InvalidArgument(format!("snapshot {field} not an array")))?;

    let mut parsed = Vec::with_capacity(entries.len());
    for entry in entries {
        let price = parse_string(entry.get("price"), "snapshot price")?;
        let size = parse_numeric(entry.get("size"), "snapshot size")?;
        parsed.push(OrderBookLevel { price, size });
    }
    Ok(parsed)
}

fn parse_price_change_event(text: &str) -> Result<Option<Vec<PriceChange>>> {
    let parsed: Value = serde_json::from_str(text)?;
    let event_source = if parsed.get("event_type").is_some() {
        &parsed
    } else if let Some(payload) = parsed.get("payload") {
        payload
    } else {
        return Ok(None);
    };

    let event_type = event_source
        .get("event_type")
        .and_then(|value| value.as_str());
    if event_type != Some("price_change") {
        return Ok(None);
    }

    let root_asset_id = event_source
        .get("asset_id")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());

    let changes_value = event_source
        .get("price_changes")
        .or_else(|| event_source.get("changes"))
        .ok_or_else(|| BankaiError::InvalidArgument("price_change missing changes".to_string()))?;
    let changes = changes_value.as_array().ok_or_else(|| {
        BankaiError::InvalidArgument("price_change changes not array".to_string())
    })?;

    let mut parsed_changes = Vec::with_capacity(changes.len());
    for change in changes {
        match parse_price_change(change, root_asset_id.as_deref()) {
            Ok(entry) => parsed_changes.push(entry),
            Err(error) => {
                tracing::warn!(?error, "skipping invalid price_change entry");
            }
        }
    }

    Ok(Some(parsed_changes))
}

fn parse_last_trade_event(text: &str) -> Result<Option<LastTrade>> {
    let parsed: Value = serde_json::from_str(text)?;
    let event_source = if parsed.get("event_type").is_some() {
        &parsed
    } else if let Some(payload) = parsed.get("payload") {
        payload
    } else {
        return Ok(None);
    };

    let event_type = event_source
        .get("event_type")
        .and_then(|value| value.as_str());
    if event_type != Some("last_trade_price") {
        return Ok(None);
    }
    let asset_id = event_source
        .get("asset_id")
        .and_then(|value| value.as_str())
        .ok_or_else(|| {
            BankaiError::InvalidArgument("last_trade_price asset_id missing".to_string())
        })?;
    let price = parse_string(event_source.get("price"), "last_trade_price price")?
        .parse::<f64>()
        .map_err(|_| {
            BankaiError::InvalidArgument("last_trade_price price not numeric".to_string())
        })?;
    let timestamp_ms = parse_numeric(event_source.get("timestamp"), "last_trade_price timestamp")?
        .round()
        .max(0.0) as u64;
    Ok(Some(LastTrade {
        asset_id: asset_id.to_string(),
        price,
        timestamp_ms,
    }))
}

fn parse_price_change(value: &Value, root_asset_id: Option<&str>) -> Result<PriceChange> {
    let asset_id = value
        .get("asset_id")
        .and_then(|value| value.as_str())
        .or(root_asset_id)
        .ok_or_else(|| BankaiError::InvalidArgument("price_change asset_id missing".to_string()))?;
    let price = parse_string(value.get("price"), "price_change price")?;
    let size = parse_numeric(value.get("size"), "price_change size")?;
    let side = parse_side(value.get("side"))?;

    Ok(PriceChange {
        asset_id: asset_id.to_string(),
        price,
        size,
        side,
    })
}

fn parse_side(value: Option<&Value>) -> Result<BookSide> {
    let raw = parse_string(value, "price_change side")?;
    match raw.to_ascii_uppercase().as_str() {
        "BUY" | "BID" => Ok(BookSide::Bid),
        "SELL" | "ASK" => Ok(BookSide::Ask),
        _ => Err(BankaiError::InvalidArgument(format!(
            "price_change side invalid: {raw}"
        ))),
    }
}

fn parse_numeric(value: Option<&Value>, field: &str) -> Result<f64> {
    let value = value.ok_or_else(|| BankaiError::InvalidArgument(format!("{field} missing")))?;
    if let Some(number) = value.as_f64() {
        return Ok(number);
    }
    if let Some(text) = value.as_str() {
        return text
            .parse::<f64>()
            .map_err(|_| BankaiError::InvalidArgument(format!("{field} not numeric")));
    }
    Err(BankaiError::InvalidArgument(format!("{field} invalid")))
}

fn parse_string(value: Option<&Value>, field: &str) -> Result<String> {
    let value = value.ok_or_else(|| BankaiError::InvalidArgument(format!("{field} missing")))?;
    if let Some(text) = value.as_str() {
        if text.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(format!("{field} empty")));
        }
        return Ok(text.trim().to_string());
    }
    Err(BankaiError::InvalidArgument(format!("{field} invalid")))
}

fn asset_ids_changed(current: &[String], latest: &[String]) -> bool {
    if current.len() != latest.len() {
        return true;
    }
    let mut current_sorted = current.to_vec();
    let mut latest_sorted = latest.to_vec();
    current_sorted.sort();
    latest_sorted.sort();
    current_sorted != latest_sorted
}
