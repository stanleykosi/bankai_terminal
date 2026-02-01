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

use crate::accounting::no_money::{record_no_money_intent, PaperSimConfig};
use crate::engine::types::{TradeIntent, TradeMode, TradeSide};
use crate::error::{BankaiError, Result};
use crate::execution::cancel::CancelClient;
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
use sha2::{Digest, Sha256};

const DEFAULT_RELAYER_TIMEOUT_MS: u64 = 500;
const ORDER_LOG_LIMIT: usize = 20;

#[derive(Debug, Clone)]
pub struct ExecutionOrchestratorConfig {
    pub relayer_timeout: Duration,
    pub prefer_ws_reconcile: bool,
    pub max_retries: u32,
    pub backoff_ms: u64,
    pub backoff_max_ms: u64,
    pub idempotency_ttl_secs: u64,
    pub cancel_before_replace: bool,
    pub no_money_mode: bool,
}

impl Default for ExecutionOrchestratorConfig {
    fn default() -> Self {
        Self {
            relayer_timeout: Duration::from_millis(DEFAULT_RELAYER_TIMEOUT_MS),
            prefer_ws_reconcile: true,
            max_retries: 2,
            backoff_ms: 50,
            backoff_max_ms: 500,
            idempotency_ttl_secs: 30,
            cancel_before_replace: true,
            no_money_mode: false,
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
    cancel_client: Option<CancelClient>,
    database: Option<DatabaseManager>,
    nonce_manager: Option<NonceManager>,
    activity_redis: Option<RedisManager>,
    wallet_key: Option<String>,
    builder: Arc<dyn ExecutionPayloadBuilder>,
    paper_sim: Option<PaperSimConfig>,
}

impl ExecutionOrchestrator {
    pub fn new(
        config: ExecutionOrchestratorConfig,
        relayer: RelayerClient,
        direct: Option<DirectExecutionClient>,
        cancel_client: Option<CancelClient>,
        database: Option<DatabaseManager>,
        nonce_manager: Option<NonceManager>,
        activity_redis: Option<RedisManager>,
        wallet_key: Option<String>,
        builder: Arc<dyn ExecutionPayloadBuilder>,
        paper_sim: Option<PaperSimConfig>,
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
            cancel_client,
            database,
            nonce_manager,
            activity_redis,
            wallet_key,
            builder,
            paper_sim,
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

        if self.config.no_money_mode {
            if let Some(redis) = self.activity_redis.as_ref() {
                if let Some(sim) = self.paper_sim.as_ref() {
                    let _ = record_no_money_intent(redis, &intent, sim).await;
                }
                self.log_activity_event(format!(
                    "[PAPER] intent captured market={} mode={} edge_bps={:.1}",
                    intent.market_id,
                    trade_mode_label(intent.mode),
                    intent.edge_bps
                ))
                .await;
            }
            return Ok(());
        }

        if intent.mode == TradeMode::Ladder && self.config.cancel_before_replace {
            if let Some(client) = self.cancel_client.as_ref() {
                match client
                    .cancel_market_orders(&intent.market_id, &intent.asset_id)
                    .await
                {
                    Ok(response) => {
                        self.log_activity_event(format!(
                            "[CANCEL] ok market={} asset={} cancelled={}",
                            intent.market_id,
                            intent.asset_id,
                            response.canceled.len()
                        ))
                        .await;
                    }
                    Err(error) => {
                        self.log_activity_event(format!(
                            "[CANCEL] fail market={} asset={} error={:?}",
                            intent.market_id, intent.asset_id, error
                        ))
                        .await;
                    }
                }
            }
        }

        let payloads = self.builder.build_payloads(&intent).await?;
        if let Some(redis) = self.activity_redis.as_ref() {
            let fingerprint = fingerprint_payload(&payloads.relayer_payload)?;
            let key = format!("orders:idempotency:{fingerprint}");
            if !redis
                .set_if_absent(&key, "1", self.config.idempotency_ttl_secs)
                .await?
            {
                self.log_activity_event(format!(
                    "[RELAYER] skip duplicate market={} mode={}",
                    intent.market_id,
                    trade_mode_label(intent.mode)
                ))
                .await;
                return Ok(());
            }
        }
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
        self.log_activity_event(format!(
            "[RELAYER] send market={} mode={} edge_bps={:.1}",
            intent.market_id,
            trade_mode_label(intent.mode),
            intent.edge_bps
        ))
        .await;
        let relayer_result = self.execute_relayer_with_retry(payloads).await;

        match relayer_result {
            Ok(response) => {
                if let (Some(redis), Some(wallet_key)) =
                    (self.activity_redis.as_ref(), self.wallet_key.as_ref())
                {
                    if let Some(order_id) = extract_order_id(&response.body) {
                        let payload = serde_json::json!({
                            "id": order_id,
                            "status": "SENT",
                            "market_id": intent.market_id,
                            "asset_id": intent.asset_id,
                            "side": trade_side_label(intent.side),
                        });
                        if let Ok(json) = serde_json::to_string(&payload) {
                            let now = now_ms().unwrap_or(0);
                            let _ = redis
                                .set_order_state(wallet_key, &order_id, &json, now)
                                .await;
                        }
                    }
                }
                self.log_activity_event(format!(
                    "[RELAYER] ok market={} request_id={}",
                    intent.market_id,
                    response.request_id.as_deref().unwrap_or("n/a")
                ))
                .await;
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
                self.log_activity_event(format!(
                    "[RELAYER] fail market={} error={:?}",
                    intent.market_id, error
                ))
                .await;
                metadata.insert("relayer".to_string(), relayer_error_metadata(&error));
                if error.should_failover() {
                    if payloads.direct_request.is_some() {
                        metrics::increment_rail_failover();
                        let direct_result = self.execute_direct(payloads).await;

                        match direct_result {
                            Ok(result) => {
                                metadata
                                    .insert("direct".to_string(), direct_success_metadata(&result));
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

    async fn execute_relayer_with_retry(
        &self,
        payloads: &ExecutionPayloads,
    ) -> std::result::Result<RelayerResponse, RelayerError> {
        let mut attempt = 0u32;
        loop {
            let result = self.execute_relayer(payloads).await;
            match result {
                Ok(response) => return Ok(response),
                Err(error) => {
                    if !should_retry(&error) || attempt >= self.config.max_retries {
                        return Err(error);
                    }
                    let delay = backoff_delay_ms(
                        self.config.backoff_ms,
                        self.config.backoff_max_ms,
                        attempt,
                    );
                    self.log_activity_event(format!(
                        "[RELAYER] retry attempt={} delay_ms={} error={:?}",
                        attempt + 1,
                        delay,
                        error
                    ))
                    .await;
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                    attempt += 1;
                }
            }
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
    async fn log_activity_event(&self, message: String) {
        let Some(redis) = self.activity_redis.as_ref() else {
            return;
        };
        let prefix = log_prefix();
        let entry = format!("{prefix} {message}");
        let _ = redis.push_activity_log(&entry, ORDER_LOG_LIMIT).await;
    }

    async fn log_order_event(&self, intent: &TradeIntent, report: &ExecutionReport) {
        let Some(redis) = self.activity_redis.as_ref() else {
            return;
        };
        let prefix = log_prefix();
        let status = if report.success { "OK" } else { "FAIL" };
        let message = format!(
            "{prefix} [ORDER] {status} rail={} market={} mode={} edge_bps={:.1}",
            report.rail.as_str(),
            intent.market_id,
            trade_mode_label(intent.mode),
            intent.edge_bps
        );
        let _ = redis.push_order_log(&message, ORDER_LOG_LIMIT).await;
    }

    async fn update_tracked_position(&self, intent: &TradeIntent, payloads: &ExecutionPayloads) {
        if self.config.prefer_ws_reconcile {
            return;
        }
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

fn should_retry(error: &RelayerError) -> bool {
    matches!(
        error.kind,
        RelayerErrorKind::Timeout
            | RelayerErrorKind::Transport
            | RelayerErrorKind::Congestion
            | RelayerErrorKind::Server
    )
}

fn backoff_delay_ms(base_ms: u64, max_ms: u64, attempt: u32) -> u64 {
    let exp = 2u64.saturating_pow(attempt.min(16));
    let base = base_ms.saturating_mul(exp);
    let jitter = (now_ms().unwrap_or(0) % (base_ms.max(1))) as u64;
    base.saturating_add(jitter).min(max_ms.max(base_ms))
}

fn fingerprint_payload(payload: &Value) -> Result<String> {
    let body = serde_json::to_string(payload)?;
    let mut hasher = Sha256::new();
    hasher.update(body.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

fn extract_order_id(body: &Value) -> Option<String> {
    if let Some(id) = body.get("orderId").and_then(|v| v.as_str()) {
        return Some(id.to_string());
    }
    if let Some(id) = body.get("orderID").and_then(|v| v.as_str()) {
        return Some(id.to_string());
    }
    if let Some(id) = body.get("id").and_then(|v| v.as_str()) {
        return Some(id.to_string());
    }
    None
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
