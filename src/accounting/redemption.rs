/**
 * @purpose
 * Listen for Conditional Tokens resolution events (primary) with UMA adapter fallback
 * and optional subgraph backfill, then redeem positions on-chain.
 *
 * @dependencies
 * - ethers-core: ABI encoding/decoding, log parsing, keccak
 * - ethers-providers: JSON-RPC transport
 * - ethers-signers: local signing
 * - tokio: async runtime utilities
 * - reqwest: GraphQL fallback client
 * - redis: bankroll updates via RedisManager
 *
 * @notes
 * - ConditionResolution logs from the CTF contract are the primary signal.
 * - UMA adapter QuestionResolved logs provide redundancy.
 * - Subgraph backfill is optional and runs on a slower interval.
 */
use ethers_core::abi::{
    Abi, Event, EventParam, Function, LogParam, Param, ParamType, RawLog, StateMutability, Token,
};
use ethers_core::types::{
    transaction::eip2718::TypedTransaction, Address, BlockNumber, Bytes, Eip1559TransactionRequest,
    Filter, TransactionRequest, H256, U256, U64,
};
use ethers_core::utils::keccak256;
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::{LocalWallet, Signer};
use reqwest::Client;
use secrecy::ExposeSecret;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::error::{BankaiError, Result};
use crate::security::Secrets;
use crate::storage::redis::RedisManager;

const DEFAULT_ABI_PATH: &str = "abi/ConditionalTokens.json";
const DEFAULT_PARENT_COLLECTION_ID: &str =
    "0x0000000000000000000000000000000000000000000000000000000000000000";
const DEFAULT_POLL_INTERVAL_SECS: u64 = 6;
const DEFAULT_MIN_CONFIRMATIONS: u64 = 3;
const DEFAULT_RECEIPT_CONFIRMATIONS: u64 = 1;
const DEFAULT_MAX_BLOCK_RANGE: u64 = 1_000;
const DEFAULT_TIMEOUT_MS: u64 = 4_000;
const DEFAULT_MAX_FEE_GWEI: u64 = 50;
const DEFAULT_PRIORITY_FEE_GWEI: u64 = 2;
const DEFAULT_ADAPTER_OUTCOME_SLOTS: u32 = 2;
const DEFAULT_SUBGRAPH_POLL_INTERVAL_SECS: u64 = 60;
const DEFAULT_SUBGRAPH_MAX_ITEMS: usize = 200;
const WEI_PER_GWEI: u64 = 1_000_000_000;
const BANKROLL_REDIS_KEY: &str = "sys:bankroll:usdc";
const SUBGRAPH_QUERY: &str =
    "query Resolutions($last: BigInt!, $limit: Int!) { marketResolutions(where: { status: \"resolved\", lastUpdateTimestamp_gt: $last }, orderBy: lastUpdateTimestamp, orderDirection: asc, first: $limit) { id lastUpdateTimestamp } }";

/// Runtime configuration for UMA adapter polling.
#[derive(Debug, Clone)]
pub struct AdapterConfig {
    pub address: String,
    pub start_block: Option<u64>,
    pub default_outcome_slot_count: u32,
}

impl AdapterConfig {
    /// Build adapter config with defaults (binary outcomes assumed).
    pub fn new(address: String) -> Self {
        Self {
            address,
            start_block: None,
            default_outcome_slot_count: DEFAULT_ADAPTER_OUTCOME_SLOTS,
        }
    }
}

/// Runtime configuration for subgraph backfill.
#[derive(Debug, Clone)]
pub struct SubgraphConfig {
    pub url: String,
    pub poll_interval: Duration,
    pub max_items: usize,
    pub default_outcome_slot_count: u32,
}

impl SubgraphConfig {
    /// Build a subgraph config with standard defaults.
    pub fn new(url: String) -> Self {
        Self {
            url,
            poll_interval: Duration::from_secs(DEFAULT_SUBGRAPH_POLL_INTERVAL_SECS),
            max_items: DEFAULT_SUBGRAPH_MAX_ITEMS,
            default_outcome_slot_count: DEFAULT_ADAPTER_OUTCOME_SLOTS,
        }
    }
}

/// Runtime configuration for the redemption listener and client.
#[derive(Debug, Clone)]
pub struct RedemptionConfig {
    pub rpc_url: String,
    pub ctf_address: String,
    pub collateral_token: String,
    pub chain_id: u64,
    pub parent_collection_id: String,
    pub collateral_decimals: u32,
    pub abi_path: String,
    pub poll_interval: Duration,
    pub min_confirmations: u64,
    pub receipt_confirmations: u64,
    pub max_block_range: u64,
    pub request_timeout: Duration,
    pub gas_limit: Option<U256>,
    pub max_fee_per_gas_gwei: Option<u64>,
    pub max_priority_fee_gwei: Option<u64>,
    pub start_block: Option<u64>,
    pub adapter_configs: Vec<AdapterConfig>,
    pub subgraph: Option<SubgraphConfig>,
}

impl RedemptionConfig {
    /// Build a config with standard defaults for the redemption workflow.
    pub fn new(
        rpc_url: String,
        ctf_address: String,
        collateral_token: String,
        chain_id: u64,
    ) -> Self {
        Self {
            rpc_url,
            ctf_address,
            collateral_token,
            chain_id,
            parent_collection_id: DEFAULT_PARENT_COLLECTION_ID.to_string(),
            collateral_decimals: 6,
            abi_path: DEFAULT_ABI_PATH.to_string(),
            poll_interval: Duration::from_secs(DEFAULT_POLL_INTERVAL_SECS),
            min_confirmations: DEFAULT_MIN_CONFIRMATIONS,
            receipt_confirmations: DEFAULT_RECEIPT_CONFIRMATIONS,
            max_block_range: DEFAULT_MAX_BLOCK_RANGE,
            request_timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),
            gas_limit: None,
            max_fee_per_gas_gwei: None,
            max_priority_fee_gwei: None,
            start_block: None,
            adapter_configs: Vec::new(),
            subgraph: None,
        }
    }
}

/// A redemption target associated with a resolved condition.
#[derive(Debug, Clone)]
pub struct RedemptionTarget {
    pub parent_collection_id: Option<H256>,
    pub index_sets: Vec<U256>,
}

impl RedemptionTarget {
    /// Create a redemption target with the default parent collection id.
    pub fn new(index_sets: Vec<U256>) -> Result<Self> {
        if index_sets.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "index sets must be non-empty".to_string(),
            ));
        }
        Ok(Self {
            parent_collection_id: None,
            index_sets,
        })
    }

    /// Create a redemption target with an explicit parent collection id.
    pub fn with_parent_collection_id(
        parent_collection_id: H256,
        index_sets: Vec<U256>,
    ) -> Result<Self> {
        if index_sets.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "index sets must be non-empty".to_string(),
            ));
        }
        Ok(Self {
            parent_collection_id: Some(parent_collection_id),
            index_sets,
        })
    }
}

/// Request payload for calling redeemPositions.
#[derive(Debug, Clone)]
pub struct RedemptionRequest {
    pub condition_id: H256,
    pub parent_collection_id: H256,
    pub index_sets: Vec<U256>,
}

impl RedemptionRequest {
    fn from_target(
        condition_id: H256,
        target: RedemptionTarget,
        default_parent: H256,
    ) -> Result<Self> {
        if target.index_sets.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "index sets must be non-empty".to_string(),
            ));
        }
        Ok(Self {
            condition_id,
            parent_collection_id: target.parent_collection_id.unwrap_or(default_parent),
            index_sets: target.index_sets,
        })
    }

    fn validate(&self) -> Result<()> {
        if self.index_sets.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "index sets must be non-empty".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ResolutionSource {
    Ctf,
    Adapter { address: Address, question_id: H256 },
    Subgraph { adapter: Address, question_id: H256 },
}

impl ResolutionSource {
    fn label(&self) -> &'static str {
        match self {
            ResolutionSource::Ctf => "ctf",
            ResolutionSource::Adapter { .. } => "adapter",
            ResolutionSource::Subgraph { .. } => "subgraph",
        }
    }
}

/// Unified representation of a resolved condition.
#[derive(Debug, Clone)]
pub struct ResolvedCondition {
    pub condition_id: H256,
    pub source: ResolutionSource,
    pub tx_hash: Option<H256>,
    pub block_number: Option<U64>,
}

/// ConditionResolution event payload.
#[derive(Debug, Clone)]
pub struct ConditionResolutionEvent {
    pub condition_id: H256,
    pub tx_hash: Option<H256>,
    pub block_number: Option<U64>,
}

/// Resolves redemption targets for a condition id.
pub trait PositionResolver: Send + Sync {
    fn resolve_targets(
        &self,
        condition_id: H256,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RedemptionTarget>>> + Send>>;
}

/// A resolver that always returns no positions.
pub struct NoopPositionResolver;

impl PositionResolver for NoopPositionResolver {
    fn resolve_targets(
        &self,
        _condition_id: H256,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<RedemptionTarget>>> + Send>> {
        Box::pin(async { Ok(Vec::new()) })
    }
}

/// Redemption transaction metadata after confirmation.
#[derive(Debug, Clone)]
pub struct RedemptionOutcome {
    pub tx_hash: H256,
    pub block_number: Option<U64>,
    pub gas_used: Option<U256>,
}

/// Conditional Tokens client for redemption and event polling.
pub struct RedemptionClient {
    config: RedemptionConfig,
    provider: Provider<Http>,
    http_client: Client,
    wallet: LocalWallet,
    ctf_abi: Abi,
    ctf_address: Address,
    collateral_token: Address,
    parent_collection_id: H256,
    resolution_event: Event,
    adapter_resolution_event: Event,
    adapter_resolution_legacy_event: Event,
    balance_of: Function,
}

impl RedemptionClient {
    /// Build a redemption client from config and runtime secrets.
    pub fn new(config: RedemptionConfig, secrets: &Secrets) -> Result<Self> {
        if config.rpc_url.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "polygon rpc url is required".to_string(),
            ));
        }
        if config.ctf_address.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "ctf contract address is required".to_string(),
            ));
        }
        if config.collateral_token.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "collateral token address is required".to_string(),
            ));
        }

        let ctf_address = parse_address(&config.ctf_address, "ctf contract address")?;
        let collateral_token = parse_address(&config.collateral_token, "collateral token address")?;
        let parent_collection_id = parse_parent_collection_id(&config.parent_collection_id)?;
        let wallet = build_wallet(secrets, config.chain_id)?;
        let http_client = build_http_client(&config)?;
        let provider = build_provider(&config, http_client.clone())?;
        let abi = load_ctf_abi(Path::new(&config.abi_path))?;
        let resolution_event = abi.event("ConditionResolution").map_err(|err| {
            BankaiError::InvalidArgument(format!(
                "ConditionResolution event missing from ABI: {err}"
            ))
        })?;
        let resolution_event = resolution_event.clone();
        let adapter_resolution_event = build_adapter_question_resolved_event();
        let adapter_resolution_legacy_event = build_adapter_question_resolved_legacy_event();
        let balance_of = build_balance_of_function();

        Ok(Self {
            config,
            provider,
            http_client,
            wallet,
            ctf_abi: abi,
            ctf_address,
            collateral_token,
            parent_collection_id,
            resolution_event,
            adapter_resolution_event,
            adapter_resolution_legacy_event,
            balance_of,
        })
    }

    /// Returns the active redemption configuration.
    pub fn config(&self) -> &RedemptionConfig {
        &self.config
    }

    /// Returns the signer address used for redemption.
    pub fn wallet_address(&self) -> Address {
        self.wallet.address()
    }

    /// Returns the default parent collection id.
    pub fn parent_collection_id(&self) -> H256 {
        self.parent_collection_id
    }

    /// Fetch the latest block height.
    pub async fn latest_block(&self) -> Result<u64> {
        let block = self
            .provider
            .get_block_number()
            .await
            .map_err(|err| BankaiError::Rpc(format!("block number fetch failed: {err}")))?;
        Ok(block.as_u64())
    }

    /// Fetch ConditionResolution events between block ranges.
    pub async fn fetch_resolution_events(
        &self,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<ConditionResolutionEvent>> {
        let logs = self
            .fetch_event_logs(
                self.ctf_address,
                &self.resolution_event,
                from_block,
                to_block,
            )
            .await?;
        let mut events = Vec::new();
        for log in logs {
            let parsed = parse_log(&self.resolution_event, &log)?;
            let condition_id = extract_condition_id(&parsed.params)?;
            events.push(ConditionResolutionEvent {
                condition_id,
                tx_hash: log.transaction_hash,
                block_number: log.block_number,
            });
        }
        Ok(events)
    }

    /// Fetch QuestionResolved events from UMA adapters between block ranges.
    pub async fn fetch_adapter_resolution_events(
        &self,
        adapter: Address,
        from_block: u64,
        to_block: u64,
        default_outcome_slot_count: u32,
    ) -> Result<Vec<ResolvedCondition>> {
        let mut events = Vec::new();
        let with_payouts = self
            .fetch_event_logs(
                adapter,
                &self.adapter_resolution_event,
                from_block,
                to_block,
            )
            .await?;
        for log in with_payouts {
            let parsed = parse_log(&self.adapter_resolution_event, &log)?;
            let question_id = extract_question_id(&parsed.params)?;
            let payout_len = extract_payouts_len(&parsed.params).unwrap_or(0);
            let outcome_slot_count = if payout_len == 0 {
                default_outcome_slot_count
            } else {
                payout_len as u32
            };
            let condition_id = compute_condition_id(adapter, question_id, outcome_slot_count);
            events.push(ResolvedCondition {
                condition_id,
                source: ResolutionSource::Adapter {
                    address: adapter,
                    question_id,
                },
                tx_hash: log.transaction_hash,
                block_number: log.block_number,
            });
        }

        let legacy = self
            .fetch_event_logs(
                adapter,
                &self.adapter_resolution_legacy_event,
                from_block,
                to_block,
            )
            .await?;
        for log in legacy {
            let parsed = parse_log(&self.adapter_resolution_legacy_event, &log)?;
            let question_id = extract_question_id(&parsed.params)?;
            let condition_id =
                compute_condition_id(adapter, question_id, default_outcome_slot_count.max(1));
            events.push(ResolvedCondition {
                condition_id,
                source: ResolutionSource::Adapter {
                    address: adapter,
                    question_id,
                },
                tx_hash: log.transaction_hash,
                block_number: log.block_number,
            });
        }

        Ok(events)
    }

    /// Redeem positions for the resolved condition.
    pub async fn redeem_positions(&self, request: &RedemptionRequest) -> Result<RedemptionOutcome> {
        request.validate()?;
        let calldata = self.encode_redeem_positions(request)?;
        let tx = Eip1559TransactionRequest {
            from: Some(self.wallet.address()),
            to: Some(self.ctf_address.into()),
            data: Some(calldata),
            chain_id: Some(self.config.chain_id.into()),
            value: Some(U256::zero()),
            ..Default::default()
        };

        let mut typed_tx = TypedTransaction::Eip1559(tx);
        let nonce = resolve_nonce(&self.provider, &self.wallet).await?;
        typed_tx.set_nonce(nonce);

        let gas_limit = resolve_gas_limit(&self.provider, &typed_tx, &self.config).await?;
        typed_tx.set_gas(gas_limit);

        let (max_fee_per_gas, max_priority_fee_per_gas) =
            resolve_fee_data(&self.provider, &self.config).await?;
        if let Some(tx) = typed_tx.as_eip1559_mut() {
            tx.max_fee_per_gas = Some(max_fee_per_gas);
            tx.max_priority_fee_per_gas = Some(max_priority_fee_per_gas);
        }

        let signature = self
            .wallet
            .sign_transaction(&typed_tx)
            .await
            .map_err(|err| BankaiError::Crypto(format!("transaction signing failed: {err}")))?;
        let rlp = typed_tx.rlp_signed(&signature);

        let pending = self
            .provider
            .send_raw_transaction(rlp)
            .await
            .map_err(|err| BankaiError::Rpc(format!("redeemPositions tx failed: {err}")))?;
        let tx_hash = pending.tx_hash();

        let confirmations = self.config.receipt_confirmations.max(1) as usize;
        let receipt = pending
            .confirmations(confirmations)
            .await
            .map_err(|err| BankaiError::Rpc(format!("redeemPositions receipt error: {err}")))?;
        let receipt = receipt
            .ok_or_else(|| BankaiError::Rpc("redeemPositions transaction dropped".to_string()))?;

        if receipt.status != Some(U64::from(1)) {
            return Err(BankaiError::Rpc("redeemPositions reverted".to_string()));
        }

        Ok(RedemptionOutcome {
            tx_hash,
            block_number: receipt.block_number,
            gas_used: receipt.gas_used,
        })
    }

    /// Fetch the collateral balance for the signer wallet.
    pub async fn fetch_collateral_balance(&self) -> Result<U256> {
        let data = self
            .balance_of
            .encode_input(&[Token::Address(self.wallet.address())])
            .map_err(|err| {
                BankaiError::InvalidArgument(format!("balanceOf encode failed: {err}"))
            })?;
        let tx = TransactionRequest {
            to: Some(self.collateral_token.into()),
            data: Some(Bytes::from(data)),
            ..Default::default()
        };
        let call: TypedTransaction = tx.into();
        let raw = self
            .provider
            .call(&call, None)
            .await
            .map_err(|err| BankaiError::Rpc(format!("balanceOf call failed: {err}")))?;
        let decoded = self.balance_of.decode_output(raw.as_ref()).map_err(|err| {
            BankaiError::InvalidArgument(format!("balanceOf decode failed: {err}"))
        })?;
        decode_balance(&decoded)
    }

    /// Fetch resolved questions from the subgraph since a timestamp.
    async fn fetch_subgraph_resolutions(
        &self,
        url: &str,
        since_timestamp: u64,
        limit: usize,
    ) -> Result<Vec<SubgraphResolution>> {
        let payload = json!({
            "query": SUBGRAPH_QUERY,
            "variables": {
                "last": since_timestamp.to_string(),
                "limit": limit as i64
            }
        });
        let response = self.http_client.post(url).json(&payload).send().await?;
        if !response.status().is_success() {
            return Err(BankaiError::Rpc(format!(
                "subgraph request failed with status {}",
                response.status()
            )));
        }
        let body: SubgraphResponse = response.json().await?;
        if let Some(errors) = body.errors {
            if let Some(first) = errors.first() {
                return Err(BankaiError::Rpc(format!(
                    "subgraph error: {}",
                    first.message
                )));
            }
        }
        Ok(body
            .data
            .map(|data| data.market_resolutions)
            .unwrap_or_default())
    }

    fn encode_redeem_positions(&self, request: &RedemptionRequest) -> Result<Bytes> {
        let function = self.ctf_abi.function("redeemPositions").map_err(|err| {
            BankaiError::InvalidArgument(format!(
                "redeemPositions function missing from ABI: {err}"
            ))
        })?;
        let tokens = vec![
            Token::Address(self.collateral_token),
            Token::FixedBytes(request.parent_collection_id.as_bytes().to_vec()),
            Token::FixedBytes(request.condition_id.as_bytes().to_vec()),
            Token::Array(
                request
                    .index_sets
                    .iter()
                    .map(|value| Token::Uint(*value))
                    .collect(),
            ),
        ];
        let data = function.encode_input(&tokens).map_err(|err| {
            BankaiError::InvalidArgument(format!("redeemPositions encode failed: {err}"))
        })?;
        Ok(Bytes::from(data))
    }

    async fn fetch_event_logs(
        &self,
        address: Address,
        event: &Event,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<ethers_core::types::Log>> {
        let filter = Filter::new()
            .address(address)
            .topic0(event.signature())
            .from_block(BlockNumber::Number(U64::from(from_block)))
            .to_block(BlockNumber::Number(U64::from(to_block)));
        self.provider
            .get_logs(&filter)
            .await
            .map_err(|err| BankaiError::Rpc(format!("resolution log fetch failed: {err}")))
    }
}

#[derive(Debug, Clone)]
struct AdapterState {
    address: Address,
    config: AdapterConfig,
    next_block: Option<u64>,
}

/// Background listener that polls resolution events and triggers redemption.
pub struct RedemptionListener<R: PositionResolver> {
    client: RedemptionClient,
    redis: RedisManager,
    resolver: R,
    ctf_next_block: Option<u64>,
    adapter_states: Vec<AdapterState>,
    processed_conditions: HashSet<H256>,
    subgraph_cursor: Option<u64>,
    last_subgraph_poll: Option<Instant>,
}

impl<R: PositionResolver + 'static> RedemptionListener<R> {
    /// Create a redemption listener with the provided client and resolver.
    pub fn new(client: RedemptionClient, redis: RedisManager, resolver: R) -> Self {
        let start_block = client.config().start_block;
        let adapter_states = build_adapter_states(client.config().adapter_configs.as_slice());
        Self {
            client,
            redis,
            resolver,
            ctf_next_block: start_block,
            adapter_states,
            processed_conditions: HashSet::new(),
            subgraph_cursor: None,
            last_subgraph_poll: None,
        }
    }

    /// Spawn the redemption polling task.
    pub fn spawn(mut self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Err(error) = self.poll_once().await {
                    tracing::error!(?error, "redemption listener polling failed");
                }
                sleep(self.client.config().poll_interval).await;
            }
        })
    }

    async fn poll_once(&mut self) -> Result<()> {
        self.poll_ctf_events().await?;
        self.poll_adapter_events().await?;
        self.poll_subgraph_backfill().await?;
        Ok(())
    }

    async fn poll_ctf_events(&mut self) -> Result<()> {
        let latest = self.client.latest_block().await?;
        let safe_block = latest.saturating_sub(self.client.config().min_confirmations);
        let from_block = match self.ctf_next_block {
            Some(block) => block,
            None => {
                self.ctf_next_block = Some(safe_block);
                safe_block
            }
        };
        if from_block > safe_block {
            return Ok(());
        }

        let max_range = self.client.config().max_block_range.max(1);
        let to_block = if safe_block - from_block > max_range {
            from_block + max_range
        } else {
            safe_block
        };

        let events = self
            .client
            .fetch_resolution_events(from_block, to_block)
            .await?;
        for event in events {
            let resolved = ResolvedCondition {
                condition_id: event.condition_id,
                source: ResolutionSource::Ctf,
                tx_hash: event.tx_hash,
                block_number: event.block_number,
            };
            self.handle_resolution(resolved).await?;
        }

        self.ctf_next_block = Some(to_block.saturating_add(1));
        Ok(())
    }

    async fn poll_adapter_events(&mut self) -> Result<()> {
        if self.adapter_states.is_empty() {
            return Ok(());
        }
        let latest = self.client.latest_block().await?;
        let safe_block = latest.saturating_sub(self.client.config().min_confirmations);

        for index in 0..self.adapter_states.len() {
            let (address, default_outcome_slot_count, from_block) = {
                let state = &mut self.adapter_states[index];
                let from_block = match state.next_block {
                    Some(block) => block,
                    None => {
                        let start = state.config.start_block.unwrap_or(safe_block);
                        state.next_block = Some(start);
                        start
                    }
                };
                (
                    state.address,
                    state.config.default_outcome_slot_count,
                    from_block,
                )
            };

            if from_block > safe_block {
                continue;
            }

            let max_range = self.client.config().max_block_range.max(1);
            let to_block = if safe_block - from_block > max_range {
                from_block + max_range
            } else {
                safe_block
            };

            let events = self
                .client
                .fetch_adapter_resolution_events(
                    address,
                    from_block,
                    to_block,
                    default_outcome_slot_count,
                )
                .await?;
            for event in events {
                self.handle_resolution(event).await?;
            }

            {
                let state = &mut self.adapter_states[index];
                state.next_block = Some(to_block.saturating_add(1));
            }
        }

        Ok(())
    }

    async fn poll_subgraph_backfill(&mut self) -> Result<()> {
        let subgraph = match self.client.config().subgraph.clone() {
            Some(config) => config,
            None => return Ok(()),
        };
        if self.adapter_states.is_empty() {
            return Ok(());
        }

        let now = Instant::now();
        if let Some(last) = self.last_subgraph_poll {
            if now.duration_since(last) < subgraph.poll_interval {
                return Ok(());
            }
        }
        self.last_subgraph_poll = Some(now);

        let adapter_specs: Vec<(Address, u32)> = self
            .adapter_states
            .iter()
            .map(|state| (state.address, state.config.default_outcome_slot_count))
            .collect();

        let since = self.subgraph_cursor.unwrap_or(0);
        let resolutions = self
            .client
            .fetch_subgraph_resolutions(&subgraph.url, since, subgraph.max_items)
            .await?;
        if resolutions.is_empty() {
            return Ok(());
        }

        let mut newest_timestamp = since;
        for resolution in resolutions {
            let question_id = parse_question_id(&resolution.id)?;
            let timestamp = parse_timestamp(&resolution.last_update_timestamp)?;
            if timestamp > newest_timestamp {
                newest_timestamp = timestamp;
            }

            for (address, default_outcome_slot_count) in &adapter_specs {
                let outcome_slot_count = subgraph
                    .default_outcome_slot_count
                    .max(1)
                    .max(*default_outcome_slot_count);
                let condition_id = compute_condition_id(*address, question_id, outcome_slot_count);
                let resolved = ResolvedCondition {
                    condition_id,
                    source: ResolutionSource::Subgraph {
                        adapter: *address,
                        question_id,
                    },
                    tx_hash: None,
                    block_number: None,
                };
                self.handle_resolution(resolved).await?;
            }
        }

        self.subgraph_cursor = Some(newest_timestamp);
        Ok(())
    }

    async fn handle_resolution(&mut self, resolved: ResolvedCondition) -> Result<()> {
        if self.processed_conditions.contains(&resolved.condition_id) {
            return Ok(());
        }

        match &resolved.source {
            ResolutionSource::Ctf => {
                tracing::info!(
                    condition_id = %format!("{:?}", resolved.condition_id),
                    source = resolved.source.label(),
                    "condition resolved"
                );
            }
            ResolutionSource::Adapter {
                address,
                question_id,
            } => {
                tracing::info!(
                    condition_id = %format!("{:?}", resolved.condition_id),
                    source = resolved.source.label(),
                    adapter = %format!("{:?}", address),
                    question_id = %format!("{:?}", question_id),
                    "condition resolved"
                );
            }
            ResolutionSource::Subgraph {
                adapter,
                question_id,
            } => {
                tracing::info!(
                    condition_id = %format!("{:?}", resolved.condition_id),
                    source = resolved.source.label(),
                    adapter = %format!("{:?}", adapter),
                    question_id = %format!("{:?}", question_id),
                    "condition resolved"
                );
            }
        }

        let targets = self.resolver.resolve_targets(resolved.condition_id).await?;
        if targets.is_empty() {
            tracing::info!(
                condition_id = %format!("{:?}", resolved.condition_id),
                source = resolved.source.label(),
                "no positions to redeem"
            );
            self.processed_conditions.insert(resolved.condition_id);
            return Ok(());
        }

        for target in targets {
            let request = RedemptionRequest::from_target(
                resolved.condition_id,
                target,
                self.client.parent_collection_id(),
            )?;
            let outcome = self.client.redeem_positions(&request).await?;
            tracing::info!(
                tx_hash = %format!("{:?}", outcome.tx_hash),
                source = resolved.source.label(),
                "redeemPositions confirmed"
            );
            self.update_bankroll().await?;
        }

        self.processed_conditions.insert(resolved.condition_id);
        Ok(())
    }

    async fn update_bankroll(&self) -> Result<()> {
        let balance = self.client.fetch_collateral_balance().await?;
        let scaled = scale_u256(balance, self.client.config().collateral_decimals)?;
        self.redis.set_float(BANKROLL_REDIS_KEY, scaled).await?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct SubgraphResponse {
    data: Option<SubgraphData>,
    errors: Option<Vec<SubgraphError>>,
}

#[derive(Debug, Deserialize)]
struct SubgraphData {
    #[serde(rename = "marketResolutions")]
    market_resolutions: Vec<SubgraphResolution>,
}

#[derive(Debug, Deserialize)]
struct SubgraphResolution {
    id: String,
    #[serde(rename = "lastUpdateTimestamp")]
    last_update_timestamp: String,
}

#[derive(Debug, Deserialize)]
struct SubgraphError {
    message: String,
}

fn build_wallet(secrets: &Secrets, chain_id: u64) -> Result<LocalWallet> {
    let key = secrets
        .polygon_private_key
        .as_ref()
        .ok_or_else(|| BankaiError::InvalidArgument("polygon private key missing".to_string()))?;
    let trimmed = key.expose_secret().trim();
    if trimmed.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "polygon private key is empty".to_string(),
        ));
    }
    let wallet = LocalWallet::from_str(trimmed)
        .map_err(|err| BankaiError::Crypto(format!("invalid private key: {err}")))?;
    Ok(wallet.with_chain_id(chain_id))
}

fn build_http_client(config: &RedemptionConfig) -> Result<Client> {
    Ok(Client::builder().timeout(config.request_timeout).build()?)
}

fn build_provider(config: &RedemptionConfig, client: Client) -> Result<Provider<Http>> {
    let url = reqwest::Url::parse(config.rpc_url.trim())
        .map_err(|_| BankaiError::InvalidArgument("polygon rpc url is invalid".to_string()))?;
    let http = Http::new_with_client(url, client);
    Ok(Provider::new(http))
}

fn load_ctf_abi(path: &Path) -> Result<Abi> {
    let raw = std::fs::read_to_string(path)?;
    let stripped = strip_jsdoc_header(&raw)?;
    let trimmed = stripped.trim();
    if trimmed.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "conditional tokens abi file is empty".to_string(),
        ));
    }
    let value: Value = serde_json::from_str(trimmed)?;
    match value {
        Value::Array(_) => Ok(serde_json::from_value(value)?),
        Value::Object(mut map) => {
            let abi_value = map.remove("abi").ok_or_else(|| {
                BankaiError::InvalidArgument("conditional tokens abi missing".to_string())
            })?;
            Ok(serde_json::from_value(abi_value)?)
        }
        _ => Err(BankaiError::InvalidArgument(
            "conditional tokens abi must be an array or object".to_string(),
        )),
    }
}

fn strip_jsdoc_header(contents: &str) -> Result<&str> {
    let start_index = contents
        .char_indices()
        .find(|(_, c)| !c.is_whitespace())
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    if !contents[start_index..].starts_with("/**") {
        return Ok(contents);
    }

    let header_start = start_index + 3;
    let header_end = contents[header_start..]
        .find("*/")
        .ok_or(BankaiError::InvalidHeader)?;
    let content_start = header_start + header_end + 2;

    Ok(&contents[content_start..])
}

fn parse_address(value: &str, field: &str) -> Result<Address> {
    Address::from_str(value.trim())
        .map_err(|_| BankaiError::InvalidArgument(format!("{field} is not a valid address")))
}

fn parse_parent_collection_id(value: &str) -> Result<H256> {
    if value.trim().is_empty() {
        return Ok(H256::zero());
    }
    H256::from_str(value.trim())
        .map_err(|_| BankaiError::InvalidArgument("parent collection id is invalid".to_string()))
}

#[allow(deprecated)]
fn build_balance_of_function() -> Function {
    Function {
        name: "balanceOf".to_string(),
        inputs: vec![Param {
            name: "account".to_string(),
            kind: ParamType::Address,
            internal_type: None,
        }],
        outputs: vec![Param {
            name: "balance".to_string(),
            kind: ParamType::Uint(256),
            internal_type: None,
        }],
        constant: None,
        state_mutability: StateMutability::View,
    }
}

fn build_adapter_question_resolved_event() -> Event {
    Event {
        name: "QuestionResolved".to_string(),
        inputs: vec![
            EventParam {
                name: "questionID".to_string(),
                kind: ParamType::FixedBytes(32),
                indexed: true,
            },
            EventParam {
                name: "price".to_string(),
                kind: ParamType::Int(256),
                indexed: true,
            },
            EventParam {
                name: "payouts".to_string(),
                kind: ParamType::Array(Box::new(ParamType::Uint(256))),
                indexed: false,
            },
        ],
        anonymous: false,
    }
}

fn build_adapter_question_resolved_legacy_event() -> Event {
    Event {
        name: "QuestionResolved".to_string(),
        inputs: vec![
            EventParam {
                name: "questionID".to_string(),
                kind: ParamType::FixedBytes(32),
                indexed: true,
            },
            EventParam {
                name: "resolved".to_string(),
                kind: ParamType::Bool,
                indexed: true,
            },
        ],
        anonymous: false,
    }
}

fn decode_balance(tokens: &[Token]) -> Result<U256> {
    let token = tokens.first().ok_or_else(|| {
        BankaiError::InvalidArgument("balanceOf returned empty response".to_string())
    })?;
    match token {
        Token::Uint(value) => Ok(*value),
        _ => Err(BankaiError::InvalidArgument(
            "balanceOf returned unexpected type".to_string(),
        )),
    }
}

fn extract_condition_id(params: &[LogParam]) -> Result<H256> {
    for param in params {
        if param.name == "conditionId" {
            if let Token::FixedBytes(bytes) = &param.value {
                if bytes.len() != 32 {
                    return Err(BankaiError::InvalidArgument(
                        "conditionId has invalid length".to_string(),
                    ));
                }
                return Ok(H256::from_slice(bytes));
            }
        }
    }
    Err(BankaiError::InvalidArgument(
        "conditionId missing from ConditionResolution log".to_string(),
    ))
}

fn extract_question_id(params: &[LogParam]) -> Result<H256> {
    for param in params {
        if param.name.eq_ignore_ascii_case("questionid") {
            if let Token::FixedBytes(bytes) = &param.value {
                if bytes.len() != 32 {
                    return Err(BankaiError::InvalidArgument(
                        "questionId has invalid length".to_string(),
                    ));
                }
                return Ok(H256::from_slice(bytes));
            }
        }
    }
    Err(BankaiError::InvalidArgument(
        "questionId missing from QuestionResolved log".to_string(),
    ))
}

fn extract_payouts_len(params: &[LogParam]) -> Option<usize> {
    for param in params {
        if param.name == "payouts" {
            if let Token::Array(items) = &param.value {
                return Some(items.len());
            }
        }
    }
    None
}

fn parse_log(event: &Event, log: &ethers_core::types::Log) -> Result<ethers_core::abi::Log> {
    let raw = RawLog {
        topics: log.topics.clone(),
        data: log.data.to_vec(),
    };
    event
        .parse_log(raw)
        .map_err(|err| BankaiError::InvalidArgument(format!("failed to parse log: {err}")))
}

fn compute_condition_id(oracle: Address, question_id: H256, outcome_slot_count: u32) -> H256 {
    let mut bytes = Vec::with_capacity(20 + 32 + 32);
    bytes.extend_from_slice(oracle.as_bytes());
    bytes.extend_from_slice(question_id.as_bytes());
    let mut count_bytes = [0u8; 32];
    U256::from(outcome_slot_count).to_big_endian(&mut count_bytes);
    bytes.extend_from_slice(&count_bytes);
    H256::from(keccak256(bytes))
}

fn parse_question_id(value: &str) -> Result<H256> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "subgraph question id is empty".to_string(),
        ));
    }
    H256::from_str(trimmed)
        .map_err(|_| BankaiError::InvalidArgument("subgraph question id is invalid".to_string()))
}

fn parse_timestamp(value: &str) -> Result<u64> {
    value
        .parse::<u64>()
        .map_err(|_| BankaiError::InvalidArgument("subgraph timestamp is invalid".to_string()))
}

fn build_adapter_states(configs: &[AdapterConfig]) -> Vec<AdapterState> {
    let mut states = Vec::new();
    for config in configs {
        match parse_address(&config.address, "adapter address") {
            Ok(address) => states.push(AdapterState {
                address,
                config: config.clone(),
                next_block: config.start_block,
            }),
            Err(error) => {
                tracing::warn!(?error, "skipping invalid adapter config");
            }
        }
    }
    states
}

async fn resolve_nonce(provider: &Provider<Http>, wallet: &LocalWallet) -> Result<U256> {
    provider
        .get_transaction_count(
            wallet.address(),
            Some(ethers_core::types::BlockId::Number(BlockNumber::Pending)),
        )
        .await
        .map_err(|err| BankaiError::Rpc(format!("nonce fetch failed: {err}")))
}

async fn resolve_gas_limit(
    provider: &Provider<Http>,
    tx: &TypedTransaction,
    config: &RedemptionConfig,
) -> Result<U256> {
    if let Some(limit) = config.gas_limit {
        return Ok(limit);
    }
    provider
        .estimate_gas(tx, None)
        .await
        .map_err(|err| BankaiError::Rpc(format!("gas estimation failed: {err}")))
}

async fn resolve_fee_data(
    provider: &Provider<Http>,
    config: &RedemptionConfig,
) -> Result<(U256, U256)> {
    let mut max_fee = config.max_fee_per_gas_gwei.map(gwei_to_wei);
    let mut max_priority = config.max_priority_fee_gwei.map(gwei_to_wei);

    if max_fee.is_none() || max_priority.is_none() {
        let (estimated_max_fee, estimated_priority) = provider
            .estimate_eip1559_fees(None)
            .await
            .map_err(|err| BankaiError::Rpc(format!("fee data fetch failed: {err}")))?;
        if max_fee.is_none() {
            max_fee = Some(estimated_max_fee);
        }
        if max_priority.is_none() {
            max_priority = Some(estimated_priority);
        }
    }

    let max_fee = max_fee.unwrap_or_else(|| gwei_to_wei(DEFAULT_MAX_FEE_GWEI));
    let mut max_priority = max_priority.unwrap_or_else(|| gwei_to_wei(DEFAULT_PRIORITY_FEE_GWEI));
    if max_priority > max_fee {
        max_priority = max_fee;
    }

    Ok((max_fee, max_priority))
}

fn gwei_to_wei(value: u64) -> U256 {
    U256::from(value) * U256::from(WEI_PER_GWEI)
}

fn scale_u256(value: U256, decimals: u32) -> Result<f64> {
    if decimals == 0 {
        return value.to_string().parse::<f64>().map_err(|_| {
            BankaiError::InvalidArgument("failed to parse integer balance".to_string())
        });
    }
    let raw = value.to_string();
    let decimals = decimals as usize;
    let scaled = if raw.len() <= decimals {
        let zeros = "0".repeat(decimals - raw.len());
        format!("0.{zeros}{raw}")
    } else {
        let split = raw.len() - decimals;
        format!("{}.{}", &raw[..split], &raw[split..])
    };
    scaled
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("failed to parse scaled balance".to_string()))
}
