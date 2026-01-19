/**
 * @description
 * Redis manager for hot state storage (order books, nonces, market metadata).
 *
 * @dependencies
 * - redis: async connection manager and command helpers
 *
 * @notes
 * - ConnectionManager handles reconnects and multiplexing.
 */
use redis::aio::ConnectionManager;
use redis::AsyncCommands;

use crate::error::Result;

#[derive(Clone)]
pub struct RedisManager {
    connection: ConnectionManager,
}

impl RedisManager {
    pub async fn new(redis_url: &str) -> Result<Self> {
        let client = redis::Client::open(redis_url)?;
        let connection = ConnectionManager::new(client).await?;
        Ok(Self { connection })
    }

    pub fn connection(&self) -> ConnectionManager {
        self.connection.clone()
    }

    pub async fn set_float(&self, key: &str, value: f64) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.set(key, value).await?;
        Ok(())
    }

    pub async fn get_float(&self, key: &str) -> Result<Option<f64>> {
        let mut conn = self.connection.clone();
        Ok(conn.get(key).await?)
    }

    pub async fn hset_float(&self, key: &str, field: &str, value: f64) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.hset(key, field, value).await?;
        Ok(())
    }

    pub async fn hget_float(&self, key: &str, field: &str) -> Result<Option<f64>> {
        let mut conn = self.connection.clone();
        Ok(conn.hget(key, field).await?)
    }

    pub async fn hdel(&self, key: &str, field: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.hdel(key, field).await?;
        Ok(())
    }

    pub async fn zadd(&self, key: &str, score: f64, member: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.zadd(key, member, score).await?;
        Ok(())
    }

    pub async fn zrem(&self, key: &str, member: &str) -> Result<()> {
        let mut conn = self.connection.clone();
        conn.zrem(key, member).await?;
        Ok(())
    }

    pub async fn zrange_with_scores(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> Result<Vec<(String, f64)>> {
        let mut conn = self.connection.clone();
        Ok(conn.zrange_withscores(key, start, stop).await?)
    }

    pub async fn zrevrange_with_scores(
        &self,
        key: &str,
        start: isize,
        stop: isize,
    ) -> Result<Vec<(String, f64)>> {
        let mut conn = self.connection.clone();
        Ok(conn.zrevrange_withscores(key, start, stop).await?)
    }
}
