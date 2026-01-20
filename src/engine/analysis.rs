/**
 * @description
 * Trade opportunity analysis for implied vs true probabilities and intent selection.
 *
 * @dependencies
 * - None (pure computations)
 *
 * @notes
 * - Enforces snipe guard: edge must clear taker fees + gas + 2%.
 */
use crate::config::{FeeConfig, StrategyConfig};
use crate::engine::types::{TradeIntent, TradeMode};
use crate::error::{BankaiError, Result};

const SNIPE_EDGE_BUFFER_BPS: f64 = 200.0;

#[derive(Debug, Clone)]
pub struct AnalysisInput {
    pub market_id: String,
    pub asset_id: String,
    pub implied_prob: f64,
    pub true_prob: f64,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub implied_prob: f64,
    pub true_prob: f64,
    pub edge: f64,
    pub edge_bps: f64,
    pub decision: TradeDecision,
    pub intent: Option<TradeIntent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TradeDecision {
    NoEdge,
    Ladder,
    Snipe,
}

pub fn analyze_opportunity(
    input: AnalysisInput,
    strategy: &StrategyConfig,
    fees: &FeeConfig,
) -> Result<AnalysisResult> {
    let implied_prob = validate_probability(input.implied_prob, "implied_prob")?;
    let true_prob = validate_probability(input.true_prob, "true_prob")?;
    let edge = true_prob - implied_prob;
    let edge_bps = edge * 10_000.0;

    let decision = if edge_bps <= 0.0 {
        TradeDecision::NoEdge
    } else {
        let snipe_floor_bps = snipe_threshold_bps(strategy, fees);
        if edge_bps >= snipe_floor_bps {
            TradeDecision::Snipe
        } else {
            TradeDecision::Ladder
        }
    };

    let intent = match decision {
        TradeDecision::Ladder | TradeDecision::Snipe => Some(TradeIntent {
            market_id: input.market_id,
            asset_id: input.asset_id,
            mode: match decision {
                TradeDecision::Snipe => TradeMode::Snipe,
                _ => TradeMode::Ladder,
            },
            implied_prob,
            true_prob,
            edge,
            edge_bps,
            spread_offset_bps: strategy.spread_offset_bps,
            timestamp_ms: input.timestamp_ms,
        }),
        TradeDecision::NoEdge => None,
    };

    Ok(AnalysisResult {
        implied_prob,
        true_prob,
        edge,
        edge_bps,
        decision,
        intent,
    })
}

pub fn implied_probability_from_price(price: f64) -> Result<f64> {
    validate_probability(price, "price")
}

pub fn implied_probability_from_decimal_odds(odds: f64) -> Result<f64> {
    if odds <= 1.0 {
        return Err(BankaiError::InvalidArgument(
            "decimal odds must be > 1.0".to_string(),
        ));
    }
    let implied = 1.0 / odds;
    validate_probability(implied, "implied_prob")
}

pub fn true_probability_from_allora(value: f64) -> Result<f64> {
    validate_probability(value, "allora_inference")
}

pub fn snipe_threshold_bps(strategy: &StrategyConfig, fees: &FeeConfig) -> f64 {
    let fee_guard = fees.taker_fee_bps + fees.estimated_gas_bps + SNIPE_EDGE_BUFFER_BPS;
    strategy.snipe_min_edge_bps.max(fee_guard)
}

fn validate_probability(value: f64, field: &str) -> Result<f64> {
    if !(0.0..=1.0).contains(&value) {
        return Err(BankaiError::InvalidArgument(format!(
            "{field} must be within [0, 1]"
        )));
    }
    Ok(value)
}
