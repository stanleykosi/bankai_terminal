/**
 * @description
 * Polymarket user websocket listener for trade/order updates.
 *
 * @dependencies
 * - tokio-tungstenite: websocket client
 * - serde_json: message parsing
 *
 * @notes
 * - Uses user channel with auth to reconcile fills in near real time.
 */
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::Duration;
use tokio_tungstenite::tungstenite::Message;

use crate::accounting::keys::REALIZED_PNL_KEY;
use crate::accounting::trade_events::{is_seen_trade, mark_seen_trade, record_realized_pnl_event};
use crate::error::Result;
use crate::storage::redis::RedisManager;
use crate::telemetry::metrics;

#[derive(Debug, Clone)]
pub struct PolymarketUserWsConfig {
    pub ws_endpoint: String,
    pub ping_interval: Duration,
    pub reconnect_delay: Duration,
    pub auth: PolymarketUserAuth,
    pub markets: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PolymarketUserAuth {
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: String,
}

pub struct PolymarketUserWs {
    config: PolymarketUserWsConfig,
    redis: RedisManager,
    wallet_key: String,
}

impl PolymarketUserWs {
    pub fn new(config: PolymarketUserWsConfig, redis: RedisManager, wallet_key: String) -> Self {
        Self {
            config,
            redis,
            wallet_key,
        }
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "polymarket user ws stopped");
            }
        })
    }

    async fn run(self) -> Result<()> {
        loop {
            match self.connect_and_stream().await {
                Ok(_) => {}
                Err(error) => tracing::warn!(?error, "polymarket user ws error"),
            }
            tokio::time::sleep(self.config.reconnect_delay).await;
        }
    }

    async fn connect_and_stream(&self) -> Result<()> {
        let (ws_stream, _) = tokio_tungstenite::connect_async(&self.config.ws_endpoint).await?;
        let (mut writer, mut reader) = ws_stream.split();
        let payload = json!({
            "type": "user",
            "markets": self.config.markets,
            "auth": {
                "apiKey": self.config.auth.api_key,
                "secret": self.config.auth.api_secret,
                "passphrase": self.config.auth.api_passphrase,
            }
        });
        writer.send(Message::Text(payload.to_string())).await?;

        let mut ping_interval = tokio::time::interval(self.config.ping_interval);
        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    writer.send(Message::Text("PING".to_string())).await?;
                }
                message = reader.next() => {
                    match message {
                        Some(Ok(Message::Text(text))) => {
                            if let Err(error) = self.handle_message(&text).await {
                                tracing::warn!(?error, "failed to handle user ws message");
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
        Ok(())
    }

    async fn handle_message(&self, text: &str) -> Result<()> {
        let value: serde_json::Value = serde_json::from_str(text)?;
        let Some(event_type) = value.get("event_type").and_then(|v| v.as_str()) else {
            return Ok(());
        };
        match event_type {
            "trade" => {
                let trade: TradeEvent = serde_json::from_value(value)?;
                self.handle_trade(trade).await?;
            }
            "order" => {
                let order: OrderEvent = serde_json::from_value(value)?;
                self.handle_order(order).await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_trade(&self, trade: TradeEvent) -> Result<()> {
        let status = trade.status.to_ascii_uppercase();
        if status == "FAILED" {
            self.log_activity(format!(
                "[ALERT] Trade failed id={} asset={}",
                trade.id, trade.asset_id
            ))
            .await;
            return Ok(());
        }
        if status != "CONFIRMED" {
            return Ok(());
        }
        if is_seen_trade(&self.redis, &self.wallet_key, &trade.id).await? {
            return Ok(());
        }
        mark_seen_trade(&self.redis, &self.wallet_key, &trade.id).await?;

        let size = trade.size.parse::<f64>().unwrap_or(0.0);
        let price = trade.price.parse::<f64>().unwrap_or(0.0);
        if size <= 0.0 || price <= 0.0 {
            return Ok(());
        }
        let side = trade.side.to_ascii_lowercase();
        match side.as_str() {
            "buy" => {
                self.apply_buy(&trade.asset_id, size, price).await?;
            }
            "sell" => {
                let timestamp = trade.timestamp().unwrap_or_else(current_unix_timestamp);
                self.apply_sell(&trade.asset_id, size, price, &trade.id, timestamp)
                    .await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn handle_order(&self, order: OrderEvent) -> Result<()> {
        let original = order.original_size.parse::<f64>().unwrap_or(0.0);
        let matched = order.size_matched.parse::<f64>().unwrap_or(0.0);
        if original > 0.0 {
            let rate = (matched / original).clamp(0.0, 1.0) * 100.0;
            metrics::record_order_fill_rate(rate);
        }
        let status = if order.order_type.to_ascii_uppercase() == "CANCELLATION" && matched <= 0.0 {
            "CANCELLED"
        } else if original > 0.0 && matched >= original {
            "FILLED"
        } else if matched > 0.0 {
            "PARTIAL"
        } else {
            "OPEN"
        };
        if status == "CANCELLED" {
            self.log_activity(format!(
                "[ALERT] Order cancelled asset={} id={}",
                order.asset_id, order.id
            ))
            .await;
        }
        let payload = OrderState {
            id: order.id.clone(),
            asset_id: order.asset_id.clone(),
            status: status.to_string(),
            original_size: original,
            matched_size: matched,
            order_type: order.order_type.clone(),
        };
        if let Ok(json) = serde_json::to_string(&payload) {
            let now = current_unix_timestamp() * 1000;
            let _ = self
                .redis
                .set_order_state(&self.wallet_key, &order.id, &json, now)
                .await;
        }
        let open_key = open_orders_key(&self.wallet_key);
        match status {
            "OPEN" | "PARTIAL" => {
                let _ = self.redis.sadd(&open_key, &order.id).await;
            }
            "FILLED" | "CANCELLED" => {
                let _ = self.redis.srem(&open_key, &order.id).await;
            }
            _ => {}
        }
        Ok(())
    }

    async fn apply_buy(&self, asset_id: &str, size: f64, price: f64) -> Result<()> {
        let current = self
            .redis
            .get_tracked_position(&self.wallet_key, asset_id)
            .await?;
        let entry = self
            .redis
            .get_entry_price(&self.wallet_key, asset_id)
            .await?
            .unwrap_or(0.0);
        let new_balance = current + size;
        let weighted = if current > 0.0 && entry > 0.0 {
            ((entry * current) + (price * size)) / new_balance
        } else {
            price
        };
        self.redis
            .set_tracked_position(&self.wallet_key, asset_id, new_balance)
            .await?;
        self.redis
            .set_entry_price(&self.wallet_key, asset_id, weighted)
            .await?;
        let peak = self
            .redis
            .get_peak_price(&self.wallet_key, asset_id)
            .await?
            .unwrap_or(weighted);
        if price > peak {
            self.redis
                .set_peak_price(&self.wallet_key, asset_id, price)
                .await?;
        }
        Ok(())
    }

    async fn apply_sell(
        &self,
        asset_id: &str,
        size: f64,
        price: f64,
        trade_id: &str,
        timestamp: u64,
    ) -> Result<()> {
        let current = self
            .redis
            .get_tracked_position(&self.wallet_key, asset_id)
            .await?;
        if current <= 0.0 {
            return Ok(());
        }
        let entry = self
            .redis
            .get_entry_price(&self.wallet_key, asset_id)
            .await?
            .unwrap_or(0.0);
        let new_balance = (current - size).max(0.0);
        self.redis
            .set_tracked_position(&self.wallet_key, asset_id, new_balance)
            .await?;
        if new_balance <= 0.0 {
            self.redis
                .set_entry_price(&self.wallet_key, asset_id, 0.0)
                .await?;
            self.redis
                .set_peak_price(&self.wallet_key, asset_id, 0.0)
                .await?;
        }
        if entry > 0.0 {
            let realized = (price - entry) * size;
            let _ = self.redis.incr_float(REALIZED_PNL_KEY, realized).await?;
            let _ = record_realized_pnl_event(
                &self.redis,
                &self.wallet_key,
                trade_id,
                timestamp,
                realized,
            )
            .await;
        }
        Ok(())
    }

    async fn log_activity(&self, message: String) {
        let _ = self.redis.push_activity_log(&message, 8).await;
    }
}

#[derive(Debug, Deserialize)]
struct TradeEvent {
    pub asset_id: String,
    pub id: String,
    pub price: String,
    pub side: String,
    pub size: String,
    pub status: String,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub last_update: Option<String>,
    #[serde(default)]
    pub matchtime: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OrderEvent {
    pub asset_id: String,
    pub id: String,
    pub original_size: String,
    pub size_matched: String,
    #[serde(rename = "type")]
    pub order_type: String,
}

fn current_unix_timestamp() -> u64 {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    duration.as_secs()
}

impl TradeEvent {
    fn timestamp(&self) -> Option<u64> {
        if let Some(value) = self.timestamp.as_deref() {
            if let Some(parsed) = parse_timestamp(value) {
                return Some(parsed);
            }
        }
        if let Some(value) = self.last_update.as_deref() {
            if let Some(parsed) = parse_timestamp(value) {
                return Some(parsed);
            }
        }
        if let Some(value) = self.matchtime.as_deref() {
            return parse_timestamp(value);
        }
        None
    }
}

fn parse_timestamp(value: &str) -> Option<u64> {
    if let Ok(parsed) = value.parse::<u64>() {
        return Some(parsed);
    }
    if let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(value) {
        return Some(parsed.timestamp() as u64);
    }
    None
}

#[derive(Debug, Serialize)]
struct OrderState {
    id: String,
    asset_id: String,
    status: String,
    original_size: f64,
    matched_size: f64,
    order_type: String,
}

fn open_orders_key(address: &str) -> String {
    format!("orders:open:{address}")
}
