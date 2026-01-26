/**
 * @description
 * Trade opportunity analysis for implied vs true probabilities and intent selection.
 *
 * @dependencies
 * - Optional Redis lookup helper for market time windows.
 *
 * @notes
 * - Enforces snipe guard: edge must clear taker fees + gas + 2%.
 */
use crate::config::{FeeConfig, StrategyConfig};
use crate::engine::types::{MarketWindow, TradeIntent, TradeMode};
use crate::error::{BankaiError, Result};
use crate::storage::redis::RedisManager;

const SNIPE_EDGE_BUFFER_BPS: f64 = 200.0;

#[derive(Debug, Clone)]
pub struct AnalysisInput {
    pub market_id: String,
    pub asset_id: String,
    pub implied_prob: f64,
    pub true_prob: f64,
    pub timestamp_ms: u64,
    pub market_window: Option<MarketWindow>,
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
    OutOfWindow,
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

    if let Some(window) = input.market_window {
        validate_market_window(window)?;
        if !is_within_window(input.timestamp_ms, window) {
            return Ok(AnalysisResult {
                implied_prob,
                true_prob,
                edge,
                edge_bps,
                decision: TradeDecision::OutOfWindow,
                intent: None,
            });
        }
    }

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
            market_window: input.market_window,
        }),
        TradeDecision::NoEdge | TradeDecision::OutOfWindow => None,
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

pub async fn analyze_opportunity_with_redis(
    mut input: AnalysisInput,
    strategy: &StrategyConfig,
    fees: &FeeConfig,
    redis: &RedisManager,
) -> Result<AnalysisResult> {
    if input.market_window.is_none() {
        input.market_window = redis.get_market_window(&input.market_id).await?;
    }
    analyze_opportunity(input, strategy, fees)
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

fn validate_market_window(window: MarketWindow) -> Result<()> {
    if window.end_time_ms <= window.start_time_ms {
        return Err(BankaiError::InvalidArgument(
            "market window end time must be after start time".to_string(),
        ));
    }
    Ok(())
}

fn is_within_window(timestamp_ms: u64, window: MarketWindow) -> bool {
    timestamp_ms >= window.start_time_ms && timestamp_ms <= window.end_time_ms
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::types::TradeMode;

    fn base_strategy() -> StrategyConfig {
        StrategyConfig {
            kelly_fraction: 0.25,
            snipe_min_edge_bps: 500.0,
            spread_offset_bps: 15.0,
        }
    }

    fn base_fees() -> FeeConfig {
        FeeConfig {
            taker_fee_bps: 40.0,
            estimated_gas_bps: 25.0,
        }
    }

    fn sample_input(implied_prob: f64, true_prob: f64) -> AnalysisInput {
        AnalysisInput {
            market_id: "m1".to_string(),
            asset_id: "a1".to_string(),
            implied_prob,
            true_prob,
            timestamp_ms: 1_717_171_717,
            market_window: None,
        }
    }

    #[test]
    fn it_rejects_invalid_probabilities() {
        let err = analyze_opportunity(sample_input(1.2, 0.5), &base_strategy(), &base_fees())
            .unwrap_err();
        match err {
            BankaiError::InvalidArgument(msg) => assert!(msg.contains("implied_prob")),
            _ => panic!("unexpected error variant"),
        }
    }

    #[test]
    fn it_returns_no_edge_when_true_prob_lower() {
        let result = analyze_opportunity(sample_input(0.6, 0.4), &base_strategy(), &base_fees())
            .expect("analysis result");
        assert_eq!(result.decision, TradeDecision::NoEdge);
        assert!(result.intent.is_none());
        assert!(result.edge < 0.0);
    }

    #[test]
    fn it_returns_ladder_when_edge_below_snipe_floor() {
        let result =
            analyze_opportunity(sample_input(0.45, 0.495), &base_strategy(), &base_fees()).unwrap();
        assert_eq!(result.decision, TradeDecision::Ladder);

        let intent = result.intent.expect("ladder intent");
        assert_eq!(intent.mode, TradeMode::Ladder);
        assert!(result.edge_bps < snipe_threshold_bps(&base_strategy(), &base_fees()));
    }

    #[test]
    fn it_returns_snipe_when_edge_exceeds_guard() {
        let result =
            analyze_opportunity(sample_input(0.2, 0.55), &base_strategy(), &base_fees()).unwrap();
        assert_eq!(result.decision, TradeDecision::Snipe);

        let intent = result.intent.expect("snipe intent");
        assert_eq!(intent.mode, TradeMode::Snipe);
        assert!(result.edge_bps >= snipe_threshold_bps(&base_strategy(), &base_fees()));
    }

    #[test]
    fn it_uses_maximum_threshold_between_strategy_and_fees() {
        let strategy = StrategyConfig {
            kelly_fraction: 0.25,
            snipe_min_edge_bps: 150.0,
            spread_offset_bps: 10.0,
        };
        let fees = FeeConfig {
            taker_fee_bps: 80.0,
            estimated_gas_bps: 50.0,
        };

        let threshold = snipe_threshold_bps(&strategy, &fees);
        assert_eq!(threshold, 330.0);
    }

    #[test]
    fn it_skips_when_outside_market_window() {
        let input = AnalysisInput {
            market_id: "m1".to_string(),
            asset_id: "a1".to_string(),
            implied_prob: 0.4,
            true_prob: 0.6,
            timestamp_ms: 1_000,
            market_window: Some(MarketWindow {
                start_time_ms: 2_000,
                end_time_ms: 3_000,
            }),
        };

        let result = analyze_opportunity(input, &base_strategy(), &base_fees()).unwrap();
        assert_eq!(result.decision, TradeDecision::OutOfWindow);
        assert!(result.intent.is_none());
    }
}
