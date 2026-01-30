/**
 * @description
 * Redis manager for hot state storage (order books, nonces, market metadata).
 *
 * @dependencies
 * - redis: async connection manager and command helpers
 *
 * @notes
 * - ConnectionManager handles reconnects and multiplexing.
 */
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::collections::HashMap;

use crate::engine::types::MarketWindow;
use crate::error::Result;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct RedisManager {
    connection: ConnectionManager,
}

impl std::fmt::Debug for RedisManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisManager")
            .field("connection", &"<redis::aio::ConnectionManager>")
            .finish()
    }
}

const POLYMARKET_ASSET_IDS_KEY: &str = "polymarket:asset_ids";
const TUI_ACTIVITY_LOG_KEY: &str = "tui:activity_log";
const TUI_INTENT_LOG_KEY: &str = "tui:intent_log";
const TUI_ORDER_LOG_KEY: &str = "tui:order_log";
const POSITIONS_TRACKED_PREFIX: &str = "positions:tracked:";
const POSITIONS_ENTRY_PREFIX: &str = "positions:entry:";
const POSITIONS_PEAK_PREFIX: &str = "positions:peak:";
const ASSET_WINDOW_NEXT_PREFIX: &str = "polymarket:window_next:";
const ASSET_WINDOW_CACHE_PREFIX: &str = "polymarket:windows:";
const FEE_RATE_PREFIX: &str = "polymarket:fee_rate:";
const ASSET_START_PRICE_PREFIX: &str = "polymarket:start_price:";
const ASSET_END_PRICE_PREFIX: &str = "polymarket:end_price:";
const CHAINLINK_STATUS_KEY: &str = "oracle:chainlink:status";
const ORDERBOOK_TS_PREFIX: &str = "polymarket:book_ts:";
const LAST_TRADE_PREFIX: &str = "polymarket:last_trade:";
const TOKEN_MARKET_PREFIX: &str = "polymarket:token_market:";

#[derive(Debug, Clone)]
pub struct AssetWindow {
    pub start_time_ms: u64,
    pub end_time_ms: u64,
    pub market_id: String,
    pub updated_at_ms: u64,
}

#[derive(Debug, Clone)]
pub struct OutcomeTokenIds {
    pub up: String,
    pub down: String,
}

#[derive(Debug, Clone)]
pub struct MarketMetadata {
    pub fee_rate_bps: Option<f64>,
    pub min_tick_size: Option<f64>,
    pub min_order_size: Option<f64>,
    pub start_time_ms: Option<u64>,
    pub end_time_ms: Option<u64>,
    pub outcome_up_token_id: Option<String>,
    pub outcome_down_token_id: Option<String>,
}

impl RedisManager {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let connection = ConnectionManager::new(client).await?;
        Ok(Self { connection })
    }

    pub fn connection(&self) -> ConnectionManager {
        self.connection.clone()
    }

    pub async fn set_float(&self, key: &str, value: f64) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.set::<_, _, ()>(key, value).await?;
        Ok(())
    }

    pub async fn get_float(&self, key: &str) -> Result<Option<f64>> {
        let mut conn = self.connection.clone();
        Ok(conn.get(key).await?)
    }

    pub async fn incr_float(&self, key: &str, delta: f64) -> Result<f64> {
        let mut conn = self.connection.clone();
        Ok(conn.incr(key, delta).await?)
    }

    pub async fn hset_float(&self, key: &str, field: &str, value: f64) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key, field, value).await?;
        Ok(())
    }

    pub async fn hget_float(&self, key: &str, field: &str) -> Result<Option<f64>> {
        let mut conn = self.connection.clone();
        Ok(conn.hget(key, field).await?)
    }

    pub async fn hget_string(&self, key: &str, field: &str) -> Result<Option<String>> {
        let mut conn = self.connection.clone();
        Ok(conn.hget(key, field).await?)
    }

    pub async fn hget_i64(&self, key: &str, field: &str) -> Result<Option<i64>> {
        let mut conn = self.connection.clone();
        Ok(conn.hget(key, field).await?)
    }

    pub async fn hdel(&self, key: &str, field: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.hdel::<_, _, ()>(key, field).await?;
        Ok(())
    }

    pub async fn zadd(&self, key: &str, score: f64, member: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.zadd::<_, _, _, ()>(key, member, score).await?;
        Ok(())
    }

    pub async fn zrem(&self, key: &str, member: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.zrem::<_, _, ()>(key, member).await?;
        Ok(())
    }

    pub async fn zrange_with_scores(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> Result<Vec<(String, f64)>> {
        let mut conn = self.connection.clone();
        Ok(conn.zrange_withscores(key, start, stop).await?)
    }

    pub async fn zrevrange_with_scores(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> Result<Vec<(String, f64)>> {
        let mut conn = self.connection.clone();
        Ok(conn.zrevrange_withscores(key, start, stop).await?)
    }

    pub async fn zrangebyscore(&self, key: &str, min: f64, max: f64) -> Result<Vec<String>> {
        let mut conn = self.connection.clone();
        Ok(conn.zrangebyscore(key, min, max).await?)
    }

    pub async fn zremrangebyscore(&self, key: &str, min: f64, max: f64) -> Result<usize> {
        let mut conn = self.connection.clone();
        let removed: i64 = redis::cmd("ZREMRANGEBYSCORE")
            .arg(key)
            .arg(min)
            .arg(max)
            .query_async(&mut conn)
            .await?;
        Ok(removed.max(0) as usize)
    }

    pub async fn set_market_metadata(
        &self,
        market_id: &str,
        fee_rate_bps: f64,
        min_tick_size: f64,
        min_order_size: Option<f64>,
        start_time_ms: u64,
        end_time_ms: u64,
        outcome_tokens: Option<&OutcomeTokenIds>,
    ) -> Result<()> {
        let key = market_metadata_key(market_id);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "feeRateBps", fee_rate_bps)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "minTickSize", min_tick_size)
            .await?;
        if let Some(min_order_size) = min_order_size {
            conn.hset::<_, _, _, ()>(key.as_str(), "orderMinSize", min_order_size)
                .await?;
        }
        conn.hset::<_, _, _, ()>(key.as_str(), "startTimeMs", start_time_ms as i64)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "endTimeMs", end_time_ms as i64)
            .await?;
        if let Some(tokens) = outcome_tokens {
            conn.hset::<_, _, _, ()>(key.as_str(), "outcomeUpTokenId", &tokens.up)
                .await?;
            conn.hset::<_, _, _, ()>(key.as_str(), "outcomeDownTokenId", &tokens.down)
                .await?;
        }
        Ok(())
    }

    pub async fn set_asset_window(
        &self,
        asset: &str,
        window: MarketWindow,
        market_id: &str,
        updated_at_ms: u64,
    ) -> Result<()> {
        let key = asset_window_key(asset);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "startTimeMs", window.start_time_ms as i64)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "endTimeMs", window.end_time_ms as i64)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "marketId", market_id)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn set_asset_window_next(
        &self,
        asset: &str,
        window: MarketWindow,
        market_id: &str,
        updated_at_ms: u64,
    ) -> Result<()> {
        let key = asset_window_next_key(asset);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "startTimeMs", window.start_time_ms as i64)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "endTimeMs", window.end_time_ms as i64)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "marketId", market_id)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn add_asset_window_cache(
        &self,
        asset: &str,
        market_id: &str,
        start_time_ms: u64,
    ) -> Result<()> {
        let key = asset_window_cache_key(asset);
        let mut conn = self.connection.clone();
        let _: () = redis::cmd("ZADD")
            .arg(&key)
            .arg(start_time_ms as i64)
            .arg(market_id)
            .query_async(&mut conn)
            .await?;
        Ok(())
    }

    pub async fn prune_asset_window_cache(&self, asset: &str, cutoff_ms: u64) -> Result<usize> {
        if cutoff_ms == 0 {
            return Ok(0);
        }
        let key = asset_window_cache_key(asset);
        self.zremrangebyscore(&key, 0.0, cutoff_ms as f64).await
    }

    pub async fn clear_asset_window_next(&self, asset: &str) -> Result<()> {
        let key = asset_window_next_key(asset);
        let mut conn = self.connection.clone();
        conn.del::<_, ()>(key).await?;
        Ok(())
    }

    pub async fn get_asset_window(&self, asset: &str) -> Result<Option<AssetWindow>> {
        let now = now_ms().unwrap_or(0);
        let current = self
            .read_asset_window(asset_window_key(asset).as_str())
            .await?;
        if let Some(window) = current {
            if now < window.end_time_ms {
                return Ok(Some(window));
            }
        }
        if let Some(next) = self.get_asset_window_next(asset).await? {
            let _ = self
                .set_asset_window(
                    asset,
                    MarketWindow {
                        start_time_ms: next.start_time_ms,
                        end_time_ms: next.end_time_ms,
                    },
                    &next.market_id,
                    now,
                )
                .await;
            return Ok(Some(AssetWindow {
                start_time_ms: next.start_time_ms,
                end_time_ms: next.end_time_ms,
                market_id: next.market_id,
                updated_at_ms: now,
            }));
        }
        if let Some((current, next)) = self.resolve_cached_window(asset, now).await? {
            let _ = self
                .set_asset_window(
                    asset,
                    MarketWindow {
                        start_time_ms: current.start_time_ms,
                        end_time_ms: current.end_time_ms,
                    },
                    &current.market_id,
                    now,
                )
                .await;
            if let Some(next) = next {
                let _ = self
                    .set_asset_window_next(
                        asset,
                        MarketWindow {
                            start_time_ms: next.start_time_ms,
                            end_time_ms: next.end_time_ms,
                        },
                        &next.market_id,
                        now,
                    )
                    .await;
            }
            return Ok(Some(AssetWindow {
                start_time_ms: current.start_time_ms,
                end_time_ms: current.end_time_ms,
                market_id: current.market_id,
                updated_at_ms: now,
            }));
        }
        Ok(None)
    }

    pub async fn get_asset_window_next(&self, asset: &str) -> Result<Option<AssetWindow>> {
        self.read_asset_window(asset_window_next_key(asset).as_str())
            .await
    }

    pub async fn push_activity_log(&self, entry: &str, max_len: usize) -> Result<()> {
        self.push_log(TUI_ACTIVITY_LOG_KEY, entry, max_len).await
    }

    pub async fn get_activity_log(&self, limit: usize) -> Result<Vec<String>> {
        self.get_log(TUI_ACTIVITY_LOG_KEY, limit).await
    }

    pub async fn push_intent_log(&self, entry: &str, max_len: usize) -> Result<()> {
        self.push_log(TUI_INTENT_LOG_KEY, entry, max_len).await
    }

    pub async fn get_intent_log(&self, limit: usize) -> Result<Vec<String>> {
        self.get_log(TUI_INTENT_LOG_KEY, limit).await
    }

    pub async fn push_order_log(&self, entry: &str, max_len: usize) -> Result<()> {
        self.push_log(TUI_ORDER_LOG_KEY, entry, max_len).await
    }

    pub async fn get_order_log(&self, limit: usize) -> Result<Vec<String>> {
        self.get_log(TUI_ORDER_LOG_KEY, limit).await
    }

    pub async fn get_market_window(&self, market_id: &str) -> Result<Option<MarketWindow>> {
        let key = market_metadata_key(market_id);
        let start_time_ms = self.hget_i64(&key, "startTimeMs").await?;
        let end_time_ms = self.hget_i64(&key, "endTimeMs").await?;
        match (start_time_ms, end_time_ms) {
            (Some(start), Some(end)) if start > 0 && end > 0 && end > start => {
                Ok(Some(MarketWindow {
                    start_time_ms: start as u64,
                    end_time_ms: end as u64,
                }))
            }
            _ => Ok(None),
        }
    }

    pub async fn get_market_metadata(&self, market_id: &str) -> Result<MarketMetadata> {
        let key = market_metadata_key(market_id);
        let fee_rate_bps = self.hget_float(&key, "feeRateBps").await?;
        let min_tick_size = self.hget_float(&key, "minTickSize").await?;
        let min_order_size = self.hget_float(&key, "orderMinSize").await?;
        let start_time_ms = self.hget_i64(&key, "startTimeMs").await?;
        let end_time_ms = self.hget_i64(&key, "endTimeMs").await?;
        let outcome_up_token_id = self.hget_string(&key, "outcomeUpTokenId").await?;
        let outcome_down_token_id = self.hget_string(&key, "outcomeDownTokenId").await?;
        Ok(MarketMetadata {
            fee_rate_bps,
            min_tick_size,
            min_order_size,
            start_time_ms: start_time_ms.map(|value| value as u64),
            end_time_ms: end_time_ms.map(|value| value as u64),
            outcome_up_token_id,
            outcome_down_token_id,
        })
    }

    pub async fn set_fee_rate_bps(
        &self,
        token_id: &str,
        fee_rate_bps: f64,
        updated_at_ms: u64,
    ) -> Result<()> {
        let key = fee_rate_key(token_id);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "feeRateBps", fee_rate_bps)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn get_fee_rate_bps(&self, token_id: &str) -> Result<Option<f64>> {
        let key = fee_rate_key(token_id);
        self.hget_float(&key, "feeRateBps").await
    }

    pub async fn set_token_market(&self, token_id: &str, market_id: &str) -> Result<()> {
        let key = token_market_key(token_id);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "marketId", market_id)
            .await?;
        Ok(())
    }

    pub async fn get_token_market(&self, token_id: &str) -> Result<Option<String>> {
        let key = token_market_key(token_id);
        self.hget_string(&key, "marketId").await
    }

    pub async fn set_orderbook_update_ms(&self, token_id: &str, updated_at_ms: u64) -> Result<()> {
        let key = orderbook_ts_key(token_id);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn set_last_trade_price(
        &self,
        token_id: &str,
        price: f64,
        updated_at_ms: u64,
    ) -> Result<()> {
        let key = last_trade_key(token_id);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "price", price)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn get_last_trade_price(&self, token_id: &str) -> Result<Option<f64>> {
        let key = last_trade_key(token_id);
        self.hget_float(&key, "price").await
    }

    pub async fn get_orderbook_update_ms(&self, token_id: &str) -> Result<Option<u64>> {
        let key = orderbook_ts_key(token_id);
        let updated_at_ms = self.hget_i64(&key, "updatedAtMs").await?;
        Ok(updated_at_ms.map(|value| value as u64))
    }

    pub async fn set_asset_start_price(
        &self,
        asset: &str,
        start_time_ms: u64,
        price: f64,
        updated_at_ms: u64,
    ) -> Result<()> {
        let key = asset_start_price_key(asset);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "startTimeMs", start_time_ms as i64)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "price", price)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn get_asset_start_price(&self, asset: &str) -> Result<Option<(u64, f64)>> {
        let key = asset_start_price_key(asset);
        let start_time_ms = self.hget_i64(&key, "startTimeMs").await?;
        let price = self.hget_float(&key, "price").await?;
        match (start_time_ms, price) {
            (Some(start), Some(price)) if start > 0 && price > 0.0 => {
                Ok(Some((start as u64, price)))
            }
            _ => Ok(None),
        }
    }

    pub async fn set_asset_end_price(
        &self,
        asset: &str,
        end_time_ms: u64,
        price: f64,
        updated_at_ms: u64,
    ) -> Result<()> {
        let key = asset_end_price_key(asset);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(key.as_str(), "endTimeMs", end_time_ms as i64)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "price", price)
            .await?;
        conn.hset::<_, _, _, ()>(key.as_str(), "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn get_asset_end_price(&self, asset: &str) -> Result<Option<(u64, f64)>> {
        let key = asset_end_price_key(asset);
        let end_time_ms = self.hget_i64(&key, "endTimeMs").await?;
        let price = self.hget_float(&key, "price").await?;
        match (end_time_ms, price) {
            (Some(end), Some(price)) if end > 0 && price > 0.0 => Ok(Some((end as u64, price))),
            _ => Ok(None),
        }
    }

    pub async fn set_chainlink_price(
        &self,
        asset: &str,
        price: f64,
        updated_at_ms: u64,
    ) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(CHAINLINK_STATUS_KEY, asset, price)
            .await?;
        conn.hset::<_, _, _, ()>(CHAINLINK_STATUS_KEY, "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn get_chainlink_price(&self, asset: &str) -> Result<Option<f64>> {
        self.hget_float(CHAINLINK_STATUS_KEY, asset).await
    }

    pub async fn get_chainlink_updated_ms(&self) -> Result<Option<u64>> {
        let value = self.hget_i64(CHAINLINK_STATUS_KEY, "updatedAtMs").await?;
        Ok(value.map(|ts| ts as u64))
    }

    pub async fn set_polymarket_asset_ids(&self, asset_ids: &[String]) -> Result<()> {
        self.replace_set(POLYMARKET_ASSET_IDS_KEY, asset_ids).await
    }

    pub async fn get_polymarket_asset_ids(&self) -> Result<Vec<String>> {
        self.smembers(POLYMARKET_ASSET_IDS_KEY).await
    }

    pub async fn smembers(&self, key: &str) -> Result<Vec<String>> {
        let mut conn = self.connection.clone();
        Ok(conn.smembers(key).await?)
    }

    pub async fn scard(&self, key: &str) -> Result<usize> {
        let mut conn = self.connection.clone();
        let count: i64 = conn.scard(key).await?;
        Ok(count.max(0) as usize)
    }

    pub async fn sadd(&self, key: &str, member: &str) -> Result<bool> {
        let mut conn = self.connection.clone();
        let added: u32 = conn.sadd(key, member).await?;
        Ok(added > 0)
    }

    pub async fn srem(&self, key: &str, member: &str) -> Result<bool> {
        let mut conn = self.connection.clone();
        let removed: u32 = conn.srem(key, member).await?;
        Ok(removed > 0)
    }

    pub async fn sismember(&self, key: &str, member: &str) -> Result<bool> {
        let mut conn = self.connection.clone();
        Ok(conn.sismember(key, member).await?)
    }

    pub async fn expire(&self, key: &str, ttl_secs: usize) -> Result<()> {
        let mut conn = self.connection.clone();
        let ttl = i64::try_from(ttl_secs).unwrap_or(i64::MAX);
        let _: bool = conn.expire(key, ttl).await?;
        Ok(())
    }

    pub async fn set_if_absent(&self, key: &str, value: &str, ttl_secs: u64) -> Result<bool> {
        let mut conn = self.connection.clone();
        let ttl = ttl_secs.max(1);
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("NX")
            .arg("EX")
            .arg(ttl)
            .query_async(&mut conn)
            .await?;
        Ok(result.is_some())
    }

    pub async fn set_order_state(
        &self,
        wallet_key: &str,
        order_id: &str,
        payload: &str,
        updated_at_ms: u64,
    ) -> Result<()> {
        let details_key = orders_state_key(wallet_key);
        let last_key = orders_last_key(wallet_key);
        let mut conn = self.connection.clone();
        conn.hset::<_, _, _, ()>(details_key.as_str(), order_id, payload)
            .await?;
        conn.hset::<_, _, _, ()>(last_key.as_str(), "payload", payload)
            .await?;
        conn.hset::<_, _, _, ()>(last_key.as_str(), "updatedAtMs", updated_at_ms as i64)
            .await?;
        Ok(())
    }

    pub async fn get_last_order_state(&self, wallet_key: &str) -> Result<Option<String>> {
        let key = orders_last_key(wallet_key);
        self.hget_string(&key, "payload").await
    }

    pub async fn replace_set(&self, key: &str, values: &[String]) -> Result<()> {
        let mut conn = self.connection.clone();
        let mut pipe = redis::pipe();
        pipe.del(key);
        if !values.is_empty() {
            pipe.sadd(key, values);
        }
        pipe.query_async::<_, ()>(&mut conn).await?;
        Ok(())
    }

    pub async fn set_string(&self, key: &str, value: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.set::<_, _, ()>(key, value).await?;
        Ok(())
    }

    pub async fn get_string(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.connection.clone();
        Ok(conn.get(key).await?)
    }

    pub async fn del(&self, key: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.del::<_, ()>(key).await?;
        Ok(())
    }

    pub async fn hgetall_f64(&self, key: &str) -> Result<HashMap<String, f64>> {
        let mut conn = self.connection.clone();
        Ok(conn.hgetall(key).await?)
    }

    pub async fn get_tracked_position(&self, wallet_key: &str, asset_id: &str) -> Result<f64> {
        let key = tracked_positions_key(wallet_key);
        Ok(self.hget_float(&key, asset_id).await?.unwrap_or(0.0))
    }

    pub async fn set_tracked_position(
        &self,
        wallet_key: &str,
        asset_id: &str,
        balance: f64,
    ) -> Result<()> {
        let key = tracked_positions_key(wallet_key);
        let mut conn = self.connection.clone();
        if balance <= 0.0 {
            conn.hdel::<_, _, ()>(&key, asset_id).await?;
        } else {
            conn.hset::<_, _, _, ()>(&key, asset_id, balance).await?;
        }
        Ok(())
    }

    pub async fn get_entry_price(&self, wallet_key: &str, asset_id: &str) -> Result<Option<f64>> {
        let key = entry_price_key(wallet_key);
        self.hget_float(&key, asset_id).await
    }

    pub async fn set_entry_price(
        &self,
        wallet_key: &str,
        asset_id: &str,
        price: f64,
    ) -> Result<()> {
        let key = entry_price_key(wallet_key);
        let mut conn = self.connection.clone();
        if price <= 0.0 {
            conn.hdel::<_, _, ()>(&key, asset_id).await?;
        } else {
            conn.hset::<_, _, _, ()>(&key, asset_id, price).await?;
        }
        Ok(())
    }

    pub async fn get_peak_price(&self, wallet_key: &str, asset_id: &str) -> Result<Option<f64>> {
        let key = peak_price_key(wallet_key);
        self.hget_float(&key, asset_id).await
    }

    pub async fn set_peak_price(&self, wallet_key: &str, asset_id: &str, price: f64) -> Result<()> {
        let key = peak_price_key(wallet_key);
        let mut conn = self.connection.clone();
        if price <= 0.0 {
            conn.hdel::<_, _, ()>(&key, asset_id).await?;
        } else {
            conn.hset::<_, _, _, ()>(&key, asset_id, price).await?;
        }
        Ok(())
    }
}

fn market_metadata_key(market_id: &str) -> String {
    format!("market:{market_id}:metadata")
}

fn asset_window_key(asset: &str) -> String {
    format!("polymarket:window:{asset}")
}

fn asset_window_next_key(asset: &str) -> String {
    format!("{ASSET_WINDOW_NEXT_PREFIX}{asset}")
}

fn asset_window_cache_key(asset: &str) -> String {
    format!("{ASSET_WINDOW_CACHE_PREFIX}{asset}")
}

fn fee_rate_key(token_id: &str) -> String {
    format!("{FEE_RATE_PREFIX}{token_id}")
}

fn asset_start_price_key(asset: &str) -> String {
    format!("{ASSET_START_PRICE_PREFIX}{asset}")
}

fn asset_end_price_key(asset: &str) -> String {
    format!("{ASSET_END_PRICE_PREFIX}{asset}")
}

fn orderbook_ts_key(token_id: &str) -> String {
    format!("{ORDERBOOK_TS_PREFIX}{token_id}")
}

fn last_trade_key(token_id: &str) -> String {
    format!("{LAST_TRADE_PREFIX}{token_id}")
}

fn token_market_key(token_id: &str) -> String {
    format!("{TOKEN_MARKET_PREFIX}{token_id}")
}

fn orders_state_key(wallet_key: &str) -> String {
    format!("orders:state:{wallet_key}")
}

fn orders_last_key(wallet_key: &str) -> String {
    format!("orders:last:{wallet_key}")
}

fn tracked_positions_key(wallet_key: &str) -> String {
    format!("{POSITIONS_TRACKED_PREFIX}{wallet_key}")
}

fn entry_price_key(wallet_key: &str) -> String {
    format!("{POSITIONS_ENTRY_PREFIX}{wallet_key}")
}

fn peak_price_key(wallet_key: &str) -> String {
    format!("{POSITIONS_PEAK_PREFIX}{wallet_key}")
}

impl RedisManager {
    async fn read_asset_window(&self, key: &str) -> Result<Option<AssetWindow>> {
        let mut conn = self.connection.clone();
        let start_time_ms: Option<i64> = conn.hget(key, "startTimeMs").await?;
        let end_time_ms: Option<i64> = conn.hget(key, "endTimeMs").await?;
        let market_id: Option<String> = conn.hget(key, "marketId").await?;
        let updated_at_ms: Option<i64> = conn.hget(key, "updatedAtMs").await?;
        match (start_time_ms, end_time_ms, market_id, updated_at_ms) {
            (Some(start), Some(end), Some(market_id), Some(updated_at_ms))
                if start > 0 && end > 0 && end > start =>
            {
                Ok(Some(AssetWindow {
                    start_time_ms: start as u64,
                    end_time_ms: end as u64,
                    market_id,
                    updated_at_ms: updated_at_ms as u64,
                }))
            }
            _ => Ok(None),
        }
    }

    async fn push_log(&self, key: &str, entry: &str, max_len: usize) -> Result<()> {
        let mut conn = self.connection.clone();
        let mut pipe = redis::pipe();
        pipe.lpush(key, entry);
        if max_len > 0 {
            pipe.ltrim(key, 0, (max_len - 1) as isize);
        }
        pipe.query_async::<_, ()>(&mut conn).await?;
        Ok(())
    }

    async fn get_log(&self, key: &str, limit: usize) -> Result<Vec<String>> {
        let mut conn = self.connection.clone();
        if limit == 0 {
            return Ok(Vec::new());
        }
        Ok(conn.lrange(key, 0, (limit - 1) as isize).await?)
    }

    async fn resolve_cached_window(
        &self,
        asset: &str,
        now_ms: u64,
    ) -> Result<Option<(AssetWindow, Option<AssetWindow>)>> {
        let key = asset_window_cache_key(asset);
        let horizon_ms = now_ms.saturating_add(24 * 60 * 60 * 1000);
        let min_score = now_ms.saturating_sub(6 * 60 * 60 * 1000);
        let candidates = self
            .zrangebyscore(&key, min_score as f64, horizon_ms as f64)
            .await?;
        if candidates.is_empty() {
            return Ok(None);
        }

        let mut active: Option<AssetWindow> = None;
        let mut next: Option<AssetWindow> = None;
        for market_id in candidates {
            let metadata = self.get_market_metadata(&market_id).await?;
            let (Some(start), Some(end)) = (metadata.start_time_ms, metadata.end_time_ms) else {
                continue;
            };
            if now_ms >= start && now_ms < end {
                active = Some(AssetWindow {
                    start_time_ms: start,
                    end_time_ms: end,
                    market_id: market_id.clone(),
                    updated_at_ms: now_ms,
                });
            } else if start > now_ms {
                let candidate = AssetWindow {
                    start_time_ms: start,
                    end_time_ms: end,
                    market_id: market_id.clone(),
                    updated_at_ms: now_ms,
                };
                if next
                    .as_ref()
                    .map(|current| current.start_time_ms <= start)
                    .unwrap_or(false)
                {
                    continue;
                }
                next = Some(candidate);
            }
        }

        if let Some(active) = active {
            return Ok(Some((active, next)));
        }
        if let Some(next) = next {
            return Ok(Some((next, None)));
        }
        Ok(None)
    }
}

fn now_ms() -> Option<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    Some(now.as_millis() as u64)
}
