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

#[derive(Clone)]
pub struct OrderBookStore {
    redis: RedisManager,
}

impl OrderBookStore {
    pub fn new(redis: RedisManager) -> Self {
        Self { redis }
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
}

fn parse_price_score(price: &str) -> Result<f64> {
    let trimmed = price.trim();
    if trimmed.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "order book price missing".to_string(),
        ));
    }
    trimmed.parse::<f64>().map_err(|_| {
        BankaiError::InvalidArgument("order book price not numeric".to_string())
    })
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
