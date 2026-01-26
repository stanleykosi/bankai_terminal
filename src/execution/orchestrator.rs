/**
 * @purpose
 * Execution orchestrator that routes TradeIntents through Relayer (Rail A) with
 * failover to Direct (Rail B), persisting outcomes to TimescaleDB.
 *
 * @dependencies
 * - tokio: async task and timeout handling
 * - serde_json: structured metadata logging
 * - tracing: execution path logging
 *
 * @notes
 * - Requires an ExecutionPayloadBuilder to translate TradeIntent into rail payloads.
 * - Relayer timeouts trigger failover; relayer errors marked as failover-safe do as well.
 */
use serde_json::{json, Map, Value};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

use crate::engine::types::{TradeIntent, TradeMode, TradeSide};
use crate::error::{BankaiError, Result};
use crate::execution::direct::{DirectExecutionClient, DirectExecutionResult, FillOrdersRequest};
use crate::execution::nonce::NonceManager;
use crate::execution::relayer::{
    RelayerAuth, RelayerClient, RelayerError, RelayerErrorKind, RelayerResponse,
};
use crate::storage::database::{DatabaseManager, TradeExecutionLog};
use crate::storage::redis::RedisManager;
use crate::telemetry::metrics;
use chrono::Utc;
use chrono_tz::America::New_York;

const DEFAULT_RELAYER_TIMEOUT_MS: u64 = 500;
const ORDER_LOG_LIMIT: usize = 20;

#[derive(Debug, Clone)]
pub struct ExecutionOrchestratorConfig {
    pub relayer_timeout: Duration,
}

impl Default for ExecutionOrchestratorConfig {
    fn default() -> Self {
        Self {
            relayer_timeout: Duration::from_millis(DEFAULT_RELAYER_TIMEOUT_MS),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionPayloads {
    pub relayer_payload: Value,
    pub relayer_auth: Option<RelayerAuth>,
    pub direct_request: Option<FillOrdersRequest>,
    pub fees_paid: f64,
    pub metadata: Option<Value>,
}

#[async_trait::async_trait]
pub trait ExecutionPayloadBuilder: Send + Sync {
    async fn build_payloads(&self, intent: &TradeIntent) -> Result<ExecutionPayloads>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionRail {
    Relayer,
    Direct,
}

impl ExecutionRail {
    fn as_str(self) -> &'static str {
        match self {
            ExecutionRail::Relayer => "RELAYER",
            ExecutionRail::Direct => "DIRECT",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub success: bool,
    pub rail: ExecutionRail,
    pub latency_ms: Option<u64>,
    pub request_id: Option<String>,
    pub tx_hash: Option<String>,
    pub error: Option<String>,
    pub metadata: Value,
}

pub struct ExecutionOrchestrator {
    config: ExecutionOrchestratorConfig,
    relayer: RelayerClient,
    direct: Option<DirectExecutionClient>,
    database: Option<DatabaseManager>,
    nonce_manager: Option<NonceManager>,
    activity_redis: Option<RedisManager>,
    wallet_key: Option<String>,
    builder: Arc<dyn ExecutionPayloadBuilder>,
}

impl ExecutionOrchestrator {
    pub fn new(
        config: ExecutionOrchestratorConfig,
        relayer: RelayerClient,
        direct: Option<DirectExecutionClient>,
        database: Option<DatabaseManager>,
        nonce_manager: Option<NonceManager>,
        activity_redis: Option<RedisManager>,
        wallet_key: Option<String>,
        builder: Arc<dyn ExecutionPayloadBuilder>,
    ) -> Result<Self> {
        if let Some(manager) = nonce_manager.as_ref() {
            if let Some(direct) = direct.as_ref() {
                let expected = format!("{}", direct.wallet_address()).to_ascii_lowercase();
                if manager.address() != expected {
                    return Err(BankaiError::InvalidArgument(
                        "nonce manager address does not match signer".to_string(),
                    ));
                }
            }
        }

        Ok(Self {
            config,
            relayer,
            direct,
            database,
            nonce_manager,
            activity_redis,
            wallet_key,
            builder,
        })
    }

    pub fn spawn(self, mut receiver: mpsc::Receiver<TradeIntent>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(intent) = receiver.recv().await {
                if let Err(error) = self.handle_intent(intent).await {
                    tracing::error!(?error, "execution orchestrator failed to handle intent");
                }
            }
        })
    }

    async fn handle_intent(&self, intent: TradeIntent) -> Result<()> {
        if let Some(window) = intent.market_window {
            let now = now_ms()?;
            if !is_within_window(now, window) {
                tracing::warn!(
                    market_id = %intent.market_id,
                    start_time_ms = window.start_time_ms,
                    end_time_ms = window.end_time_ms,
                    now_ms = now,
                    "trade intent outside market window; skipping execution"
                );
                return Ok(());
            }
        }

        let payloads = self.builder.build_payloads(&intent).await?;
        let report = self.execute_intent(&intent, &payloads).await?;
        self.persist_report(&intent, &payloads, &report).await?;

        if report.success {
            tracing::info!(
                rail = report.rail.as_str(),
                market_id = %intent.market_id,
                "execution succeeded"
            );
            self.update_tracked_position(&intent, &payloads).await;
        } else {
            tracing::warn!(
                rail = report.rail.as_str(),
                market_id = %intent.market_id,
                error = report.error.as_deref().unwrap_or("unknown"),
                "execution failed"
            );
        }
        self.log_order_event(&intent, &report).await;
        Ok(())
    }

    async fn execute_intent(
        &self,
        intent: &TradeIntent,
        payloads: &ExecutionPayloads,
    ) -> Result<ExecutionReport> {
        let mut metadata = base_metadata(intent, payloads.metadata.clone());
        let relayer_result = self.execute_relayer(payloads).await;

        match relayer_result {
            Ok(response) => {
                metadata.insert("relayer".to_string(), relayer_success_metadata(&response));
                Ok(ExecutionReport {
                    success: true,
                    rail: ExecutionRail::Relayer,
                    latency_ms: Some(response.latency_ms),
                    request_id: response.request_id.clone(),
                    tx_hash: None,
                    error: None,
                    metadata: Value::Object(metadata),
                })
            }
            Err(error) => {
                metadata.insert("relayer".to_string(), relayer_error_metadata(&error));
                if error.should_failover() {
                    if payloads.direct_request.is_some() {
                        metrics::increment_rail_failover();
                        let direct_result = self.execute_direct(payloads).await;

                        match direct_result {
                            Ok(result) => {
                                metadata.insert("direct".to_string(), direct_success_metadata(&result));
                                return Ok(ExecutionReport {
                                    success: true,
                                    rail: ExecutionRail::Direct,
                                    latency_ms: None,
                                    request_id: None,
                                    tx_hash: Some(format!("{:?}", result.tx_hash)),
                                    error: None,
                                    metadata: Value::Object(metadata),
                                });
                            }
                            Err(direct_error) => {
                                metadata.insert(
                                    "direct".to_string(),
                                    direct_error_metadata(&direct_error),
                                );
                                return Ok(ExecutionReport {
                                    success: false,
                                    rail: ExecutionRail::Direct,
                                    latency_ms: None,
                                    request_id: None,
                                    tx_hash: None,
                                    error: Some(direct_error.to_string()),
                                    metadata: Value::Object(metadata),
                                });
                            }
                        }
                    } else {
                        tracing::warn!(
                            market_id = %intent.market_id,
                            "relayer failed but no direct fallback configured"
                        );
                    }
                }
                Ok(ExecutionReport {
                    success: false,
                    rail: ExecutionRail::Relayer,
                    latency_ms: error.latency_ms,
                    request_id: None,
                    tx_hash: None,
                    error: Some(error.message.clone()),
                    metadata: Value::Object(metadata),
                })
            }
        }
    }

    async fn execute_relayer(
        &self,
        payloads: &ExecutionPayloads,
    ) -> std::result::Result<RelayerResponse, RelayerError> {
        let timeout = self.config.relayer_timeout;
        let future = self
            .relayer
            .post_order(&payloads.relayer_payload, payloads.relayer_auth.as_ref());
        match tokio::time::timeout(timeout, future).await {
            Ok(result) => result,
            Err(_) => Err(RelayerError {
                kind: RelayerErrorKind::Timeout,
                message: format!("relayer request exceeded {}ms", duration_to_ms(timeout)),
                status: None,
                body: None,
                latency_ms: Some(duration_to_ms(timeout)),
            }),
        }
    }

    async fn execute_direct(&self, payloads: &ExecutionPayloads) -> Result<DirectExecutionResult> {
        let request = payloads.direct_request.clone().ok_or_else(|| {
            BankaiError::InvalidArgument("direct execution request missing".to_string())
        })?;
        let direct = self.direct.as_ref().ok_or_else(|| {
            BankaiError::InvalidArgument("direct execution client missing".to_string())
        })?;
        let mut request = request;
        if let Some(manager) = self.nonce_manager.as_ref() {
            let nonce = self.ensure_reserved_nonce(manager).await?;
            request.options.nonce = Some(nonce.into());
        }
        direct.send_fill_orders(&request).await
    }

    async fn ensure_reserved_nonce(&self, manager: &NonceManager) -> Result<u64> {
        let direct = self.direct.as_ref().ok_or_else(|| {
            BankaiError::InvalidArgument("direct execution client missing".to_string())
        })?;
        if manager.get_next_nonce().await?.is_none() {
            let chain_nonce = direct.fetch_chain_nonce().await?;
            let next_nonce = u256_to_u64(chain_nonce, "chain nonce")?;
            let _ = manager.initialize_if_missing(next_nonce).await?;
        }
        let now_ms = now_ms()?;
        manager.reserve_nonce(now_ms).await
    }

    async fn persist_report(
        &self,
        intent: &TradeIntent,
        payloads: &ExecutionPayloads,
        report: &ExecutionReport,
    ) -> Result<()> {
        if let Some(database) = self.database.as_ref() {
            let log = TradeExecutionLog {
                market_id: intent.market_id.clone(),
                rail: report.rail.as_str().to_string(),
                mode: trade_mode_label(intent.mode).to_string(),
                expected_ev: intent.edge,
                actual_pnl: None,
                fees_paid: payloads.fees_paid,
                latency_ms: report.latency_ms,
                metadata: Some(report.metadata.clone()),
            };
            database.log_trade_execution(&log).await?;
        }
        Ok(())
    }
}

impl ExecutionOrchestrator {
    async fn log_order_event(&self, intent: &TradeIntent, report: &ExecutionReport) {
        let Some(redis) = self.activity_redis.as_ref() else {
            return;
        };
        let prefix = log_prefix();
        let status = if report.success { "OK" } else { "FAIL" };
        let message = format!(
            "{prefix} Order {status} rail={} market={} mode={} edge_bps={:.1}",
            report.rail.as_str(),
            intent.market_id,
            trade_mode_label(intent.mode),
            intent.edge_bps
        );
        let _ = redis.push_order_log(&message, ORDER_LOG_LIMIT).await;
    }

    async fn update_tracked_position(&self, intent: &TradeIntent, payloads: &ExecutionPayloads) {
        let Some(redis) = self.activity_redis.as_ref() else {
            return;
        };
        let Some(wallet_key) = self.wallet_key.as_ref() else {
            return;
        };
        let Some(context) = payloads.metadata.as_ref() else {
            return;
        };
        let size = context
            .get("size")
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0);
        let price = context
            .get("price")
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0);
        if size <= 0.0 || price <= 0.0 {
            return;
        }

        let asset_id = intent.asset_id.as_str();
        let current = match redis.get_tracked_position(wallet_key, asset_id).await {
            Ok(value) => value,
            Err(_) => return,
        };
        let mut new_balance = current;
        match intent.side {
            TradeSide::Buy => {
                new_balance += size;
                if let Ok(entry) = redis.get_entry_price(wallet_key, asset_id).await {
                    let entry_price = entry.unwrap_or(0.0);
                    let weighted = if entry_price > 0.0 && current > 0.0 {
                        ((entry_price * current) + (price * size)) / new_balance
                    } else {
                        price
                    };
                    let _ = redis.set_entry_price(wallet_key, asset_id, weighted).await;
                    let _ = redis.set_peak_price(wallet_key, asset_id, price).await;
                }
            }
            TradeSide::Sell => {
                new_balance = (current - size).max(0.0);
                if new_balance <= 0.0 {
                    let _ = redis.set_entry_price(wallet_key, asset_id, 0.0).await;
                    let _ = redis.set_peak_price(wallet_key, asset_id, 0.0).await;
                }
            }
        }
        let _ = redis
            .set_tracked_position(wallet_key, asset_id, new_balance)
            .await;
    }
}

fn log_prefix() -> String {
    let now = Utc::now();
    let et = now.with_timezone(&New_York);
    format!("[{}]", et.format("%H:%M:%S"))
}

fn base_metadata(intent: &TradeIntent, extra: Option<Value>) -> Map<String, Value> {
    let mut metadata = Map::new();
    metadata.insert(
        "intent".to_string(),
        json!({
            "market_id": intent.market_id.as_str(),
            "asset_id": intent.asset_id.as_str(),
            "side": trade_side_label(intent.side),
            "mode": trade_mode_label(intent.mode),
            "implied_prob": intent.implied_prob,
            "true_prob": intent.true_prob,
            "edge": intent.edge,
            "edge_bps": intent.edge_bps,
            "spread_offset_bps": intent.spread_offset_bps,
            "timestamp_ms": intent.timestamp_ms,
            "market_window": intent.market_window.map(|window| {
                json!({
                    "start_time_ms": window.start_time_ms,
                    "end_time_ms": window.end_time_ms,
                })
            }),
            "requested_size": intent.requested_size,
        }),
    );
    if let Some(extra) = extra {
        metadata.insert("context".to_string(), extra);
    }
    metadata
}

fn relayer_success_metadata(response: &RelayerResponse) -> Value {
    json!({
        "status": "ok",
        "http_status": response.status.as_u16(),
        "request_id": response.request_id.clone(),
        "latency_ms": response.latency_ms,
        "body": response.body.clone(),
    })
}

fn relayer_error_metadata(error: &RelayerError) -> Value {
    json!({
        "status": "error",
        "kind": relayer_error_kind_label(error.kind),
        "message": error.message.clone(),
        "http_status": error.status.map(|status| status.as_u16()),
        "latency_ms": error.latency_ms,
        "body": error.body.clone(),
    })
}

fn direct_success_metadata(result: &DirectExecutionResult) -> Value {
    json!({
        "status": "ok",
        "tx_hash": format!("{:?}", result.tx_hash),
        "nonce": result.nonce.to_string(),
        "gas_limit": result.gas_limit.to_string(),
        "max_fee_per_gas": result.max_fee_per_gas.to_string(),
        "max_priority_fee_per_gas": result.max_priority_fee_per_gas.to_string(),
    })
}

fn direct_error_metadata(error: &BankaiError) -> Value {
    json!({
        "status": "error",
        "message": error.to_string(),
    })
}

fn relayer_error_kind_label(kind: RelayerErrorKind) -> &'static str {
    match kind {
        RelayerErrorKind::Timeout => "timeout",
        RelayerErrorKind::Transport => "transport",
        RelayerErrorKind::Congestion => "congestion",
        RelayerErrorKind::Server => "server",
        RelayerErrorKind::Client => "client",
        RelayerErrorKind::InvalidRequest => "invalid_request",
        RelayerErrorKind::InvalidResponse => "invalid_response",
    }
}

fn trade_mode_label(mode: TradeMode) -> &'static str {
    match mode {
        TradeMode::Ladder => "LADDER",
        TradeMode::Snipe => "SNIPE",
    }
}

fn trade_side_label(side: TradeSide) -> &'static str {
    match side {
        TradeSide::Buy => "BUY",
        TradeSide::Sell => "SELL",
    }
}

fn duration_to_ms(duration: Duration) -> u64 {
    let ms = duration.as_millis();
    if ms > u128::from(u64::MAX) {
        u64::MAX
    } else {
        ms as u64
    }
}

fn is_within_window(timestamp_ms: u64, window: crate::engine::types::MarketWindow) -> bool {
    timestamp_ms >= window.start_time_ms && timestamp_ms <= window.end_time_ms
}

fn now_ms() -> Result<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now.as_millis() as u64)
}

fn u256_to_u64(value: ethers_core::types::U256, label: &str) -> Result<u64> {
    if value > ethers_core::types::U256::from(u64::MAX) {
        return Err(BankaiError::InvalidArgument(format!(
            "{label} exceeds u64 range"
        )));
    }
    Ok(value.as_u64())
}
