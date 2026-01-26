/**
 * @description
 * Redis-backed order book maintenance for Polymarket RTDS updates.
 *
 * @dependencies
 * - redis: async commands for ZSET/HASH writes
 *
 * @notes
 * - ZSETs store price ordering, depth hash stores aggregate size per price level.
 * - Levels are removed when size <= 0 to keep Redis state minimal.
 */
use redis::AsyncCommands;

use crate::error::{BankaiError, Result};
use crate::storage::redis::RedisManager;

#[derive(Debug, Clone, Copy)]
pub enum BookSide {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct OrderBookLevel {
    pub price: String,
    pub size: f64,
}

#[derive(Debug, Clone)]
pub struct VwapQuote {
    pub avg_price: f64,
    pub filled_size: f64,
    pub notional: f64,
}

#[derive(Clone, Debug)]
pub struct OrderBookStore {
    redis: RedisManager,
}

impl OrderBookStore {
    pub fn new(redis: RedisManager) -> Self {
        Self { redis }
    }

    pub async fn load_polymarket_asset_ids(&self) -> Result<Vec<String>> {
        self.redis.get_polymarket_asset_ids().await
    }

    pub async fn reset_book(&self, token_id: &str) -> Result<()> {
        let bids_key = bids_key(token_id);
        let asks_key = asks_key(token_id);
        let depth_key = depth_key(token_id);
        let mut conn = self.redis.connection();
        conn.del::<_, ()>(vec![bids_key, asks_key, depth_key])
            .await?;
        Ok(())
    }

    pub async fn apply_snapshot(
        &self,
        token_id: &str,
        bids: &[OrderBookLevel],
        asks: &[OrderBookLevel],
    ) -> Result<()> {
        self.reset_book(token_id).await?;

        for level in bids {
            self.apply_level(token_id, BookSide::Bid, &level.price, level.size)
                .await?;
        }
        for level in asks {
            self.apply_level(token_id, BookSide::Ask, &level.price, level.size)
                .await?;
        }
        Ok(())
    }

    pub async fn apply_level(
        &self,
        token_id: &str,
        side: BookSide,
        price: &str,
        size: f64,
    ) -> Result<()> {
        let score = parse_price_score(price)?;
        let zset_key = match side {
            BookSide::Bid => bids_key(token_id),
            BookSide::Ask => asks_key(token_id),
        };
        let depth_key = depth_key(token_id);

        if size <= 0.0 {
            self.redis.zrem(&zset_key, price).await?;
            self.redis.hdel(&depth_key, price).await?;
            return Ok(());
        }

        self.redis.zadd(&zset_key, score, price).await?;
        self.redis.hset_float(&depth_key, price, size).await?;
        Ok(())
    }

    pub async fn best_level(
        &self,
        token_id: &str,
        side: BookSide,
    ) -> Result<Option<OrderBookLevel>> {
        let mut levels = self.top_levels(token_id, side, 1).await?;
        Ok(levels.pop())
    }

    pub async fn top_levels(
        &self,
        token_id: &str,
        side: BookSide,
        limit: usize,
    ) -> Result<Vec<OrderBookLevel>> {
        if limit == 0 {
            return Ok(Vec::new());
        }
        let key = match side {
            BookSide::Bid => bids_key(token_id),
            BookSide::Ask => asks_key(token_id),
        };
        let depth_key = depth_key(token_id);
        let entries = match side {
            BookSide::Bid => {
                self.redis
                    .zrevrange_with_scores(&key, 0, (limit - 1) as isize)
                    .await?
            }
            BookSide::Ask => {
                self.redis
                    .zrange_with_scores(&key, 0, (limit - 1) as isize)
                    .await?
            }
        };
        let mut levels = Vec::new();
        for (price, _) in entries {
            if let Some(size) = self.redis.hget_float(&depth_key, &price).await? {
                if size > 0.0 {
                    levels.push(OrderBookLevel { price, size });
                }
            }
        }
        Ok(levels)
    }

    pub async fn mid_price(&self, token_id: &str) -> Result<Option<f64>> {
        let bid = self.best_level(token_id, BookSide::Bid).await?;
        let ask = self.best_level(token_id, BookSide::Ask).await?;
        match (bid, ask) {
            (Some(bid), Some(ask)) => {
                let bid = bid.price.parse::<f64>().ok();
                let ask = ask.price.parse::<f64>().ok();
                match (bid, ask) {
                    (Some(bid), Some(ask)) if bid > 0.0 && ask > 0.0 => {
                        Ok(Some((bid + ask) / 2.0))
                    }
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }

    pub async fn vwap_for_size(
        &self,
        token_id: &str,
        side: BookSide,
        size: f64,
        level_limit: usize,
    ) -> Result<Option<VwapQuote>> {
        if size <= 0.0 {
            return Ok(None);
        }
        let levels = self.top_levels(token_id, side, level_limit).await?;
        if levels.is_empty() {
            return Ok(None);
        }
        let mut remaining = size;
        let mut notional = 0.0;
        let mut filled = 0.0;
        for level in levels {
            if remaining <= 0.0 {
                break;
            }
            let price = match level.price.parse::<f64>() {
                Ok(value) if value > 0.0 => value,
                _ => continue,
            };
            let take = remaining.min(level.size);
            if take <= 0.0 {
                continue;
            }
            notional += take * price;
            filled += take;
            remaining -= take;
        }
        if filled <= 0.0 {
            return Ok(None);
        }
        let avg_price = notional / filled;
        if filled + 1e-9 < size {
            return Ok(None);
        }
        Ok(Some(VwapQuote {
            avg_price,
            filled_size: filled,
            notional,
        }))
    }
}

fn parse_price_score(price: &str) -> Result<f64> {
    let trimmed = price.trim();
    if trimmed.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "order book price missing".to_string(),
        ));
    }
    trimmed
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("order book price not numeric".to_string()))
}

fn bids_key(token_id: &str) -> String {
    format!("book:{token_id}:bids")
}

fn asks_key(token_id: &str) -> String {
    format!("book:{token_id}:asks")
}

fn depth_key(token_id: &str) -> String {
    format!("book:{token_id}:depth")
}
