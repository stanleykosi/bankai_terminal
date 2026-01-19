/**
 * @description
 * Metrics helpers for critical KPIs (latency, fill rate, rail failovers).
 *
 * @dependencies
 * - metrics: recorder-agnostic metric macros
 *
 * @notes
 * - Exporters must be configured elsewhere to collect these metrics.
 */
const LATENCY_MS: &str = "latency_ms";
const ORDER_FILL_RATE: &str = "order_fill_rate";
const RAIL_FAILOVER_COUNT: &str = "rail_failover_count";

pub fn init_metrics() {
    metrics::describe_histogram!(LATENCY_MS, "End-to-end latency in milliseconds.");
    metrics::describe_gauge!(ORDER_FILL_RATE, "Order fill rate percentage.");
    metrics::describe_counter!(RAIL_FAILOVER_COUNT, "Count of rail failovers.");
}

pub fn record_latency_ms(value_ms: f64) {
    metrics::histogram!(LATENCY_MS, value_ms);
}

pub fn record_order_fill_rate(rate_pct: f64) {
    metrics::gauge!(ORDER_FILL_RATE, rate_pct);
}

pub fn increment_rail_failover() {
    metrics::counter!(RAIL_FAILOVER_COUNT, 1);
}
