/**
 * @description
 * Polymarket market discovery via Gamma API with liquidity and risk filters.
 *
 * @dependencies
 * - reqwest: HTTP client for Gamma API polling
 * - serde_json: response parsing
 *
 * @notes
 * - Filters out augmented negative risk markets (any with "Other" outcomes).
 * - Stores feeRateBps and minTickSize in Redis for eligible markets.
 */
use reqwest::Client;
use serde_json::Value;
use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::error::Result;
use crate::storage::redis::RedisManager;

const DEFAULT_LIMIT: usize = 200;
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(30);
const DEFAULT_JITTER_MS: u64 = 3_000;
const DEFAULT_MIN_LIQUIDITY: f64 = 2000.0;

#[derive(Debug, Clone)]
pub struct PolymarketDiscoveryConfig {
    pub base_url: String,
    pub poll_interval: Duration,
    pub jitter_ms: u64,
    pub min_liquidity: f64,
    pub limit: usize,
}

impl PolymarketDiscoveryConfig {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            poll_interval: DEFAULT_POLL_INTERVAL,
            jitter_ms: DEFAULT_JITTER_MS,
            min_liquidity: DEFAULT_MIN_LIQUIDITY,
            limit: DEFAULT_LIMIT,
        }
    }
}

pub struct PolymarketDiscovery {
    config: PolymarketDiscoveryConfig,
    client: Client,
    redis: RedisManager,
}

impl PolymarketDiscovery {
    pub fn new(config: PolymarketDiscoveryConfig, redis: RedisManager) -> Result<Self> {
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        Ok(Self {
            config,
            client,
            redis,
        })
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "polymarket discovery stopped");
            }
        })
    }

    async fn run(self) -> Result<()> {
        loop {
            if let Err(error) = self.refresh_markets().await {
                tracing::warn!(?error, "polymarket discovery refresh failed");
            }
            let jitter = compute_jitter(self.config.jitter_ms)?;
            tokio::time::sleep(self.config.poll_interval + jitter).await;
        }
    }

    async fn refresh_markets(&self) -> Result<()> {
        let mut offset = 0usize;
        let mut asset_ids = HashSet::new();
        loop {
            let markets = self.fetch_markets(offset).await?;
            if markets.is_empty() {
                break;
            }
            for market in &markets {
                if let Some(metadata) =
                    extract_market_metadata(market, self.config.min_liquidity)
                {
                    self.redis
                        .set_market_metadata(
                            &metadata.market_id,
                            metadata.fee_rate_bps,
                            metadata.min_tick_size,
                        )
                        .await?;
                    for asset_id in extract_market_asset_ids(market) {
                        asset_ids.insert(asset_id);
                    }
                }
            }

            if markets.len() < self.config.limit {
                break;
            }
            offset += self.config.limit;
        }
        let mut asset_ids: Vec<String> = asset_ids.into_iter().collect();
        asset_ids.sort();
        self.redis.set_polymarket_asset_ids(&asset_ids).await?;
        Ok(())
    }

    async fn fetch_markets(&self, offset: usize) -> Result<Vec<Value>> {
        let url = format!(
            "{}/markets?closed=false&limit={}&offset={}",
            self.config.base_url.trim_end_matches('/'),
            self.config.limit,
            offset
        );
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json::<Vec<Value>>().await?)
    }
}

#[derive(Debug)]
struct MarketMetadata {
    market_id: String,
    fee_rate_bps: f64,
    min_tick_size: f64,
}

fn extract_market_metadata(market: &Value, min_liquidity: f64) -> Option<MarketMetadata> {
    let market_id = parse_market_id(market)?;
    let liquidity = parse_liquidity(market)?;
    if liquidity < min_liquidity {
        return None;
    }
    if is_augmented_negative_risk(market) {
        return None;
    }
    let fee_rate_bps = parse_fee_rate_bps(market)?;
    let min_tick_size = parse_min_tick_size(market)?;

    Some(MarketMetadata {
        market_id,
        fee_rate_bps,
        min_tick_size,
    })
}

fn parse_market_id(market: &Value) -> Option<String> {
    let raw = market.get("id")?;
    match raw {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}

fn parse_liquidity(market: &Value) -> Option<f64> {
    let candidates = ["liquidityNum", "liquidity_num", "liquidity"];
    for key in candidates {
        if let Some(value) = market.get(key) {
            if let Some(parsed) = parse_numeric(value) {
                return Some(parsed);
            }
        }
    }
    None
}

fn parse_fee_rate_bps(market: &Value) -> Option<f64> {
    let candidates = ["feeRateBps", "fee_rate_bps", "feeBps", "fee_bps", "fee"];
    for key in candidates {
        if let Some(value) = market.get(key) {
            if let Some(parsed) = parse_numeric(value) {
                return Some(parsed);
            }
        }
    }
    None
}

fn parse_min_tick_size(market: &Value) -> Option<f64> {
    let candidates = ["minTickSize", "min_tick_size", "tickSize", "tick_size"];
    for key in candidates {
        if let Some(value) = market.get(key) {
            if let Some(parsed) = parse_numeric(value) {
                return Some(parsed);
            }
        }
    }
    None
}

fn is_augmented_negative_risk(market: &Value) -> bool {
    if market
        .get("negRiskOther")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return true;
    }

    if market
        .get("neg_risk_other")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return true;
    }

    let outcomes = match market.get("outcomes") {
        Some(value) => value,
        None => return false,
    };
    if let Some(entries) = outcomes.as_array() {
        return entries.iter().any(is_other_outcome);
    }
    if let Some(text) = outcomes.as_str() {
        if text.to_ascii_lowercase().contains("other") {
            if let Ok(parsed) = serde_json::from_str::<Value>(text) {
                if let Some(entries) = parsed.as_array() {
                    return entries.iter().any(is_other_outcome);
                }
            }
            return true;
        }
    }
    false
}

fn is_other_outcome(value: &Value) -> bool {
    value
        .as_str()
        .map(|item| item.eq_ignore_ascii_case("other"))
        .unwrap_or(false)
}

fn parse_numeric(value: &Value) -> Option<f64> {
    if let Some(number) = value.as_f64() {
        return Some(number);
    }
    if let Some(text) = value.as_str() {
        if let Ok(parsed) = text.parse::<f64>() {
            return Some(parsed);
        }
    }
    None
}

fn extract_market_asset_ids(market: &Value) -> Vec<String> {
    let candidates = [
        "clobTokenIds",
        "clob_token_ids",
        "tokenIds",
        "token_ids",
        "assetIds",
        "asset_ids",
    ];
    let mut ids = Vec::new();
    for key in candidates {
        if let Some(value) = market.get(key) {
            parse_token_field(value, &mut ids);
        }
    }
    ids
}

fn parse_token_field(value: &Value, output: &mut Vec<String>) {
    match value {
        Value::Array(entries) => {
            for entry in entries {
                match entry {
                    Value::String(text) => push_token_id(text, output),
                    Value::Number(num) => output.push(num.to_string()),
                    _ => {}
                }
            }
        }
        Value::String(text) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return;
            }
            if let Ok(parsed) = serde_json::from_str::<Value>(trimmed) {
                parse_token_field(&parsed, output);
            } else {
                for part in trimmed.split(',') {
                    push_token_id(part.trim(), output);
                }
            }
        }
        Value::Number(num) => output.push(num.to_string()),
        _ => {}
    }
}

fn push_token_id(value: &str, output: &mut Vec<String>) {
    let trimmed = value.trim();
    if !trimmed.is_empty() {
        output.push(trimmed.to_string());
    }
}

fn compute_jitter(max_ms: u64) -> Result<Duration> {
    if max_ms == 0 {
        return Ok(Duration::from_millis(0));
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let jitter = now.subsec_millis() as u64 % (max_ms + 1);
    Ok(Duration::from_millis(jitter))
}
