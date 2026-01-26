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
 * - Stores feeRateBps, minTickSize, and 15m time windows in Redis for eligible markets.
 */
use chrono::offset::LocalResult;
use chrono::{Datelike, Duration as ChronoDuration, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use chrono_tz::America::New_York;
use regex::Regex;
use reqwest::Client;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::engine::types::MarketWindow;
use crate::error::Result;
use crate::storage::redis::RedisManager;

const DEFAULT_LIMIT: usize = 200;
const DEFAULT_POLL_INTERVAL: Duration = Duration::from_secs(30);
const DEFAULT_JITTER_MS: u64 = 3_000;
const DEFAULT_MIN_LIQUIDITY: f64 = 2000.0;
const ACTIVITY_LOG_LIMIT: usize = 50;

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
    logged_markets: HashSet<String>,
    last_asset_windows: HashMap<AssetSymbol, MarketTimeWindow>,
}

impl PolymarketDiscovery {
    pub fn new(config: PolymarketDiscoveryConfig, redis: RedisManager) -> Result<Self> {
        let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
        Ok(Self {
            config,
            client,
            redis,
            logged_markets: HashSet::new(),
            last_asset_windows: HashMap::new(),
        })
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "polymarket discovery stopped");
            }
        })
    }

    async fn run(mut self) -> Result<()> {
        loop {
            if let Err(error) = self.refresh_markets().await {
                tracing::warn!(?error, "polymarket discovery refresh failed");
            }
            let jitter = compute_jitter(self.config.jitter_ms)?;
            tokio::time::sleep(self.config.poll_interval + jitter).await;
        }
    }

    async fn refresh_markets(&mut self) -> Result<()> {
        let mut scanned = 0usize;
        let mut accepted = 0usize;
        let mut windows_by_asset: HashMap<AssetSymbol, Vec<MarketCandidate>> =
            HashMap::new();

        let now_ms = now_ms().unwrap_or(0);
        for tag in TARGET_TAGS {
            let mut offset = 0usize;
            loop {
                let markets = self.fetch_markets(tag, offset).await?;
                if markets.is_empty() {
                    break;
                }
                for market in &markets {
                    scanned += 1;
                    if !is_open_market(market) {
                        continue;
                    }
                    let Some(asset_symbol) = target_asset(market).or_else(|| asset_from_tag(tag))
                    else {
                        continue;
                    };
                    if let Some(time_window) = is_target_market(market) {
                        if time_window.end_time_ms <= now_ms {
                            continue;
                        }
                        if let Some(metadata) =
                            extract_market_metadata(market, self.config.min_liquidity, &time_window)
                        {
                            self.redis
                                .set_market_metadata(
                                    &metadata.market_id,
                                    metadata.fee_rate_bps,
                                    metadata.min_tick_size,
                                    metadata.start_time_ms,
                                    metadata.end_time_ms,
                                )
                                .await?;
                            accepted += 1;
                            let label = market_label(market);
                            windows_by_asset
                                .entry(asset_symbol)
                                .or_default()
                                .push(MarketCandidate {
                                    window: time_window,
                                    market_id: metadata.market_id.clone(),
                                    label: label.clone(),
                                    asset_ids: extract_market_asset_ids(market),
                                });
                            self.log_market_if_new(
                                &metadata.market_id,
                                asset_symbol,
                                &time_window,
                                &label,
                            )
                            .await;
                        }
                    }
                }

                if markets.len() < self.config.limit {
                    break;
                }
                offset += self.config.limit;
            }
        }
        let mut asset_ids = HashSet::new();
        self.update_asset_windows(windows_by_asset, &mut asset_ids)
            .await;
        let mut asset_ids: Vec<String> = asset_ids.into_iter().collect();
        asset_ids.sort();
        self.redis.set_polymarket_asset_ids(&asset_ids).await?;
        self.log_refresh_summary(scanned, accepted, asset_ids.len())
            .await;
        Ok(())
    }

    async fn fetch_markets(&self, tag_id: &str, offset: usize) -> Result<Vec<Value>> {
        let url = format!(
            "{}/markets?closed=false&tag_id={}&limit={}&offset={}",
            self.config.base_url.trim_end_matches('/'),
            tag_id,
            self.config.limit,
            offset
        );
        let response = self.client.get(url).send().await?.error_for_status()?;
        Ok(response.json::<Vec<Value>>().await?)
    }

    async fn log_market_if_new(
        &mut self,
        market_id: &str,
        asset: AssetSymbol,
        window: &MarketTimeWindow,
        label: &str,
    ) {
        if !self.logged_markets.insert(market_id.to_string()) {
            return;
        }
        let prefix = log_prefix();
        let window_label = format_window_et(window);
        let message = format!(
            "{prefix} Market accepted {} id={} window={}",
            asset.as_str(),
            market_id,
            window_label
        );
        let _ = self
            .redis
            .push_activity_log(&message, ACTIVITY_LOG_LIMIT)
            .await;
        if !label.is_empty() && label.len() < 120 {
            let detail = format!("{prefix}  -> {label}");
            let _ = self
                .redis
                .push_activity_log(&detail, ACTIVITY_LOG_LIMIT)
                .await;
        }
    }

    async fn log_refresh_summary(&self, scanned: usize, accepted: usize, asset_count: usize) {
        let prefix = log_prefix();
        let message = format!(
            "{prefix} Discovery scan: scanned={scanned} accepted={accepted} assets={asset_count}"
        );
        let _ = self
            .redis
            .push_activity_log(&message, ACTIVITY_LOG_LIMIT)
            .await;
    }

    async fn update_asset_windows(
        &mut self,
        windows_by_asset: HashMap<AssetSymbol, Vec<MarketCandidate>>,
        asset_ids: &mut HashSet<String>,
    ) {
        let now_ms = now_ms().unwrap_or(0);
        for (asset, windows) in windows_by_asset {
            let Some(candidate) = pick_asset_window(now_ms, &windows) else {
                continue;
            };
            let window = candidate.window;
            let market_id = candidate.market_id.as_str();
            let label = candidate.label.as_str();
            let _ = self
                .redis
                .set_asset_window(asset.as_str(), to_market_window(window), market_id, now_ms)
                .await;
            for asset_id in &candidate.asset_ids {
                asset_ids.insert(asset_id.clone());
            }
            let changed = self
                .last_asset_windows
                .get(&asset)
                .map(|prev| prev != &window)
                .unwrap_or(true);
            if changed {
                self.last_asset_windows.insert(asset, window);
                let prefix = log_prefix();
                let window_label = format_window_et(&window);
                let message = format!(
                    "{prefix} Window set {} id={} window={}",
                    asset.as_str(),
                    market_id,
                    window_label
                );
                let _ = self
                    .redis
                    .push_activity_log(&message, ACTIVITY_LOG_LIMIT)
                    .await;
                if !label.is_empty() && label.len() < 120 {
                    let detail = format!("{prefix}  -> {label}");
                    let _ = self
                        .redis
                        .push_activity_log(&detail, ACTIVITY_LOG_LIMIT)
                        .await;
                }
            }
        }
    }
}

#[derive(Debug)]
struct MarketMetadata {
    market_id: String,
    fee_rate_bps: f64,
    min_tick_size: f64,
    start_time_ms: u64,
    end_time_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MarketTimeWindow {
    start_time_ms: u64,
    end_time_ms: u64,
}

#[derive(Debug, Clone)]
struct MarketCandidate {
    window: MarketTimeWindow,
    market_id: String,
    label: String,
    asset_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AssetSymbol {
    Btc,
    Eth,
    Sol,
}

impl AssetSymbol {
    fn as_str(self) -> &'static str {
        match self {
            AssetSymbol::Btc => "BTC",
            AssetSymbol::Eth => "ETH",
            AssetSymbol::Sol => "SOL",
        }
    }
}

fn extract_market_metadata(
    market: &Value,
    min_liquidity: f64,
    time_window: &MarketTimeWindow,
) -> Option<MarketMetadata> {
    let market_id = parse_market_id(market)?;
    let liquidity = parse_liquidity(market)?;
    if liquidity < min_liquidity {
        return None;
    }
    if is_augmented_negative_risk(market) {
        return None;
    }
    let fee_rate_bps = parse_fee_rate_bps(market).unwrap_or(0.0);
    let min_tick_size = parse_min_tick_size(market).unwrap_or(0.001);

    Some(MarketMetadata {
        market_id,
        fee_rate_bps,
        min_tick_size,
        start_time_ms: time_window.start_time_ms,
        end_time_ms: time_window.end_time_ms,
    })
}

fn is_open_market(market: &Value) -> bool {
    let closed = market
        .get("closed")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let active = market
        .get("active")
        .and_then(|value| value.as_bool())
        .unwrap_or(true);
    !closed && active
}

// Match Tag Logic
// BTC: 235, ETH: 39, SOL: 968
const BTC_TAG: &str = "235";
const ETH_TAG: &str = "39";
const SOL_TAG: &str = "968";
const TARGET_TAGS: [&str; 3] = [BTC_TAG, ETH_TAG, SOL_TAG];

fn is_target_market(market: &Value) -> Option<MarketTimeWindow> {
    // 1. Tag Check
    if !has_target_tag(market) {
        return None;
    }

    // 2. Title/Question Check
    let question = market.get("question").and_then(|v| v.as_str()).unwrap_or("");
    let title = market.get("title").and_then(|v| v.as_str()).unwrap_or("");
    let lower_q = question.to_ascii_lowercase();
    let lower_title = title.to_ascii_lowercase();
    if !lower_q.contains("up or down") && !lower_title.contains("up or down") {
        return None;
    }

    // 3. Outcome Check: Must be ["Up", "Down"]
    if !has_valid_outcomes(market) {
        return None;
    }

    // 4. 15m Classification: require 15m slug and validate time window.
    if !has_15m_slug(market) {
        return None;
    }

    extract_time_window(market, title, question)
}

fn has_target_tag(market: &Value) -> bool {
    target_asset(market).is_some()
}

fn target_asset(market: &Value) -> Option<AssetSymbol> {
    if let Some(tags) = market.get("tags").and_then(|v| v.as_array()) {
        for tag in tags {
            if let Some(id_val) = tag.get("id") {
                let id_str = if let Some(s) = id_val.as_str() {
                    s.to_string()
                } else if let Some(n) = id_val.as_u64() {
                    n.to_string()
                } else {
                    continue;
                };
                if let Some(asset) = asset_from_tag(&id_str) {
                    return Some(asset);
                }
            }
        }
    }

    let slug = market.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    if let Some(asset) = asset_from_slug(slug) {
        return Some(asset);
    }

    let question = market.get("question").and_then(|v| v.as_str()).unwrap_or("");
    let title = market.get("title").and_then(|v| v.as_str()).unwrap_or("");
    asset_from_text(question).or_else(|| asset_from_text(title))
}

fn asset_from_tag(tag_id: &str) -> Option<AssetSymbol> {
    match tag_id {
        BTC_TAG => Some(AssetSymbol::Btc),
        ETH_TAG => Some(AssetSymbol::Eth),
        SOL_TAG => Some(AssetSymbol::Sol),
        _ => None,
    }
}

fn asset_from_slug(slug: &str) -> Option<AssetSymbol> {
    let lower = slug.to_ascii_lowercase();
    if lower.starts_with("btc") || lower.starts_with("bitcoin") {
        return Some(AssetSymbol::Btc);
    }
    if lower.starts_with("eth") || lower.starts_with("ethereum") {
        return Some(AssetSymbol::Eth);
    }
    if lower.starts_with("sol") || lower.starts_with("solana") {
        return Some(AssetSymbol::Sol);
    }
    None
}

fn asset_from_text(text: &str) -> Option<AssetSymbol> {
    let lower = text.to_ascii_lowercase();
    if lower.contains("bitcoin") || lower.contains("btc") {
        return Some(AssetSymbol::Btc);
    }
    if lower.contains("ethereum") || lower.contains("eth") {
        return Some(AssetSymbol::Eth);
    }
    if lower.contains("solana") || lower.contains("sol") {
        return Some(AssetSymbol::Sol);
    }
    None
}

fn market_label(market: &Value) -> String {
    let question = market.get("question").and_then(|v| v.as_str()).unwrap_or("");
    if !question.is_empty() {
        return question.to_string();
    }
    let title = market.get("title").and_then(|v| v.as_str()).unwrap_or("");
    title.to_string()
}

fn has_valid_outcomes(market: &Value) -> bool {
    if let Some(outcomes_val) = market.get("outcomes") {
        // Can be string (escaped JSON) or array
        if let Some(arr) = outcomes_val.as_array() {
            return matches_up_down(arr);
        } else if let Some(s) = outcomes_val.as_str() {
            if let Ok(parsed) = serde_json::from_str::<Vec<String>>(s) {
                if parsed.len() == 2 && parsed[0] == "Up" && parsed[1] == "Down" {
                    return true;
                }
            }
        }
    }
    false
}

fn matches_up_down(arr: &[Value]) -> bool {
    if arr.len() != 2 {
        return false;
    }
    arr[0].as_str() == Some("Up") && arr[1].as_str() == Some("Down")
}

fn extract_time_window(market: &Value, title: &str, question: &str) -> Option<MarketTimeWindow> {
    let slug_window = parse_slug_time_window(market)?;
    let reference_et = New_York.from_utc_datetime(&Utc.timestamp_millis_opt(slug_window.start_time_ms as i64).single()?.naive_utc());
    let parsed_window = parse_time_window_from_text(title, reference_et)
        .or_else(|| parse_time_window_from_text(question, reference_et));

    if let Some(parsed) = parsed_window {
        if !windows_match(slug_window, parsed) {
            return None;
        }
    } else {
        return None;
    }

    Some(slug_window)
}

fn has_15m_slug(market: &Value) -> bool {
    let slug = market.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    slug.to_ascii_lowercase().contains("15m")
}

fn parse_slug_time_window(market: &Value) -> Option<MarketTimeWindow> {
    let slug = market.get("slug").and_then(|v| v.as_str()).unwrap_or("");
    let epoch_seconds = parse_slug_epoch_seconds(slug)?;
    let start_time_ms = epoch_seconds.checked_mul(1_000)?;
    let end_time_ms = start_time_ms.checked_add(15 * 60 * 1_000)?;
    Some(MarketTimeWindow {
        start_time_ms,
        end_time_ms,
    })
}

fn parse_slug_epoch_seconds(slug: &str) -> Option<u64> {
    let last = slug.rsplit('-').next()?;
    if last.len() < 9 || last.len() > 12 {
        return None;
    }
    if !last.chars().all(|ch| ch.is_ascii_digit()) {
        return None;
    }
    last.parse::<u64>().ok()
}

fn windows_match(a: MarketTimeWindow, b: MarketTimeWindow) -> bool {
    const TOLERANCE_MS: i64 = 60_000;
    let start_delta = a.start_time_ms as i64 - b.start_time_ms as i64;
    let end_delta = a.end_time_ms as i64 - b.end_time_ms as i64;
    start_delta.abs() <= TOLERANCE_MS && end_delta.abs() <= TOLERANCE_MS
}

fn parse_time_window_from_text(
    text: &str,
    reference_et: chrono::DateTime<chrono_tz::Tz>,
) -> Option<MarketTimeWindow> {
    let captures = time_range_regex().captures(text)?;
    let month = parse_month(captures.get(1)?.as_str())?;
    let day: u32 = captures.get(2)?.as_str().parse().ok()?;
    let year_capture = captures
        .get(3)
        .and_then(|value| value.as_str().parse::<i32>().ok());
    let start_hour: u32 = captures.get(4)?.as_str().parse().ok()?;
    let start_min: u32 = captures.get(5)?.as_str().parse().ok()?;
    let start_meridiem = captures.get(6)?.as_str();
    let end_hour: u32 = captures.get(7)?.as_str().parse().ok()?;
    let end_min: u32 = captures.get(8)?.as_str().parse().ok()?;
    let end_meridiem = captures.get(9)?.as_str();

    let mut year = year_capture.unwrap_or(reference_et.year());
    let mut date = NaiveDate::from_ymd_opt(year, month, day)?;
    if year_capture.is_none() {
        let reference_date = reference_et.date_naive();
        if date < reference_date - ChronoDuration::days(1) {
            year += 1;
            date = NaiveDate::from_ymd_opt(year, month, day)?;
        }
    }

    let start_time = parse_ampm_time(start_hour, start_min, start_meridiem)?;
    let end_time = parse_ampm_time(end_hour, end_min, end_meridiem)?;

    let mut end_date = date;
    if end_time <= start_time {
        end_date = date.checked_add_signed(ChronoDuration::days(1))?;
    }

    let start_naive = date.and_time(start_time);
    let end_naive = end_date.and_time(end_time);
    let start_et = resolve_local_datetime(start_naive)?;
    let end_et = resolve_local_datetime(end_naive)?;

    let start_utc = start_et.with_timezone(&Utc);
    let end_utc = end_et.with_timezone(&Utc);
    let duration = end_utc - start_utc;
    if duration.num_minutes() != 15 {
        return None;
    }

    Some(MarketTimeWindow {
        start_time_ms: start_utc.timestamp_millis() as u64,
        end_time_ms: end_utc.timestamp_millis() as u64,
    })
}

fn time_range_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)(January|February|March|April|May|June|July|August|September|October|November|December|Jan|Feb|Mar|Apr|Jun|Jul|Aug|Sep|Sept|Oct|Nov|Dec)\.?\s+(\d{1,2})(?:,?\s*(\d{4}))?,?\s*(\d{1,2}):(\d{2})\s*([AP]M)\s*(?:ET|EST|EDT)?\s*[-–]\s*(\d{1,2}):(\d{2})\s*([AP]M)\s*(?:ET|EST|EDT)?",
        )
        .expect("valid time range regex")
    })
}

fn parse_month(value: &str) -> Option<u32> {
    let normalized = value.trim_end_matches('.').to_ascii_lowercase();
    match normalized.as_str() {
        "jan" | "january" => Some(1),
        "feb" | "february" => Some(2),
        "mar" | "march" => Some(3),
        "apr" | "april" => Some(4),
        "may" => Some(5),
        "jun" | "june" => Some(6),
        "jul" | "july" => Some(7),
        "aug" | "august" => Some(8),
        "sep" | "sept" | "september" => Some(9),
        "oct" | "october" => Some(10),
        "nov" | "november" => Some(11),
        "dec" | "december" => Some(12),
        _ => None,
    }
}

fn parse_ampm_time(hour: u32, minute: u32, meridiem: &str) -> Option<NaiveTime> {
    if hour == 0 || hour > 12 || minute > 59 {
        return None;
    }
    let is_pm = meridiem.eq_ignore_ascii_case("pm");
    let mut hour_24 = hour % 12;
    if is_pm {
        hour_24 += 12;
    }
    NaiveTime::from_hms_opt(hour_24, minute, 0)
}

fn resolve_local_datetime(naive: NaiveDateTime) -> Option<chrono::DateTime<chrono_tz::Tz>> {
    match New_York.from_local_datetime(&naive) {
        LocalResult::Single(value) => Some(value),
        LocalResult::Ambiguous(earliest, _) => Some(earliest),
        LocalResult::None => None,
    }
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
    let candidates = [
        "feeRateBps",
        "fee_rate_bps",
        "feeBps",
        "fee_bps",
        "fee",
        "makerFeeRateBps",
        "takerFeeRateBps",
    ];
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
    let candidates = [
        "minTickSize",
        "min_tick_size",
        "tickSize",
        "tick_size",
        "orderPriceMinTickSize",
    ];
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

fn log_prefix() -> String {
    let now = Utc::now();
    let et = now.with_timezone(&New_York);
    format!("[{}]", et.format("%H:%M:%S"))
}

fn format_window_et(window: &MarketTimeWindow) -> String {
    let start = Utc.timestamp_millis_opt(window.start_time_ms as i64).single();
    let end = Utc.timestamp_millis_opt(window.end_time_ms as i64).single();
    match (start, end) {
        (Some(start), Some(end)) => {
            let start_et = start.with_timezone(&New_York);
            let end_et = end.with_timezone(&New_York);
            format!(
                "{}-{} ET",
                start_et.format("%b %d %I:%M%p"),
                end_et.format("%I:%M%p")
            )
        }
        _ => "unknown".to_string(),
    }
}

fn pick_asset_window(now_ms: u64, windows: &[MarketCandidate]) -> Option<MarketCandidate> {
    let mut active: Option<&MarketCandidate> = None;
    let mut next: Option<&MarketCandidate> = None;

    for candidate in windows {
        let window = candidate.window;
        if now_ms >= window.start_time_ms && now_ms < window.end_time_ms {
            match active {
                Some(current) if current.window.end_time_ms <= window.end_time_ms => {}
                _ => active = Some(candidate),
            }
        } else if window.start_time_ms > now_ms {
            match next {
                Some(current) if current.window.start_time_ms <= window.start_time_ms => {}
                _ => next = Some(candidate),
            }
        }
    }

    active.or(next).cloned()
}

fn to_market_window(window: MarketTimeWindow) -> MarketWindow {
    MarketWindow {
        start_time_ms: window.start_time_ms,
        end_time_ms: window.end_time_ms,
    }
}

fn now_ms() -> Option<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
    Some(now.as_millis() as u64)
}

fn compute_jitter(max_ms: u64) -> Result<Duration> {
    if max_ms == 0 {
        return Ok(Duration::from_millis(0));
    }
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let jitter = now.subsec_millis() as u64 % (max_ms + 1);
    Ok(Duration::from_millis(jitter))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_15m_window_from_title() {
        let reference = New_York
            .with_ymd_and_hms(2026, 1, 23, 12, 0, 0)
            .single()
            .unwrap();
        let title = "Bitcoin Up or Down - January 24, 5:45PM-6:00PM ET";
        let window = parse_time_window_from_text(title, reference).expect("window");
        let start_et = New_York
            .with_ymd_and_hms(2026, 1, 24, 17, 45, 0)
            .single()
            .unwrap();
        let end_et = New_York
            .with_ymd_and_hms(2026, 1, 24, 18, 0, 0)
            .single()
            .unwrap();
        assert_eq!(
            window.start_time_ms,
            start_et.with_timezone(&Utc).timestamp_millis() as u64
        );
        assert_eq!(
            window.end_time_ms,
            end_et.with_timezone(&Utc).timestamp_millis() as u64
        );
    }

    #[test]
    fn rejects_non_15m_range() {
        let reference = New_York
            .with_ymd_and_hms(2026, 1, 23, 12, 0, 0)
            .single()
            .unwrap();
        let title = "Ethereum Up or Down - January 24, 5:00PM-6:00PM ET";
        assert!(parse_time_window_from_text(title, reference).is_none());
    }

    #[test]
    fn rolls_year_forward_when_missing_year_before_reference() {
        let reference = New_York
            .with_ymd_and_hms(2025, 12, 31, 23, 0, 0)
            .single()
            .unwrap();
        let title = "Solana Up or Down - January 1, 12:00AM-12:15AM ET";
        let window = parse_time_window_from_text(title, reference).expect("window");
        let start_et = New_York
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
            .single()
            .unwrap();
        let end_et = New_York
            .with_ymd_and_hms(2026, 1, 1, 0, 15, 0)
            .single()
            .unwrap();
        assert_eq!(
            window.start_time_ms,
            start_et.with_timezone(&Utc).timestamp_millis() as u64
        );
        assert_eq!(
            window.end_time_ms,
            end_et.with_timezone(&Utc).timestamp_millis() as u64
        );
    }

    #[test]
    fn parses_abbreviated_month_and_spaced_range() {
        let reference = New_York
            .with_ymd_and_hms(2026, 1, 23, 12, 0, 0)
            .single()
            .unwrap();
        let title = "BTC Up or Down - Jan 24, 9:15AM - 9:30AM ET";
        let window = parse_time_window_from_text(title, reference).expect("window");
        let start_et = New_York
            .with_ymd_and_hms(2026, 1, 24, 9, 15, 0)
            .single()
            .unwrap();
        let end_et = New_York
            .with_ymd_and_hms(2026, 1, 24, 9, 30, 0)
            .single()
            .unwrap();
        assert_eq!(
            window.start_time_ms,
            start_et.with_timezone(&Utc).timestamp_millis() as u64
        );
        assert_eq!(
            window.end_time_ms,
            end_et.with_timezone(&Utc).timestamp_millis() as u64
        );
    }
}
