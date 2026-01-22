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

use crate::engine::types::{TradeIntent, TradeMode};
use crate::error::{BankaiError, Result};
use crate::execution::direct::{DirectExecutionClient, DirectExecutionResult, FillOrdersRequest};
use crate::execution::nonce::NonceManager;
use crate::execution::relayer::{
    RelayerAuth, RelayerClient, RelayerError, RelayerErrorKind, RelayerResponse,
};
use crate::storage::database::{DatabaseManager, TradeExecutionLog};
use crate::telemetry::metrics;

const DEFAULT_RELAYER_TIMEOUT_MS: u64 = 500;

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
    pub direct_request: FillOrdersRequest,
    pub fees_paid: f64,
    pub metadata: Option<Value>,
}

pub trait ExecutionPayloadBuilder: Send + Sync {
    fn build_payloads(&self, intent: &TradeIntent) -> Result<ExecutionPayloads>;
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
    direct: DirectExecutionClient,
    database: DatabaseManager,
    nonce_manager: Option<NonceManager>,
    builder: Arc<dyn ExecutionPayloadBuilder>,
}

impl ExecutionOrchestrator {
    pub fn new(
        config: ExecutionOrchestratorConfig,
        relayer: RelayerClient,
        direct: DirectExecutionClient,
        database: DatabaseManager,
        nonce_manager: Option<NonceManager>,
        builder: Arc<dyn ExecutionPayloadBuilder>,
    ) -> Result<Self> {
        if let Some(manager) = nonce_manager.as_ref() {
            let expected = format!("{}", direct.wallet_address()).to_ascii_lowercase();
            if manager.address() != expected {
                return Err(BankaiError::InvalidArgument(
                    "nonce manager address does not match signer".to_string(),
                ));
            }
        }

        Ok(Self {
            config,
            relayer,
            direct,
            database,
            nonce_manager,
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
        let payloads = self.builder.build_payloads(&intent)?;
        let report = self.execute_intent(&intent, &payloads).await?;
        self.persist_report(&intent, &payloads, &report).await?;

        if report.success {
            tracing::info!(
                rail = report.rail.as_str(),
                market_id = %intent.market_id,
                "execution succeeded"
            );
        } else {
            tracing::warn!(
                rail = report.rail.as_str(),
                market_id = %intent.market_id,
                error = report.error.as_deref().unwrap_or("unknown"),
                "execution failed"
            );
        }
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
                metrics::increment_rail_failover();
                let direct_result = self.execute_direct(payloads).await;

                match direct_result {
                    Ok(result) => {
                        metadata.insert("direct".to_string(), direct_success_metadata(&result));
                        Ok(ExecutionReport {
                            success: true,
                            rail: ExecutionRail::Direct,
                            latency_ms: None,
                            request_id: None,
                            tx_hash: Some(format!("{:?}", result.tx_hash)),
                            error: None,
                            metadata: Value::Object(metadata),
                        })
                    }
                    Err(direct_error) => {
                        metadata.insert("direct".to_string(), direct_error_metadata(&direct_error));
                        Ok(ExecutionReport {
                            success: false,
                            rail: ExecutionRail::Direct,
                            latency_ms: None,
                            request_id: None,
                            tx_hash: None,
                            error: Some(direct_error.to_string()),
                            metadata: Value::Object(metadata),
                        })
                    }
                }
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
        let mut request = payloads.direct_request.clone();
        if let Some(manager) = self.nonce_manager.as_ref() {
            let nonce = self.ensure_reserved_nonce(manager).await?;
            request.options.nonce = Some(nonce.into());
        }
        self.direct.send_fill_orders(&request).await
    }

    async fn ensure_reserved_nonce(&self, manager: &NonceManager) -> Result<u64> {
        if manager.get_next_nonce().await?.is_none() {
            let chain_nonce = self.direct.fetch_chain_nonce().await?;
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
        self.database.log_trade_execution(&log).await
    }
}

fn base_metadata(intent: &TradeIntent, extra: Option<Value>) -> Map<String, Value> {
    let mut metadata = Map::new();
    metadata.insert(
        "intent".to_string(),
        json!({
            "market_id": intent.market_id.as_str(),
            "asset_id": intent.asset_id.as_str(),
            "mode": trade_mode_label(intent.mode),
            "implied_prob": intent.implied_prob,
            "true_prob": intent.true_prob,
            "edge": intent.edge,
            "edge_bps": intent.edge_bps,
            "spread_offset_bps": intent.spread_offset_bps,
            "timestamp_ms": intent.timestamp_ms,
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

fn duration_to_ms(duration: Duration) -> u64 {
    let ms = duration.as_millis();
    if ms > u128::from(u64::MAX) {
        u64::MAX
    } else {
        ms as u64
    }
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
