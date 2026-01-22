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
use std::fs::{self, OpenOptions};
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

use tracing::Span;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
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
    let writer = select_writer();

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_level(true)
        .with_writer(writer)
        .init();
}

fn select_writer() -> BoxMakeWriter {
    if tui_mode_enabled() {
        match file_writer() {
            Ok(writer) => writer,
            Err(error) => {
                eprintln!("logging fallback to stderr (failed to open log file): {error}");
                stderr_writer()
            }
        }
    } else {
        stderr_writer()
    }
}

fn file_writer() -> io::Result<BoxMakeWriter> {
    let path = log_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Validate the path once so we can fall back cleanly if it fails.
    OpenOptions::new().create(true).append(true).open(&path)?;

    Ok(BoxMakeWriter::new(move || {
        match OpenOptions::new().create(true).append(true).open(&path) {
            Ok(file) => Box::new(file) as Box<dyn Write + Send>,
            Err(_) => Box::new(io::stderr()) as Box<dyn Write + Send>,
        }
    }))
}

fn stderr_writer() -> BoxMakeWriter {
    BoxMakeWriter::new(|| Box::new(io::stderr()) as Box<dyn Write + Send>)
}

fn log_file_path() -> PathBuf {
    std::env::var("BANKAI_LOG_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target_local/logs/bankai.log"))
}

fn tui_mode_enabled() -> bool {
    if let Ok(value) = std::env::var("BANKAI_TUI") {
        if let Some(parsed) = parse_env_bool(&value) {
            return parsed;
        }
    }
    io::stdout().is_terminal()
}

fn parse_env_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}
