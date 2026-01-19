/**
 * @description
 * Initial TimescaleDB schema for trade logs and regret metrics.
 *
 * @dependencies
 * - timescaledb extension
 * - pgcrypto extension for UUID generation
 *
 * @notes
 * - Run via `sqlx migrate run`.
 */
CREATE EXTENSION IF NOT EXISTS timescaledb;
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS trade_logs (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    time TIMESTAMPTZ NOT NULL DEFAULT now(),
    market_id TEXT NOT NULL,
    rail TEXT NOT NULL,
    mode TEXT NOT NULL,
    expected_ev NUMERIC NOT NULL,
    actual_pnl NUMERIC,
    fees_paid NUMERIC NOT NULL,
    latency_ms BIGINT,
    metadata JSONB
);

CREATE TABLE IF NOT EXISTS regret_metrics (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    time TIMESTAMPTZ NOT NULL DEFAULT now(),
    market_id TEXT NOT NULL,
    strategy TEXT NOT NULL,
    expected_ev NUMERIC NOT NULL,
    actual_pnl NUMERIC,
    regret_value NUMERIC NOT NULL,
    metadata JSONB
);

ALTER TABLE IF EXISTS trade_logs
    DROP CONSTRAINT IF EXISTS trade_logs_pkey,
    ADD PRIMARY KEY (time, id);

ALTER TABLE IF EXISTS regret_metrics
    DROP CONSTRAINT IF EXISTS regret_metrics_pkey,
    ADD PRIMARY KEY (time, id);

SELECT create_hypertable('trade_logs', 'time', if_not_exists => TRUE);
SELECT create_hypertable('regret_metrics', 'time', if_not_exists => TRUE);
