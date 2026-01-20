/**
 * @purpose
 * Direct (Rail B) execution client for sending raw fillOrders transactions to CTFExchange.
 *
 * @dependencies
 * - ethers-core: ABI encoding and transaction types
 * - ethers-providers: JSON-RPC transport
 * - ethers-signers: local wallet signer
 * - reqwest: HTTP client with custom headers
 *
 * @notes
 * - Uses EIP-1559 transactions and supports private RPC headers.
 */
use ethers_core::abi::{Abi, Token};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Bytes, Eip1559TransactionRequest, H256, U256};
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::{LocalWallet, Signer};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use secrecy::ExposeSecret;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use crate::error::{BankaiError, Result};
use crate::security::Secrets;

const DEFAULT_ABI_PATH: &str = "abi/CTFExchange.json";
const DEFAULT_TIMEOUT_MS: u64 = 3500;
const DEFAULT_MAX_FEE_GWEI: u64 = 50;
const DEFAULT_PRIORITY_FEE_GWEI: u64 = 2;
const WEI_PER_GWEI: u64 = 1_000_000_000;

#[derive(Debug, Clone)]
pub struct DirectExecutionConfig {
    pub rpc_url: String,
    pub exchange_address: String,
    pub chain_id: u64,
    pub request_timeout: Duration,
    pub abi_path: String,
    pub gas_limit: Option<U256>,
    pub max_fee_per_gas_gwei: Option<u64>,
    pub max_priority_fee_gwei: Option<u64>,
    pub private_rpc_headers: HashMap<String, String>,
}

impl DirectExecutionConfig {
    pub fn new(rpc_url: String, exchange_address: String, chain_id: u64) -> Self {
        Self {
            rpc_url,
            exchange_address,
            chain_id,
            request_timeout: Duration::from_millis(DEFAULT_TIMEOUT_MS),
            abi_path: DEFAULT_ABI_PATH.to_string(),
            gas_limit: None,
            max_fee_per_gas_gwei: None,
            max_priority_fee_gwei: None,
            private_rpc_headers: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DirectCallOptions {
    pub nonce: Option<U256>,
    pub gas_limit: Option<U256>,
    pub max_fee_per_gas_wei: Option<U256>,
    pub max_priority_fee_per_gas_wei: Option<U256>,
    pub value: Option<U256>,
}

#[derive(Debug, Clone)]
pub struct ExchangeOrder {
    pub salt: U256,
    pub maker: Address,
    pub signer: Address,
    pub taker: Address,
    pub token_id: U256,
    pub maker_amount: U256,
    pub taker_amount: U256,
    pub expiration: U256,
    pub nonce: U256,
    pub fee_rate_bps: u64,
    pub side: u8,
    pub signature_type: u8,
    pub signature: Bytes,
}

impl ExchangeOrder {
    pub fn validate(&self) -> Result<()> {
        if self.maker_amount.is_zero() {
            return Err(BankaiError::InvalidArgument(
                "maker_amount must be non-zero".to_string(),
            ));
        }
        if self.taker_amount.is_zero() {
            return Err(BankaiError::InvalidArgument(
                "taker_amount must be non-zero".to_string(),
            ));
        }
        if self.signature.as_ref().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "signature must be non-empty".to_string(),
            ));
        }
        Ok(())
    }

    fn to_token(&self) -> Token {
        Token::Tuple(vec![
            Token::Uint(self.salt),
            Token::Address(self.maker),
            Token::Address(self.signer),
            Token::Address(self.taker),
            Token::Uint(self.token_id),
            Token::Uint(self.maker_amount),
            Token::Uint(self.taker_amount),
            Token::Uint(self.expiration),
            Token::Uint(self.nonce),
            Token::Uint(U256::from(self.fee_rate_bps)),
            Token::Uint(U256::from(self.side)),
            Token::Uint(U256::from(self.signature_type)),
            Token::Bytes(self.signature.to_vec()),
        ])
    }
}

#[derive(Debug, Clone)]
pub struct FillOrdersRequest {
    pub orders: Vec<ExchangeOrder>,
    pub fill_amounts: Vec<U256>,
    pub options: DirectCallOptions,
}

impl FillOrdersRequest {
    pub fn new(orders: Vec<ExchangeOrder>, fill_amounts: Vec<U256>) -> Self {
        Self {
            orders,
            fill_amounts,
            options: DirectCallOptions::default(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        if self.orders.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "fill orders request requires at least one order".to_string(),
            ));
        }
        if self.orders.len() != self.fill_amounts.len() {
            return Err(BankaiError::InvalidArgument(
                "fill orders request length mismatch".to_string(),
            ));
        }
        for order in &self.orders {
            order.validate()?;
        }
        Ok(())
    }

    fn to_tokens(&self) -> Result<Vec<Token>> {
        self.validate()?;
        let orders = self
            .orders
            .iter()
            .map(ExchangeOrder::to_token)
            .collect::<Vec<_>>();
        let fill_amounts = self
            .fill_amounts
            .iter()
            .map(|amount| Token::Uint(*amount))
            .collect::<Vec<_>>();
        Ok(vec![Token::Array(orders), Token::Array(fill_amounts)])
    }
}

#[derive(Debug, Clone)]
pub struct DirectExecutionResult {
    pub tx_hash: H256,
    pub nonce: U256,
    pub gas_limit: U256,
    pub max_fee_per_gas: U256,
    pub max_priority_fee_per_gas: U256,
}

pub struct DirectExecutionClient {
    config: DirectExecutionConfig,
    provider: Provider<Http>,
    wallet: LocalWallet,
    exchange_abi: Abi,
    exchange_address: Address,
}

impl DirectExecutionClient {
    pub fn new(config: DirectExecutionConfig, secrets: &Secrets) -> Result<Self> {
        if config.rpc_url.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "polygon rpc url is required".to_string(),
            ));
        }
        if config.exchange_address.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "ctf exchange address is required".to_string(),
            ));
        }

        let exchange_address = parse_address(&config.exchange_address, "ctf exchange address")?;
        let wallet = build_wallet(secrets, config.chain_id)?;
        let provider = build_provider(&config)?;
        let abi = load_exchange_abi(Path::new(&config.abi_path))?;

        Ok(Self {
            config,
            provider,
            wallet,
            exchange_abi: abi,
            exchange_address,
        })
    }

    pub fn config(&self) -> &DirectExecutionConfig {
        &self.config
    }

    pub fn wallet_address(&self) -> Address {
        self.wallet.address()
    }

    pub fn exchange_address(&self) -> Address {
        self.exchange_address
    }

    pub async fn fetch_chain_nonce(&self) -> Result<U256> {
        self.provider
            .get_transaction_count(self.wallet.address(), None)
            .await
            .map_err(|err| BankaiError::Rpc(format!("nonce fetch failed: {err}")))
    }

    pub fn encode_fill_orders(&self, request: &FillOrdersRequest) -> Result<Bytes> {
        let function = self
            .exchange_abi
            .function("fillOrders")
            .map_err(|err| {
                BankaiError::InvalidArgument(format!(
                    "fillOrders function missing from ABI: {err}"
                ))
            })?;
        let tokens = request.to_tokens()?;
        let data = function.encode_input(&tokens).map_err(|err| {
            BankaiError::InvalidArgument(format!("fillOrders encode failed: {err}"))
        })?;
        Ok(Bytes::from(data))
    }

    pub async fn send_fill_orders(&self, request: &FillOrdersRequest) -> Result<DirectExecutionResult> {
        let calldata = self.encode_fill_orders(request)?;
        self.send_call_data(calldata, &request.options).await
    }

    pub async fn send_call_data(
        &self,
        call_data: Bytes,
        options: &DirectCallOptions,
    ) -> Result<DirectExecutionResult> {
        let value = options.value.unwrap_or_default();
        let tx = Eip1559TransactionRequest {
            from: Some(self.wallet.address()),
            to: Some(self.exchange_address.into()),
            value: Some(value),
            data: Some(call_data),
            chain_id: Some(self.config.chain_id.into()),
            ..Default::default()
        };

        let mut typed_tx = TypedTransaction::Eip1559(tx);
        let nonce = resolve_nonce(&self.provider, &self.wallet, options).await?;
        typed_tx.set_nonce(nonce);

        let gas_limit = resolve_gas_limit(&self.provider, &typed_tx, options, &self.config).await?;
        typed_tx.set_gas(gas_limit);

        let (max_fee_per_gas, max_priority_fee_per_gas) =
            resolve_fee_data(&self.provider, options, &self.config).await?;
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
            .map_err(|err| BankaiError::Rpc(format!("send raw transaction failed: {err}")))?;

        Ok(DirectExecutionResult {
            tx_hash: pending.tx_hash(),
            nonce,
            gas_limit,
            max_fee_per_gas,
            max_priority_fee_per_gas,
        })
    }
}

fn build_wallet(secrets: &Secrets, chain_id: u64) -> Result<LocalWallet> {
    let key = secrets.polygon_private_key.as_ref().ok_or_else(|| {
        BankaiError::InvalidArgument("polygon private key missing".to_string())
    })?;
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

fn build_provider(config: &DirectExecutionConfig) -> Result<Provider<Http>> {
    let client = build_http_client(config)?;
    let url = reqwest::Url::parse(config.rpc_url.trim()).map_err(|_| {
        BankaiError::InvalidArgument("polygon rpc url is invalid".to_string())
    })?;
    let http = Http::new_with_client(url, client);
    Ok(Provider::new(http))
}

fn build_http_client(config: &DirectExecutionConfig) -> Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder().timeout(config.request_timeout);
    if !config.private_rpc_headers.is_empty() {
        let headers = build_rpc_headers(&config.private_rpc_headers)?;
        builder = builder.default_headers(headers);
    }
    Ok(builder.build()?)
}

fn build_rpc_headers(headers: &HashMap<String, String>) -> Result<HeaderMap> {
    let mut map = HeaderMap::new();
    for (key, value) in headers {
        let name = HeaderName::from_str(key).map_err(|_| {
            BankaiError::InvalidArgument(format!("invalid rpc header name: {key}"))
        })?;
        let value = HeaderValue::from_str(value).map_err(|_| {
            BankaiError::InvalidArgument(format!("invalid rpc header value for {key}"))
        })?;
        map.insert(name, value);
    }
    Ok(map)
}

fn load_exchange_abi(path: &Path) -> Result<Abi> {
    let raw = fs::read_to_string(path)?;
    let stripped = strip_jsdoc_header(&raw)?;
    let trimmed = stripped.trim();
    if trimmed.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "ctf exchange abi file is empty".to_string(),
        ));
    }
    let value: Value = serde_json::from_str(trimmed)?;
    match value {
        Value::Array(_) => Ok(serde_json::from_value(value)?),
        Value::Object(mut map) => {
            let abi_value = map
                .remove("abi")
                .ok_or_else(|| BankaiError::InvalidArgument("ctf exchange abi missing".to_string()))?;
            Ok(serde_json::from_value(abi_value)?)
        }
        _ => Err(BankaiError::InvalidArgument(
            "ctf exchange abi must be an array or object".to_string(),
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
    Address::from_str(value.trim()).map_err(|_| {
        BankaiError::InvalidArgument(format!("{field} is not a valid address"))
    })
}

async fn resolve_nonce(
    provider: &Provider<Http>,
    wallet: &LocalWallet,
    options: &DirectCallOptions,
) -> Result<U256> {
    if let Some(nonce) = options.nonce {
        return Ok(nonce);
    }
    provider
        .get_transaction_count(wallet.address(), None)
        .await
        .map_err(|err| BankaiError::Rpc(format!("nonce fetch failed: {err}")))
}

async fn resolve_gas_limit(
    provider: &Provider<Http>,
    tx: &TypedTransaction,
    options: &DirectCallOptions,
    config: &DirectExecutionConfig,
) -> Result<U256> {
    if let Some(limit) = options.gas_limit {
        return Ok(limit);
    }
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
    options: &DirectCallOptions,
    config: &DirectExecutionConfig,
) -> Result<(U256, U256)> {
    let mut max_fee = options
        .max_fee_per_gas_wei
        .or_else(|| config.max_fee_per_gas_gwei.map(gwei_to_wei));
    let mut max_priority = options
        .max_priority_fee_per_gas_wei
        .or_else(|| config.max_priority_fee_gwei.map(gwei_to_wei));

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
    let mut max_priority =
        max_priority.unwrap_or_else(|| gwei_to_wei(DEFAULT_PRIORITY_FEE_GWEI));
    if max_priority > max_fee {
        max_priority = max_fee;
    }

    Ok((max_fee, max_priority))
}

fn gwei_to_wei(value: u64) -> U256 {
    U256::from(value) * U256::from(WEI_PER_GWEI)
}
