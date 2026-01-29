/**
 * @description
 * Shared engine data types for cross-module messaging.
 *
 * @dependencies
 * - None
 *
 * @notes
 * - Keep payloads lightweight for hot-path delivery.
 */
#[derive(Debug, Clone)]
pub enum MarketUpdate {
    Chainlink(ChainlinkMarketUpdate),
    Allora(AlloraMarketUpdate),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeMode {
    Ladder,
    Snipe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub struct MarketWindow {
    pub start_time_ms: u64,
    pub end_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct TradeIntent {
    pub market_id: String,
    pub asset_id: String,
    pub side: TradeSide,
    pub mode: TradeMode,
    pub implied_prob: f64,
    pub true_prob: f64,
    pub edge: f64,
    pub edge_bps: f64,
    pub spread_offset_bps: f64,
    pub timestamp_ms: u64,
    pub market_window: Option<MarketWindow>,
    pub requested_size: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ChainlinkMarketUpdate {
    pub asset: String,
    pub last_price: Option<f64>,
    pub volatility_1m: Option<f64>,
    pub dfo: Option<f64>,
    pub event_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct AlloraMarketUpdate {
    pub topic_id: u64,
    pub inference_value: f64,
    pub inference_raw: Option<String>,
    pub token_decimals: Option<u32>,
    pub signature: Option<String>,
    pub request_id: Option<String>,
    pub confidence_intervals: Vec<f64>,
    pub signal_timestamp_ms: u64,
    pub received_at_ms: u64,
    pub asset: String,
    pub timeframe: String,
}
