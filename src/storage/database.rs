/**
 * @description
 * TimescaleDB/Postgres connection manager and persistence helpers.
 *
 * @dependencies
 * - sqlx: async Postgres pool and query execution
 *
 * @notes
 * - Callers should reuse the pool for all queries.
 */
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::error::Result;

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
}
