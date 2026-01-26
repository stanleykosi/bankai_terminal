/**
 * @purpose
 * Helpers for de-duplicating trades and tracking realized PnL windows.
 */
use crate::accounting::keys::{
    PNL_24H_KEY, REALIZED_EVENTS_PREFIX, REALIZED_PNL_24H_KEY, SEEN_TRADES_PREFIX,
};
use crate::error::Result;
use crate::storage::redis::RedisManager;

const REALIZED_EVENT_TTL_SECS: usize = 172_800;
const WINDOW_24H_SECS: u64 = 86_400;

pub fn seen_trade_key(wallet: &str) -> String {
    format!("{SEEN_TRADES_PREFIX}{wallet}")
}

pub fn realized_events_key(wallet: &str) -> String {
    format!("{REALIZED_EVENTS_PREFIX}{wallet}")
}

pub async fn is_seen_trade(redis: &RedisManager, wallet: &str, trade_id: &str) -> Result<bool> {
    let key = seen_trade_key(wallet);
    redis.sismember(&key, trade_id).await
}

pub async fn mark_seen_trade(redis: &RedisManager, wallet: &str, trade_id: &str) -> Result<()> {
    let key = seen_trade_key(wallet);
    let _ = redis.sadd(&key, trade_id).await?;
    let _ = redis.expire(&key, REALIZED_EVENT_TTL_SECS).await;
    Ok(())
}

pub async fn record_realized_pnl_event(
    redis: &RedisManager,
    wallet: &str,
    trade_id: &str,
    timestamp: u64,
    pnl: f64,
) -> Result<f64> {
    let key = realized_events_key(wallet);
    let member = format!("{trade_id}|{pnl}");
    let _ = redis.zadd(&key, timestamp as f64, &member).await?;
    let cutoff = timestamp.saturating_sub(REALIZED_EVENT_TTL_SECS as u64);
    let _ = redis
        .zremrangebyscore(&key, 0.0, cutoff as f64)
        .await?;

    let window_start = timestamp.saturating_sub(WINDOW_24H_SECS);
    let entries = redis
        .zrangebyscore(&key, window_start as f64, timestamp as f64)
        .await?;
    let mut sum = 0.0;
    for entry in entries {
        if let Some((_, value)) = entry.rsplit_once('|') {
            if let Ok(parsed) = value.parse::<f64>() {
                sum += parsed;
            }
        }
    }
    let _ = redis.set_float(REALIZED_PNL_24H_KEY, sum).await;
    let unrealized = redis
        .get_float(crate::accounting::keys::UNREALIZED_PNL_KEY)
        .await?
        .unwrap_or(0.0);
    let _ = redis.set_float(PNL_24H_KEY, sum + unrealized).await;
    Ok(sum)
}
