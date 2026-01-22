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
}
