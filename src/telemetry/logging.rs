/**
 * @description
 * Structured logging helpers with trace IDs for correlation.
 *
 * @dependencies
 * - tracing: structured logging
 * - tracing-subscriber: subscriber configuration
 * - uuid: trace ID generation
 *
 * @notes
 * - Attach trace_id to spans to propagate through logs.
 */
use tracing::Span;
use tracing_subscriber::{fmt, EnvFilter};
use uuid::Uuid;

pub struct TraceContext {
    trace_id: String,
}

impl TraceContext {
    pub fn new() -> Self {
        Self {
            trace_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    pub fn span(&self, name: &'static str) -> Span {
        tracing::info_span!("trace", trace_id = %self.trace_id, span_name = %name)
    }

    pub fn trade_span(&self, market_id: &str) -> Span {
        tracing::info_span!("trade", trace_id = %self.trace_id, market_id = %market_id)
    }
}

pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_level(true)
        .init();
}
