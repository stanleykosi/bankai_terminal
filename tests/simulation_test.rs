/**
 * @description
 * Simulation-style integration tests for oracle CSV inputs -> TradeIntent generation
 * and rail routing decisions with staleness guard enforcement.
 *
 * @dependencies
 * - tokio: async test runtime
 *
 * @notes
 * - Uses in-memory CSV fixtures to avoid network or disk I/O.
 */
use bankai_terminal::config::{FeeConfig, StrategyConfig};
use bankai_terminal::engine::analysis::{analyze_opportunity, AnalysisInput};
use bankai_terminal::engine::risk::evaluate_staleness;
use bankai_terminal::engine::types::{TradeIntent, TradeMode};
use bankai_terminal::execution::orchestrator::ExecutionRail;

#[derive(Debug, Clone)]
struct SimulationRow {
    asset: String,
    implied_prob: f64,
    true_prob: f64,
    signal_timestamp_ms: u64,
    candle_end_ms: u64,
    now_ms: u64,
    relayer_ok: bool,
}

fn fixture_csv() -> &'static str {
    "asset,implied_prob,true_prob,signal_ts,candle_end,now,relayer_ok
SOL,0.40,0.60,1_000,2_000,1_020,true
ETH,0.48,0.55,2_000,4_000,2_050,false
BTC,0.45,0.50,1_000,2_000,1_900,true"
}

fn parse_rows() -> Vec<SimulationRow> {
    fixture_csv()
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 7 {
                return None;
            }
            let asset = parts[0].trim().to_string();
            let implied_prob = parts[1].replace('_', "").parse().ok()?;
            let true_prob = parts[2].replace('_', "").parse().ok()?;
            let signal_timestamp_ms = parts[3].replace('_', "").parse().ok()?;
            let candle_end_ms = parts[4].replace('_', "").parse().ok()?;
            let now_ms = parts[5].replace('_', "").parse().ok()?;
            let relayer_ok = parts[6].trim().eq_ignore_ascii_case("true");
            Some(SimulationRow {
                asset,
                implied_prob,
                true_prob,
                signal_timestamp_ms,
                candle_end_ms,
                now_ms,
                relayer_ok,
            })
        })
        .collect()
}

fn base_strategy() -> StrategyConfig {
    StrategyConfig {
        kelly_fraction: 0.25,
        snipe_min_edge_bps: 500.0,
        spread_offset_bps: 10.0,
    }
}

fn base_fees() -> FeeConfig {
    FeeConfig {
        taker_fee_bps: 35.0,
        estimated_gas_bps: 30.0,
    }
}

fn build_intent(row: &SimulationRow, max_staleness_ratio: f64) -> Option<TradeIntent> {
    let staleness = evaluate_staleness(
        row.now_ms,
        row.signal_timestamp_ms,
        row.candle_end_ms,
        max_staleness_ratio,
    )
    .ok()?;
    if staleness.is_stale {
        return None;
    }

    let analysis = analyze_opportunity(
        AnalysisInput {
            market_id: format!("{}-market", row.asset),
            asset_id: row.asset.clone(),
            implied_prob: row.implied_prob,
            true_prob: row.true_prob,
            timestamp_ms: row.now_ms,
            market_window: None,
        },
        &base_strategy(),
        &base_fees(),
    )
    .ok()?;

    analysis.intent
}

#[derive(Clone, Copy)]
struct StubRails {
    relayer_ok: bool,
}

impl StubRails {
    async fn route(&self, intent: &TradeIntent) -> ExecutionRail {
        match (self.relayer_ok, intent.mode) {
            (true, _) => ExecutionRail::Relayer,
            (false, TradeMode::Snipe | TradeMode::Ladder) => ExecutionRail::Direct,
        }
    }
}

#[tokio::test]
async fn it_generates_intents_and_routes_between_rails() {
    let rows = parse_rows();
    let max_staleness_ratio = 0.05;

    let intents: Vec<_> = rows
        .iter()
        .filter_map(|row| {
            build_intent(row, max_staleness_ratio).map(|intent| (intent, row.relayer_ok))
        })
        .collect();

    assert_eq!(intents.len(), 2, "stale rows should be filtered out");
    assert!(intents
        .iter()
        .all(|(intent, _)| intent.mode == TradeMode::Snipe));

    let (intent_a, relayer_ok_a) = &intents[0];
    let rail_a = StubRails {
        relayer_ok: *relayer_ok_a,
    }
    .route(intent_a)
    .await;
    assert_eq!(
        rail_a,
        ExecutionRail::Relayer,
        "relayer path should succeed"
    );

    let (intent_b, relayer_ok_b) = &intents[1];
    let rail_b = StubRails {
        relayer_ok: *relayer_ok_b,
    }
    .route(intent_b)
    .await;
    assert_eq!(
        rail_b,
        ExecutionRail::Direct,
        "relayer failure should trigger direct rail"
    );
}

#[test]
fn it_rejects_stale_signals_from_fixture() {
    let rows = parse_rows();
    let stale_row = rows.last().expect("fixture has stale row");
    let intent = build_intent(stale_row, 0.05);
    assert!(
        intent.is_none(),
        "staleness guard should drop intents past threshold"
    );
}
