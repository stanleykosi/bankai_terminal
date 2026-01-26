/**
 * @purpose
 * Compute unrealized PnL from tracked positions and live order books.
 *
 * @dependencies
 * - redis: tracked positions + entry prices
 * - orderbook: mid prices for assets
 *
 * @notes
 * - Writes sys:pnl:unrealized and sys:pnl:24h for UI display.
 */
use std::collections::HashMap;
use std::time::Duration;

use crate::accounting::keys::{PNL_24H_KEY, REALIZED_PNL_24H_KEY, UNREALIZED_PNL_KEY};
use crate::error::Result;
use crate::storage::orderbook::OrderBookStore;
use crate::storage::redis::RedisManager;

pub struct PnlMonitor {
    redis: RedisManager,
    orderbook: OrderBookStore,
    wallet_key: String,
    interval: Duration,
}

impl PnlMonitor {
    pub fn new(
        redis: RedisManager,
        orderbook: OrderBookStore,
        wallet_key: String,
        interval: Duration,
    ) -> Self {
        Self {
            redis,
            orderbook,
            wallet_key,
            interval,
        }
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "pnl monitor stopped");
            }
        })
    }

    async fn run(self) -> Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        loop {
            interval.tick().await;
            if let Err(error) = self.refresh().await {
                tracing::warn!(?error, "pnl refresh failed");
            }
        }
    }

    async fn refresh(&self) -> Result<()> {
        let positions = self.tracked_positions().await?;
        let entries = self.entry_prices().await?;
        let mut unrealized = 0.0;

        for (asset_id, size) in positions {
            if size <= 0.0 {
                continue;
            }
            let entry = entries.get(&asset_id).copied().unwrap_or(0.0);
            if entry <= 0.0 {
                continue;
            }
            let Some(mid) = self.orderbook.mid_price(&asset_id).await? else {
                continue;
            };
            unrealized += (mid - entry) * size;
        }

        let _ = self.redis.set_float(UNREALIZED_PNL_KEY, unrealized).await;
        let realized_24h = self
            .redis
            .get_float(REALIZED_PNL_24H_KEY)
            .await?
            .unwrap_or(0.0);
        let _ = self
            .redis
            .set_float(PNL_24H_KEY, realized_24h + unrealized)
            .await;
        Ok(())
    }

    async fn tracked_positions(&self) -> Result<HashMap<String, f64>> {
        let key = format!("positions:tracked:{}", self.wallet_key);
        self.redis.hgetall_f64(&key).await
    }

    async fn entry_prices(&self) -> Result<HashMap<String, f64>> {
        let key = format!("positions:entry:{}", self.wallet_key);
        self.redis.hgetall_f64(&key).await
    }
}
