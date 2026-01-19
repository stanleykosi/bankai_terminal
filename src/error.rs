/**
 * @description
 * Centralized error definitions for the Bankai Terminal core.
 *
 * @dependencies
 * - thiserror: derives standard error implementations
 *
 * @notes
 * - Extend with domain-specific error variants as modules evolve.
 */
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BankaiError {
    #[error("config header missing closing marker")]
    InvalidHeader,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("notify error: {0}")]
    Notify(#[from] notify::Error),
    #[error("watch channel error: {0}")]
    WatchChannel(#[from] std::sync::mpsc::RecvError),
    #[error("crypto error: {0}")]
    Crypto(String),
    #[error("argon2 error: {0}")]
    Argon2(#[from] argon2::Error),
    #[error("base64 error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("redis error: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("python error: {0}")]
    Python(#[from] pyo3::PyErr),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

pub type Result<T> = std::result::Result<T, BankaiError>;
