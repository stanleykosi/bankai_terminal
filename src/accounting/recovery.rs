/**
 * @purpose
 * Startup recovery reconciles on-chain balances and open orders into Redis.
 *
 * @dependencies
 * - base64: decode API secrets for HMAC signatures
 * - ethers-core: ABI types for ERC20/ERC1155 balanceOf calls
 * - ethers-providers: JSON-RPC client for chain reads
 * - ethers-signers: wallet address derivation
 * - hmac/sha2: CLOB auth signature
 * - reqwest: CLOB HTTP client
 * - redis: reconcile hot state in Redis
 *
 * @notes
 * - Open orders are pulled from the Polymarket CLOB API when credentials exist.
 * - Chain balances are used to rehydrate bankroll and conditional token positions.
 */
use base64::engine::general_purpose;
use base64::Engine as _;
use ethers_core::abi::{Function, Param, ParamType, StateMutability, Token};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Bytes, TransactionRequest, U256};
use ethers_providers::{Http, Middleware, Provider};
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Url};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

use crate::config::Config;
use crate::error::{BankaiError, Result};
use crate::execution::signer::Eip712Signer;
use crate::security::Secrets;
use crate::storage::redis::RedisManager;

type HmacSha256 = Hmac<sha2::Sha256>;

const DEFAULT_CHAIN_ID: u64 = 137;
const DEFAULT_COLLATERAL_DECIMALS: u32 = 6;
const DEFAULT_CTF_ABI_PATH: &str = "abi/ConditionalTokens.json";
const DEFAULT_OPEN_ORDERS_PATH: &str = "/data/orders";
const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 4_000;
const DEFAULT_OPEN_ORDERS_LIMIT: usize = 200;

const BANKROLL_REDIS_KEY: &str = "sys:bankroll:usdc";
const POSITIONS_PREFIX: &str = "positions:ctf:";
const OPEN_ORDERS_PREFIX: &str = "orders:open:";
const OPEN_ORDERS_DETAILS_SUFFIX: &str = ":details";

const ENV_CHAIN_ID: &str = "POLYGON_CHAIN_ID";
const ENV_COLLATERAL_ADDRESS: &str = "POLYMARKET_COLLATERAL_ADDRESS";
const ENV_COLLATERAL_DECIMALS: &str = "POLYMARKET_COLLATERAL_DECIMALS";
const ENV_CTF_ADDRESS: &str = "POLYMARKET_CTF_ADDRESS";
const ENV_CLOB_BASE_URL: &str = "POLYMARKET_CLOB_URL";
const ENV_OPEN_ORDERS_PATH: &str = "POLYMARKET_OPEN_ORDERS_PATH";
const ENV_OPEN_ORDERS_LIMIT: &str = "POLYMARKET_OPEN_ORDERS_LIMIT";

const HEADER_POLY_ADDRESS: &str = "POLY_ADDRESS";
const HEADER_POLY_API_KEY: &str = "POLY_API_KEY";
const HEADER_POLY_PASSPHRASE: &str = "POLY_PASSPHRASE";
const HEADER_POLY_SIGNATURE: &str = "POLY_SIGNATURE";
const HEADER_POLY_TIMESTAMP: &str = "POLY_TIMESTAMP";

/// Runtime configuration for startup recovery.
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    pub polygon_rpc: String,
    pub chain_id: u64,
    pub collateral_token: String,
    pub collateral_decimals: u32,
    pub conditional_tokens: String,
    pub ctf_abi_path: String,
    pub clob_base_url: String,
    pub open_orders_path: String,
    pub request_timeout: Duration,
    pub open_orders_limit: usize,
    pub asset_ids: Vec<String>,
}

impl RecoveryConfig {
    /// Build the recovery config from environment variables and config defaults.
    pub fn from_env(config: &Config) -> Result<Option<Self>> {
        let collateral_token = read_env_value(ENV_COLLATERAL_ADDRESS);
        let conditional_tokens = read_env_value(ENV_CTF_ADDRESS);
        if collateral_token.is_none() || conditional_tokens.is_none() {
            tracing::warn!(
                "{ENV_COLLATERAL_ADDRESS} or {ENV_CTF_ADDRESS} missing; startup recovery disabled"
            );
            return Ok(None);
        }

        let polygon_rpc = config.endpoints.polygon_rpc.trim().to_string();
        if polygon_rpc.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "polygon rpc url is required for recovery".to_string(),
            ));
        }

        let chain_id = read_env_u64(ENV_CHAIN_ID)?.unwrap_or(DEFAULT_CHAIN_ID);
        let collateral_decimals =
            read_env_u32(ENV_COLLATERAL_DECIMALS)?.unwrap_or(DEFAULT_COLLATERAL_DECIMALS);
        let clob_base_url = read_env_value(ENV_CLOB_BASE_URL)
            .unwrap_or_else(|| config.endpoints.relayer_http.clone());
        let open_orders_path = read_env_value(ENV_OPEN_ORDERS_PATH)
            .unwrap_or_else(|| DEFAULT_OPEN_ORDERS_PATH.to_string());
        let open_orders_limit =
            read_env_usize(ENV_OPEN_ORDERS_LIMIT)?.unwrap_or(DEFAULT_OPEN_ORDERS_LIMIT);

        Ok(Some(Self {
            polygon_rpc,
            chain_id,
            collateral_token: collateral_token.unwrap(),
            collateral_decimals,
            conditional_tokens: conditional_tokens.unwrap(),
            ctf_abi_path: DEFAULT_CTF_ABI_PATH.to_string(),
            clob_base_url,
            open_orders_path,
            request_timeout: Duration::from_millis(DEFAULT_REQUEST_TIMEOUT_MS),
            open_orders_limit,
            asset_ids: config.polymarket.asset_ids.clone(),
        }))
    }
}

/// Summary of recovery results.
#[derive(Debug, Clone)]
pub struct RecoveryReport {
    pub collateral_balance: f64,
    pub positions_synced: usize,
    pub open_orders_synced: bool,
    pub open_orders_count: usize,
    pub asset_ids_seen: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpenOrderSnapshot {
    pub id: String,
    #[serde(default)]
    pub market: Option<String>,
    #[serde(default, rename = "asset_id")]
    pub asset_id: Option<String>,
    #[serde(default)]
    pub price: Option<String>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub size: Option<String>,
    #[serde(default, rename = "size_matched")]
    pub size_matched: Option<String>,
    #[serde(default, rename = "type")]
    pub order_type: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default)]
    pub expiration: Option<String>,
}

#[derive(Debug, Clone)]
struct ClobAuth {
    address: String,
    api_key: String,
    passphrase: String,
    secret: String,
}

struct ClobClient {
    base_url: String,
    orders_path: String,
    client: Client,
    auth: ClobAuth,
}

impl ClobClient {
    fn new(
        base_url: String,
        orders_path: String,
        timeout: Duration,
        auth: ClobAuth,
    ) -> Result<Self> {
        if base_url.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "clob base url is required".to_string(),
            ));
        }
        if orders_path.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "clob orders path is required".to_string(),
            ));
        }
        let client = Client::builder().timeout(timeout).build()?;
        Ok(Self {
            base_url,
            orders_path,
            client,
            auth,
        })
    }

    async fn fetch_open_orders(
        &self,
        asset_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<OpenOrderSnapshot>> {
        let mut url = Url::parse(self.base_url.trim_end_matches('/')).map_err(|_| {
            BankaiError::InvalidArgument("clob base url is invalid".to_string())
        })?;
        url.set_path(self.orders_path.trim_start_matches('/'));
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(asset_id) = asset_id {
                if !asset_id.trim().is_empty() {
                    pairs.append_pair("asset_id", asset_id.trim());
                }
            }
            if limit > 0 {
                pairs.append_pair("limit", &limit.to_string());
            }
        }

        let request_path = build_request_path(&url);
        let headers = build_clob_headers(&self.auth, "GET", &request_path, "")?;
        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(BankaiError::Rpc(format!(
                "clob orders request failed with status {}",
                response.status()
            )));
        }
        Ok(response.json::<Vec<OpenOrderSnapshot>>().await?)
    }
}

/// Startup recovery workflow runner.
pub struct StartupRecovery {
    config: RecoveryConfig,
    redis: RedisManager,
    provider: Provider<Http>,
    wallet_address: Address,
    wallet_key: String,
    collateral_token: Address,
    conditional_tokens: Address,
    erc20_balance_of: Function,
    erc1155_balance_of: Function,
    clob: Option<ClobClient>,
}

impl StartupRecovery {
    /// Initialize startup recovery when required settings are available.
    pub fn from_env(
        config: &Config,
        secrets: &Secrets,
        redis: RedisManager,
    ) -> Result<Option<Self>> {
        let Some(recovery_config) = RecoveryConfig::from_env(config)? else {
            return Ok(None);
        };

        let signer = Eip712Signer::from_secrets(secrets, recovery_config.chain_id)?;
        let wallet_address = signer.address();
        let wallet_key = format!("{wallet_address}").to_ascii_lowercase();
        let provider = build_provider(&recovery_config)?;
        let collateral_token =
            parse_address(&recovery_config.collateral_token, "collateral token")?;
        let conditional_tokens =
            parse_address(&recovery_config.conditional_tokens, "conditional tokens")?;
        let clob = build_clob_client(&recovery_config, secrets, &wallet_key)?;

        Ok(Some(Self {
            config: recovery_config,
            redis,
            provider,
            wallet_address,
            wallet_key,
            collateral_token,
            conditional_tokens,
            erc20_balance_of: build_erc20_balance_of_function(),
            erc1155_balance_of: build_erc1155_balance_of_function(),
            clob,
        }))
    }

    /// Execute the recovery flow and reconcile Redis state.
    pub async fn run(&self) -> Result<RecoveryReport> {
        tracing::info!(
            wallet = %self.wallet_key,
            "startup recovery started"
        );

        let asset_ids = self.resolve_asset_ids().await?;
        let collateral_balance = self.fetch_collateral_balance().await?;
        let collateral_scaled =
            scale_u256(collateral_balance, self.config.collateral_decimals)?;
        self.redis
            .set_float(BANKROLL_REDIS_KEY, collateral_scaled)
            .await?;

        let positions_synced = if asset_ids.is_empty() {
            tracing::warn!("no asset ids available; skipping conditional token recovery");
            0
        } else {
            let positions = self.fetch_positions(&asset_ids).await?;
            self.reconcile_positions(&positions).await?;
            positions.len()
        };

        let mut open_orders_count = 0;
        let mut open_orders_synced = false;
        if let Some(clob) = self.clob.as_ref() {
            match self.fetch_open_orders(clob, &asset_ids).await {
                Ok(orders) => {
                    open_orders_count = orders.len();
                    self.reconcile_open_orders(&orders).await?;
                    open_orders_synced = true;
                }
                Err(error) => {
                    tracing::warn!(?error, "open orders recovery failed");
                }
            }
        } else {
            tracing::warn!("polymarket api credentials missing; open orders not recovered");
        }

        tracing::info!(
            collateral_balance = collateral_scaled,
            positions_synced,
            open_orders_count,
            "startup recovery finished"
        );

        Ok(RecoveryReport {
            collateral_balance: collateral_scaled,
            positions_synced,
            open_orders_synced,
            open_orders_count,
            asset_ids_seen: asset_ids.len(),
        })
    }

    async fn resolve_asset_ids(&self) -> Result<Vec<String>> {
        if !self.config.asset_ids.is_empty() {
            return Ok(self.config.asset_ids.clone());
        }

        let mut asset_ids = self.redis.get_polymarket_asset_ids().await?;
        asset_ids.sort();
        Ok(asset_ids)
    }

    async fn fetch_collateral_balance(&self) -> Result<U256> {
        let data = self
            .erc20_balance_of
            .encode_input(&[Token::Address(self.wallet_address)])
            .map_err(|err| BankaiError::InvalidArgument(format!("balanceOf encode failed: {err}")))?;
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
        let decoded = self
            .erc20_balance_of
            .decode_output(raw.as_ref())
            .map_err(|err| BankaiError::InvalidArgument(format!("balanceOf decode failed: {err}")))?;
        decode_balance(&decoded)
    }

    async fn fetch_positions(&self, asset_ids: &[String]) -> Result<Vec<RecoveredPosition>> {
        let mut positions = Vec::with_capacity(asset_ids.len());
        for asset_id in asset_ids {
            let token_id = match parse_u256(asset_id, "asset_id") {
                Ok(value) => value,
                Err(error) => {
                    tracing::warn!(?error, asset_id = %asset_id, "invalid asset id");
                    continue;
                }
            };
            let balance = self.fetch_conditional_balance(token_id).await?;
            let scaled = scale_u256(balance, 0)?;
            positions.push(RecoveredPosition {
                asset_id: asset_id.clone(),
                balance: scaled,
            });
        }
        Ok(positions)
    }

    async fn fetch_conditional_balance(&self, token_id: U256) -> Result<U256> {
        let data = self
            .erc1155_balance_of
            .encode_input(&[
                Token::Address(self.wallet_address),
                Token::Uint(token_id),
            ])
            .map_err(|err| {
                BankaiError::InvalidArgument(format!("erc1155 balanceOf encode failed: {err}"))
            })?;
        let tx = TransactionRequest {
            to: Some(self.conditional_tokens.into()),
            data: Some(Bytes::from(data)),
            ..Default::default()
        };
        let call: TypedTransaction = tx.into();
        let raw = self
            .provider
            .call(&call, None)
            .await
            .map_err(|err| BankaiError::Rpc(format!("erc1155 balanceOf call failed: {err}")))?;
        let decoded = self
            .erc1155_balance_of
            .decode_output(raw.as_ref())
            .map_err(|err| {
                BankaiError::InvalidArgument(format!("erc1155 balanceOf decode failed: {err}"))
            })?;
        decode_balance(&decoded)
    }

    async fn fetch_open_orders(
        &self,
        clob: &ClobClient,
        asset_ids: &[String],
    ) -> Result<Vec<OpenOrderSnapshot>> {
        let mut orders = HashMap::new();
        if asset_ids.is_empty() {
            let batch = clob
                .fetch_open_orders(None, self.config.open_orders_limit)
                .await?;
            insert_orders(&mut orders, batch);
        } else {
            for asset_id in asset_ids {
                let batch = clob
                    .fetch_open_orders(Some(asset_id), self.config.open_orders_limit)
                    .await?;
                insert_orders(&mut orders, batch);
            }
        }
        Ok(orders.into_values().collect())
    }

    async fn reconcile_positions(&self, positions: &[RecoveredPosition]) -> Result<()> {
        let key = positions_key(&self.wallet_key);
        let mut conn = self.redis.connection();
        let mut pipe = redis::pipe();
        pipe.del(&key);
        for position in positions {
            pipe.hset(&key, &position.asset_id, position.balance);
        }
        pipe.query_async::<_, ()>(&mut conn).await?;
        Ok(())
    }

    async fn reconcile_open_orders(&self, orders: &[OpenOrderSnapshot]) -> Result<()> {
        let open_key = open_orders_key(&self.wallet_key);
        let details_key = open_orders_details_key(&self.wallet_key);
        let mut conn = self.redis.connection();
        let mut pipe = redis::pipe();
        pipe.del(&open_key);
        pipe.del(&details_key);

        if !orders.is_empty() {
            let ids: Vec<String> = orders.iter().map(|order| order.id.clone()).collect();
            pipe.sadd(&open_key, ids);
            for order in orders {
                let payload = serde_json::to_string(order)?;
                pipe.hset(&details_key, &order.id, payload);
            }
        }

        pipe.query_async::<_, ()>(&mut conn).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct RecoveredPosition {
    asset_id: String,
    balance: f64,
}

fn insert_orders(
    orders: &mut HashMap<String, OpenOrderSnapshot>,
    batch: Vec<OpenOrderSnapshot>,
) {
    for order in batch {
        if order.id.trim().is_empty() {
            continue;
        }
        orders.insert(order.id.clone(), order);
    }
}

fn build_clob_client(
    config: &RecoveryConfig,
    secrets: &Secrets,
    wallet_address: &str,
) -> Result<Option<ClobClient>> {
    let api_key = secrets
        .polymarket_api_key
        .as_ref()
        .map(|value| value.expose_secret().trim().to_string())
        .filter(|value| !value.is_empty());
    let passphrase = secrets
        .polymarket_api_passphrase
        .as_ref()
        .map(|value| value.expose_secret().trim().to_string())
        .filter(|value| !value.is_empty());
    let secret = secrets
        .polymarket_api_secret
        .as_ref()
        .map(|value| value.expose_secret().trim().to_string())
        .filter(|value| !value.is_empty());

    let (Some(api_key), Some(passphrase), Some(secret)) = (api_key, passphrase, secret) else {
        return Ok(None);
    };

    let auth = ClobAuth {
        address: wallet_address.to_string(),
        api_key,
        passphrase,
        secret,
    };

    let client = ClobClient::new(
        config.clob_base_url.clone(),
        config.open_orders_path.clone(),
        config.request_timeout,
        auth,
    )?;
    Ok(Some(client))
}

fn build_clob_headers(
    auth: &ClobAuth,
    method: &str,
    path: &str,
    body: &str,
) -> Result<HeaderMap> {
    let timestamp = current_unix_timestamp();
    let signature = build_hmac_signature(&auth.secret, timestamp, method, path, body)?;

    let mut headers = HeaderMap::new();
    headers.insert(HEADER_POLY_ADDRESS, header_value(&auth.address)?);
    headers.insert(HEADER_POLY_API_KEY, header_value(&auth.api_key)?);
    headers.insert(HEADER_POLY_PASSPHRASE, header_value(&auth.passphrase)?);
    headers.insert(HEADER_POLY_SIGNATURE, header_value(&signature)?);
    headers.insert(HEADER_POLY_TIMESTAMP, header_value(&timestamp.to_string())?);
    Ok(headers)
}

fn build_hmac_signature(
    secret: &str,
    timestamp: u64,
    method: &str,
    path: &str,
    body: &str,
) -> Result<String> {
    let key = general_purpose::STANDARD.decode(secret).map_err(|err| {
        BankaiError::InvalidArgument(format!("api secret decode error: {err}"))
    })?;
    let message = format!("{timestamp}{method}{path}{body}");
    let mut mac = HmacSha256::new_from_slice(&key).map_err(|_| {
        BankaiError::InvalidArgument("api secret is invalid for hmac".to_string())
    })?;
    mac.update(message.as_bytes());
    let result = mac.finalize().into_bytes();
    let signature = general_purpose::STANDARD.encode(result);
    Ok(signature.replace('+', "-").replace('/', "_"))
}

fn build_request_path(url: &Url) -> String {
    match url.query() {
        Some(query) => format!("{}?{}", url.path(), query),
        None => url.path().to_string(),
    }
}

fn build_provider(config: &RecoveryConfig) -> Result<Provider<Http>> {
    let url = reqwest::Url::parse(config.polygon_rpc.trim()).map_err(|_| {
        BankaiError::InvalidArgument("polygon rpc url is invalid".to_string())
    })?;
    let client = reqwest::Client::builder()
        .timeout(config.request_timeout)
        .build()?;
    let http = Http::new_with_client(url, client);
    Ok(Provider::new(http))
}

#[allow(deprecated)]
fn build_erc20_balance_of_function() -> Function {
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

#[allow(deprecated)]
fn build_erc1155_balance_of_function() -> Function {
    Function {
        name: "balanceOf".to_string(),
        inputs: vec![
            Param {
                name: "account".to_string(),
                kind: ParamType::Address,
                internal_type: None,
            },
            Param {
                name: "id".to_string(),
                kind: ParamType::Uint(256),
                internal_type: None,
            },
        ],
        outputs: vec![Param {
            name: "balance".to_string(),
            kind: ParamType::Uint(256),
            internal_type: None,
        }],
        constant: None,
        state_mutability: StateMutability::View,
    }
}

fn decode_balance(tokens: &[Token]) -> Result<U256> {
    let first = tokens.first().ok_or_else(|| {
        BankaiError::InvalidArgument("balanceOf returned empty response".to_string())
    })?;
    match first {
        Token::Uint(value) => Ok(*value),
        _ => Err(BankaiError::InvalidArgument(
            "balanceOf returned unexpected type".to_string(),
        )),
    }
}

fn scale_u256(value: U256, decimals: u32) -> Result<f64> {
    let raw = value.to_string();
    if decimals == 0 {
        return raw.parse::<f64>().map_err(|_| {
            BankaiError::InvalidArgument("failed to parse integer balance".to_string())
        });
    }

    let decimals = decimals as usize;
    let scaled = if raw.len() <= decimals {
        let mut padded = String::from("0.");
        padded.push_str(&"0".repeat(decimals - raw.len()));
        padded.push_str(&raw);
        padded
    } else {
        let split = raw.len() - decimals;
        format!("{}.{}", &raw[..split], &raw[split..])
    };

    scaled.parse::<f64>().map_err(|_| {
        BankaiError::InvalidArgument("failed to parse scaled balance".to_string())
    })
}

fn parse_u256(value: &str, field: &str) -> Result<U256> {
    U256::from_dec_str(value.trim()).map_err(|_| {
        BankaiError::InvalidArgument(format!("{field} is not a valid integer"))
    })
}

fn parse_address(value: &str, field: &str) -> Result<Address> {
    Address::from_str(value.trim()).map_err(|_| {
        BankaiError::InvalidArgument(format!("{field} is not a valid address"))
    })
}

fn positions_key(address: &str) -> String {
    format!("{POSITIONS_PREFIX}{address}")
}

fn open_orders_key(address: &str) -> String {
    format!("{OPEN_ORDERS_PREFIX}{address}")
}

fn open_orders_details_key(address: &str) -> String {
    format!("{OPEN_ORDERS_PREFIX}{address}{OPEN_ORDERS_DETAILS_SUFFIX}")
}

fn read_env_value(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn read_env_u64(key: &str) -> Result<Option<u64>> {
    let Some(raw) = read_env_value(key) else {
        return Ok(None);
    };
    raw.parse::<u64>().map(Some).map_err(|_| {
        BankaiError::InvalidArgument(format!("{key} must be a valid integer"))
    })
}

fn read_env_u32(key: &str) -> Result<Option<u32>> {
    let Some(raw) = read_env_value(key) else {
        return Ok(None);
    };
    raw.parse::<u32>().map(Some).map_err(|_| {
        BankaiError::InvalidArgument(format!("{key} must be a valid integer"))
    })
}

fn read_env_usize(key: &str) -> Result<Option<usize>> {
    let Some(raw) = read_env_value(key) else {
        return Ok(None);
    };
    raw.parse::<usize>().map(Some).map_err(|_| {
        BankaiError::InvalidArgument(format!("{key} must be a valid integer"))
    })
}

fn header_value(value: &str) -> Result<HeaderValue> {
    HeaderValue::from_str(value).map_err(|_| {
        BankaiError::InvalidArgument("header value contains invalid characters".to_string())
    })
}

fn current_unix_timestamp() -> u64 {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    duration.as_secs()
}
