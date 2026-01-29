/**
 * @purpose
 * Paper-trade evaluator for no-money mode.
 *
 * @notes
 * - Records intents without placing orders.
 * - Scores them using start/end prices from Chainlink RTDS.
 */
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::task::JoinHandle;

use crate::config::Config;
use crate::engine::types::{TradeIntent, TradeMode, TradeSide};
use crate::error::{BankaiError, Result};
use crate::storage::redis::{MarketMetadata, RedisManager};
use arc_swap::ArcSwap;

const PAPER_ZSET_KEY: &str = "paper:intents";
const PAPER_STATS_WINS: &str = "paper:stats:wins";
const PAPER_STATS_LOSSES: &str = "paper:stats:losses";
const PAPER_STATS_TOTAL: &str = "paper:stats:total";
const PAPER_STATS_ACCURACY: &str = "paper:stats:accuracy_pct";
const PAPER_LOG_LIMIT: usize = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperIntent {
    pub id: String,
    pub asset: String,
    pub market_id: String,
    pub asset_id: String,
    pub predicted: String,
    pub mode: String,
    pub side: String,
    pub implied_prob: f64,
    pub true_prob: f64,
    pub edge_bps: f64,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
    pub emitted_at_ms: u64,
}

pub fn spawn_no_money_tracker(config: Arc<ArcSwap<Config>>, redis: RedisManager) -> JoinHandle<()> {
    tokio::spawn(async move {
        if let Err(error) = run_tracker(config, redis).await {
            tracing::error!(?error, "no-money tracker stopped");
        }
    })
}

pub async fn record_no_money_intent(redis: &RedisManager, intent: &TradeIntent) -> Result<()> {
    let window = intent.market_window.ok_or_else(|| {
        BankaiError::InvalidArgument("paper intent missing market window".to_string())
    })?;
    let asset = resolve_asset_for_market(redis, &intent.market_id).await?;
    let predicted = resolve_prediction(redis, &intent.market_id, &intent.asset_id).await?;
    if predicted == "UNKNOWN" {
        return Ok(());
    }
    let id = format!("paper:intent:{}:{}", asset, window.start_time_ms);
    if redis.get_string(&id).await?.is_some() {
        return Ok(());
    }
    let record = PaperIntent {
        id: id.clone(),
        asset,
        market_id: intent.market_id.clone(),
        asset_id: intent.asset_id.clone(),
        predicted,
        mode: mode_label(intent.mode).to_string(),
        side: side_label(intent.side).to_string(),
        implied_prob: intent.implied_prob,
        true_prob: intent.true_prob,
        edge_bps: intent.edge_bps,
        start_time_ms: window.start_time_ms,
        end_time_ms: window.end_time_ms,
        emitted_at_ms: intent.timestamp_ms,
    };
    let payload = serde_json::to_string(&record)?;
    redis.set_string(&id, &payload).await?;
    redis
        .zadd(PAPER_ZSET_KEY, record.end_time_ms as f64, &id)
        .await?;
    Ok(())
}

async fn run_tracker(config: Arc<ArcSwap<Config>>, redis: RedisManager) -> Result<()> {
    let mut tick = tokio::time::interval(Duration::from_secs(1));
    loop {
        tick.tick().await;
        let cfg = config.load_full();
        if !cfg.execution.no_money_mode {
            continue;
        }

        let now = now_ms()?;
        let pending = redis.zrange_with_scores(PAPER_ZSET_KEY, 0, -1).await?;
        for (key, score) in pending {
            if score as u64 > now {
                break;
            }
            let payload = match redis.get_string(&key).await? {
                Some(value) => value,
                None => {
                    let _ = redis.zrem(PAPER_ZSET_KEY, &key).await;
                    continue;
                }
            };
            let record: PaperIntent = match serde_json::from_str(&payload) {
                Ok(value) => value,
                Err(_) => {
                    let _ = redis.zrem(PAPER_ZSET_KEY, &key).await;
                    let _ = redis.del(&key).await;
                    continue;
                }
            };

            let Some((start_ms, start_price)) = redis.get_asset_start_price(&record.asset).await?
            else {
                continue;
            };
            if start_ms != record.start_time_ms {
                continue;
            }
            let Some((end_ms, end_price)) = redis.get_asset_end_price(&record.asset).await? else {
                continue;
            };
            if end_ms != record.end_time_ms {
                continue;
            }

            let actual = if end_price >= start_price {
                "UP"
            } else {
                "DOWN"
            };
            let correct = record.predicted == actual;

            if correct {
                let _ = redis.incr_float(PAPER_STATS_WINS, 1.0).await;
            } else {
                let _ = redis.incr_float(PAPER_STATS_LOSSES, 1.0).await;
            }
            let total = redis
                .incr_float(PAPER_STATS_TOTAL, 1.0)
                .await
                .unwrap_or(0.0);
            let wins = redis.get_float(PAPER_STATS_WINS).await?.unwrap_or(0.0);
            let accuracy = if total > 0.0 {
                (wins / total) * 100.0
            } else {
                0.0
            };
            let _ = redis.set_float(PAPER_STATS_ACCURACY, accuracy).await;

            let message = format!(
                "[PAPER] asset={} market={} predicted={} actual={} start={:.4} end={:.4} ok={} accuracy={:.2}%",
                record.asset,
                record.market_id,
                record.predicted,
                actual,
                start_price,
                end_price,
                correct,
                accuracy
            );
            let _ = redis.push_activity_log(&message, PAPER_LOG_LIMIT).await;

            let _ = redis.zrem(PAPER_ZSET_KEY, &key).await;
            let _ = redis.del(&key).await;
        }
    }
}

async fn resolve_asset_for_market(redis: &RedisManager, market_id: &str) -> Result<String> {
    for asset in ["BTC", "ETH", "SOL"] {
        if let Some(window) = redis.get_asset_window(&asset).await? {
            if window.market_id == market_id {
                return Ok(asset.to_string());
            }
        }
    }
    Err(BankaiError::InvalidArgument(
        "unable to resolve asset for market".to_string(),
    ))
}

async fn resolve_prediction(
    redis: &RedisManager,
    market_id: &str,
    asset_id: &str,
) -> Result<String> {
    let metadata: MarketMetadata = redis.get_market_metadata(market_id).await?;
    if metadata
        .outcome_up_token_id
        .as_ref()
        .map(|value| value == asset_id)
        .unwrap_or(false)
    {
        return Ok("UP".to_string());
    }
    if metadata
        .outcome_down_token_id
        .as_ref()
        .map(|value| value == asset_id)
        .unwrap_or(false)
    {
        return Ok("DOWN".to_string());
    }
    Ok("UNKNOWN".to_string())
}

fn mode_label(mode: TradeMode) -> &'static str {
    match mode {
        TradeMode::Ladder => "LADDER",
        TradeMode::Snipe => "SNIPE",
    }
}

fn side_label(side: TradeSide) -> &'static str {
    match side {
        TradeSide::Buy => "BUY",
        TradeSide::Sell => "SELL",
    }
}

fn now_ms() -> Result<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now.as_millis() as u64)
}
