/**
 * @purpose
 * TimescaleDB/Postgres connection manager and persistence helpers.
 *
 * @dependencies
 * - sqlx: async Postgres pool and query execution
 *
 * @notes
 * - Callers should reuse the pool for all queries.
 */
use serde_json::Value;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::error::Result;

#[derive(Debug, Clone)]
pub struct TradeExecutionLog {
    pub market_id: String,
    pub rail: String,
    pub mode: String,
    pub expected_ev: f64,
    pub actual_pnl: Option<f64>,
    pub fees_paid: f64,
    pub latency_ms: Option<u64>,
    pub metadata: Option<Value>,
}

#[derive(Clone)]
pub struct DatabaseManager {
    pool: PgPool,
}

impl DatabaseManager {
    pub async fn new(database_url: &str, max_connections: u32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Persist a trade execution log entry to TimescaleDB.
    pub async fn log_trade_execution(&self, entry: &TradeExecutionLog) -> Result<()> {
        let metadata = entry
            .metadata
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        let latency_ms = entry.latency_ms.map(|value| value as i64);

        sqlx::query(
            r#"
            INSERT INTO trade_logs (
                market_id,
                rail,
                mode,
                expected_ev,
                actual_pnl,
                fees_paid,
                latency_ms,
                metadata
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb)
            "#,
        )
        .bind(&entry.market_id)
        .bind(&entry.rail)
        .bind(&entry.mode)
        .bind(entry.expected_ev)
        .bind(entry.actual_pnl)
        .bind(entry.fees_paid)
        .bind(latency_ms)
        .bind(metadata)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
