/**
 * @description
 * Telemetry modules for system health and metrics.
 *
 * @dependencies
 * - None (module declarations only)
 *
 * @notes
 * - Health monitors should not block the trading engine.
 */
pub mod health;
pub mod logging;
pub mod metrics;
