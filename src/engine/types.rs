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
    Binance(BinanceMarketUpdate),
    Allora(AlloraMarketUpdate),
}

#[derive(Debug, Clone)]
pub struct BinanceMarketUpdate {
    pub asset: String,
    pub best_bid: Option<f64>,
    pub best_ask: Option<f64>,
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
