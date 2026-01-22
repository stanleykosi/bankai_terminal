/**
 * @description
 * Redis-backed nonce manager with atomic reservation and replace-by-fee tracking.
 *
 * @dependencies
 * - redis: Lua scripting and async commands
 * - serde: serialization for pending nonce payloads
 * - uuid: mutex token generation
 *
 * @notes
 * - Uses Redis Lua scripts for atomic nonce reservation and bump updates.
 * - Pending nonces are tracked for RBF-style resubmission decisions.
 */
use redis::AsyncCommands;
use redis::Script;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::error::{BankaiError, Result};
use crate::storage::redis::RedisManager;

const NONCE_KEY_PREFIX: &str = "sys:nonce:";
const PENDING_SUFFIX: &str = ":pending";
const LOCK_SUFFIX: &str = ":lock";

const RESERVE_NONCE_SCRIPT: &str = r#"
local key = KEYS[1]
local pending_key = KEYS[2]
local now_ms = tonumber(ARGV[1])
local current = redis.call("GET", key)
if not current then
  return {err="nonce_not_initialized"}
end
local nonce = tonumber(current)
redis.call("SET", key, nonce + 1)
local payload = cjson.encode({first_sent_ms = now_ms, last_sent_ms = now_ms, bump_count = 0})
redis.call("HSET", pending_key, nonce, payload)
return nonce
"#;

const BUMP_NONCE_SCRIPT: &str = r#"
local pending_key = KEYS[1]
local nonce = ARGV[1]
local now_ms = tonumber(ARGV[2])
local raw = redis.call("HGET", pending_key, nonce)
if not raw then
  return nil
end
local payload = cjson.decode(raw)
payload.last_sent_ms = now_ms
payload.bump_count = payload.bump_count + 1
local encoded = cjson.encode(payload)
redis.call("HSET", pending_key, nonce, encoded)
return encoded
"#;

const RELEASE_LOCK_SCRIPT: &str = r#"
if redis.call("GET", KEYS[1]) == ARGV[1] then
  return redis.call("DEL", KEYS[1])
end
return 0
"#;

#[derive(Debug, Clone)]
pub struct NonceManagerConfig {
    pub stuck_timeout_ms: u64,
    pub bump_cooldown_ms: u64,
    pub lock_ttl_ms: u64,
}

impl Default for NonceManagerConfig {
    fn default() -> Self {
        Self {
            stuck_timeout_ms: 15_000,
            bump_cooldown_ms: 3_000,
            lock_ttl_ms: 2_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PendingNoncePayload {
    first_sent_ms: u64,
    last_sent_ms: u64,
    bump_count: u32,
}

#[derive(Debug, Clone)]
pub struct PendingNonceEntry {
    pub nonce: u64,
    pub first_sent_ms: u64,
    pub last_sent_ms: u64,
    pub bump_count: u32,
}

impl PendingNonceEntry {
    fn from_payload(nonce: u64, payload: PendingNoncePayload) -> Self {
        Self {
            nonce,
            first_sent_ms: payload.first_sent_ms,
            last_sent_ms: payload.last_sent_ms,
            bump_count: payload.bump_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NonceManager {
    redis: RedisManager,
    address: String,
    config: NonceManagerConfig,
}

impl NonceManager {
    pub fn new(
        redis: RedisManager,
        address: impl Into<String>,
        config: NonceManagerConfig,
    ) -> Result<Self> {
        let address = normalize_address(address.into())?;
        Ok(Self {
            redis,
            address,
            config,
        })
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn config(&self) -> &NonceManagerConfig {
        &self.config
    }

    pub async fn initialize_if_missing(&self, next_nonce: u64) -> Result<bool> {
        let key = nonce_key(&self.address);
        let mut conn = self.redis.connection();
        Ok(conn.set_nx(key, next_nonce).await?)
    }

    pub async fn set_next_nonce(&self, next_nonce: u64) -> Result<()> {
        let lock = self.lock();
        if !lock.acquire().await? {
            return Err(BankaiError::InvalidArgument("nonce lock busy".to_string()));
        }
        let result = self.set_next_nonce_unlocked(next_nonce).await;
        lock.release().await?;
        result
    }

    pub async fn get_next_nonce(&self) -> Result<Option<u64>> {
        let key = nonce_key(&self.address);
        let mut conn = self.redis.connection();
        Ok(conn.get(key).await?)
    }

    pub async fn reserve_nonce(&self, now_ms: u64) -> Result<u64> {
        let key = nonce_key(&self.address);
        let pending_key = pending_key(&self.address);
        let mut conn = self.redis.connection();
        let script = Script::new(RESERVE_NONCE_SCRIPT);
        let nonce: i64 = script
            .key(key)
            .key(pending_key)
            .arg(now_ms)
            .invoke_async(&mut conn)
            .await?;
        if nonce < 0 {
            return Err(BankaiError::InvalidArgument(
                "nonce reservation returned negative value".to_string(),
            ));
        }
        Ok(nonce as u64)
    }

    pub async fn mark_confirmed(&self, nonce: u64) -> Result<()> {
        let pending_key = pending_key(&self.address);
        let mut conn = self.redis.connection();
        conn.hdel::<_, _, ()>(pending_key, nonce).await?;
        Ok(())
    }

    pub async fn record_bump(&self, nonce: u64, now_ms: u64) -> Result<Option<PendingNonceEntry>> {
        let pending_key = pending_key(&self.address);
        let mut conn = self.redis.connection();
        let script = Script::new(BUMP_NONCE_SCRIPT);
        let updated: Option<String> = script
            .key(pending_key)
            .arg(nonce)
            .arg(now_ms)
            .invoke_async(&mut conn)
            .await?;
        let Some(raw) = updated else {
            return Ok(None);
        };
        let payload: PendingNoncePayload = serde_json::from_str(&raw)?;
        Ok(Some(PendingNonceEntry::from_payload(nonce, payload)))
    }

    pub async fn pending_nonces(&self) -> Result<Vec<PendingNonceEntry>> {
        let pending_key = pending_key(&self.address);
        let mut conn = self.redis.connection();
        let entries: HashMap<String, String> = conn.hgetall(pending_key).await?;
        let mut results = Vec::with_capacity(entries.len());
        for (nonce_str, raw) in entries {
            let nonce = nonce_str.parse::<u64>().map_err(|_| {
                BankaiError::InvalidArgument(format!("invalid nonce value {nonce_str}"))
            })?;
            let payload: PendingNoncePayload = serde_json::from_str(&raw)?;
            results.push(PendingNonceEntry::from_payload(nonce, payload));
        }
        Ok(results)
    }

    pub async fn stale_nonces(&self, now_ms: u64) -> Result<Vec<PendingNonceEntry>> {
        let entries = self.pending_nonces().await?;
        let mut stale = Vec::new();
        for entry in entries {
            if should_bump(&self.config, now_ms, &entry) {
                stale.push(entry);
            }
        }
        Ok(stale)
    }

    fn lock(&self) -> RedisMutex {
        let key = lock_key(&self.address);
        RedisMutex::new(self.redis.clone(), key, self.config.lock_ttl_ms)
    }

    async fn set_next_nonce_unlocked(&self, next_nonce: u64) -> Result<()> {
        let key = nonce_key(&self.address);
        let mut conn = self.redis.connection();
        conn.set::<_, _, ()>(key, next_nonce).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct RedisMutex {
    redis: RedisManager,
    key: String,
    token: String,
    ttl_ms: u64,
}

impl RedisMutex {
    fn new(redis: RedisManager, key: String, ttl_ms: u64) -> Self {
        Self {
            redis,
            key,
            token: uuid::Uuid::new_v4().to_string(),
            ttl_ms,
        }
    }

    async fn acquire(&self) -> Result<bool> {
        let mut conn = self.redis.connection();
        let result: Option<String> = redis::cmd("SET")
            .arg(&self.key)
            .arg(&self.token)
            .arg("NX")
            .arg("PX")
            .arg(self.ttl_ms)
            .query_async(&mut conn)
            .await?;
        Ok(result.is_some())
    }

    async fn release(&self) -> Result<()> {
        let mut conn = self.redis.connection();
        let script = Script::new(RELEASE_LOCK_SCRIPT);
        let _: i32 = script
            .key(&self.key)
            .arg(&self.token)
            .invoke_async(&mut conn)
            .await?;
        Ok(())
    }
}

fn normalize_address(address: String) -> Result<String> {
    let trimmed = address.trim().to_ascii_lowercase();
    if trimmed.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "address is required for nonce manager".to_string(),
        ));
    }
    Ok(trimmed)
}

fn nonce_key(address: &str) -> String {
    format!("{NONCE_KEY_PREFIX}{address}")
}

fn pending_key(address: &str) -> String {
    format!("{NONCE_KEY_PREFIX}{address}{PENDING_SUFFIX}")
}

fn lock_key(address: &str) -> String {
    format!("{NONCE_KEY_PREFIX}{address}{LOCK_SUFFIX}")
}

fn should_bump(config: &NonceManagerConfig, now_ms: u64, entry: &PendingNonceEntry) -> bool {
    let stuck_for_ms = now_ms.saturating_sub(entry.first_sent_ms);
    let since_last_send_ms = now_ms.saturating_sub(entry.last_sent_ms);
    stuck_for_ms >= config.stuck_timeout_ms && since_last_send_ms >= config.bump_cooldown_ms
}
