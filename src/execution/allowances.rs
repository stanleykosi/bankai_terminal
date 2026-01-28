/**
 * @purpose
 * Maintain ERC20/CTF approvals for Polymarket execution.
 *
 * @dependencies
 * - ethers-core: ABI encoding + transaction types
 * - ethers-providers: JSON-RPC calls
 * - ethers-signers: local wallet signing
 *
 * @notes
 * - Uses on-chain approvals only; CLOB relayer still required for matching.
 */
use ethers_core::abi::{Function, Param, ParamType, StateMutability, Token};
use ethers_core::types::transaction::eip1559::Eip1559TransactionRequest;
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Bytes, TransactionRequest, U256};
use ethers_providers::{Http, Middleware, Provider};
use ethers_signers::{LocalWallet, Signer};
use secrecy::ExposeSecret;
use std::str::FromStr;
use std::time::Duration;

use crate::config::Config;
use crate::error::{BankaiError, Result};
use crate::security::Secrets;
use crate::storage::redis::RedisManager;

const DEFAULT_CHAIN_ID: u64 = 137;
const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 4_000;
const DEFAULT_COLLATERAL_DECIMALS: u32 = 6;
const DEFAULT_MAX_FEE_GWEI: u64 = 50;
const DEFAULT_PRIORITY_FEE_GWEI: u64 = 2;
const WEI_PER_GWEI: u64 = 1_000_000_000;

const ENV_CHAIN_ID: &str = "POLYGON_CHAIN_ID";
const ENV_COLLATERAL_ADDRESS: &str = "POLYMARKET_COLLATERAL_ADDRESS";
const ENV_COLLATERAL_DECIMALS: &str = "POLYMARKET_COLLATERAL_DECIMALS";
const ENV_CTF_ADDRESS: &str = "POLYMARKET_CTF_ADDRESS";
const ENV_EXCHANGE_ADDRESS: &str = "POLYMARKET_EXCHANGE_ADDRESS";

pub struct AllowanceManager {
    provider: Provider<Http>,
    wallet: LocalWallet,
    collateral_token: Address,
    conditional_tokens: Address,
    exchange: Address,
    collateral_decimals: u32,
    target_allowance: f64,
    interval: Duration,
    redis: Option<RedisManager>,
}

impl AllowanceManager {
    pub fn from_env(
        config: &Config,
        secrets: &Secrets,
        redis: Option<RedisManager>,
    ) -> Result<Option<Self>> {
        let collateral_token = read_env_value(ENV_COLLATERAL_ADDRESS);
        let conditional_tokens = read_env_value(ENV_CTF_ADDRESS);
        let exchange = read_env_value(ENV_EXCHANGE_ADDRESS);
        if collateral_token.is_none() || conditional_tokens.is_none() || exchange.is_none() {
            tracing::warn!(
                "{ENV_COLLATERAL_ADDRESS}, {ENV_CTF_ADDRESS}, or {ENV_EXCHANGE_ADDRESS} missing; allowance manager disabled"
            );
            return Ok(None);
        }
        let rpc_url = config.endpoints.polygon_rpc.trim().to_string();
        if rpc_url.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "polygon rpc url is required for allowances".to_string(),
            ));
        }
        let chain_id = read_env_u64(ENV_CHAIN_ID)?.unwrap_or(DEFAULT_CHAIN_ID);
        let decimals =
            read_env_u32(ENV_COLLATERAL_DECIMALS)?.unwrap_or(DEFAULT_COLLATERAL_DECIMALS);

        let wallet = build_wallet(secrets, chain_id)?;
        let provider = build_provider(&rpc_url, Duration::from_millis(DEFAULT_REQUEST_TIMEOUT_MS))?;

        let target_allowance = config.execution.allowance_target_usdc;
        let interval = Duration::from_secs(config.execution.allowance_check_interval_secs.max(10));

        Ok(Some(Self {
            provider,
            wallet,
            collateral_token: parse_address(&collateral_token.unwrap(), "collateral token")?,
            conditional_tokens: parse_address(&conditional_tokens.unwrap(), "conditional tokens")?,
            exchange: parse_address(&exchange.unwrap(), "ctf exchange")?,
            collateral_decimals: decimals,
            target_allowance,
            interval,
            redis,
        }))
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "allowance manager stopped");
            }
        })
    }

    async fn run(self) -> Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        loop {
            interval.tick().await;
            if let Err(error) = self.ensure_allowances().await {
                tracing::warn!(?error, "allowance check failed");
            }
        }
    }

    async fn ensure_allowances(&self) -> Result<()> {
        if self.target_allowance <= 0.0 {
            return Ok(());
        }
        let target = to_fixed_u256(self.target_allowance, self.collateral_decimals)?;
        let allowance = self
            .fetch_erc20_allowance(self.collateral_token, self.exchange)
            .await?;
        if allowance < target {
            let tx = self
                .send_erc20_approve(self.collateral_token, self.exchange, target)
                .await?;
            self.log_activity(format!(
                "Allowance set: USDC -> {} (tx={:?})",
                format_address(self.exchange),
                tx
            ))
            .await;
        }

        let approved = self
            .fetch_erc1155_approval(self.conditional_tokens, self.exchange)
            .await?;
        if !approved {
            let tx = self
                .send_erc1155_approval(self.conditional_tokens, self.exchange, true)
                .await?;
            self.log_activity(format!(
                "Approval set: CTF -> {} (tx={:?})",
                format_address(self.exchange),
                tx
            ))
            .await;
        }
        Ok(())
    }

    async fn fetch_erc20_allowance(&self, token: Address, spender: Address) -> Result<U256> {
        let function = build_erc20_allowance()?;
        let data = function
            .encode_input(&[
                Token::Address(self.wallet.address()),
                Token::Address(spender),
            ])
            .map_err(|err| {
                BankaiError::InvalidArgument(format!("allowance encode failed: {err}"))
            })?;
        let tx = TransactionRequest {
            to: Some(token.into()),
            data: Some(Bytes::from(data)),
            ..Default::default()
        };
        let call: TypedTransaction = tx.into();
        let raw = self
            .provider
            .call(&call, None)
            .await
            .map_err(|err| BankaiError::Rpc(format!("allowance call failed: {err}")))?;
        let decoded = function.decode_output(raw.as_ref()).map_err(|err| {
            BankaiError::InvalidArgument(format!("allowance decode failed: {err}"))
        })?;
        decode_u256(&decoded)
    }

    async fn send_erc20_approve(
        &self,
        token: Address,
        spender: Address,
        amount: U256,
    ) -> Result<ethers_core::types::H256> {
        let function = build_erc20_approve()?;
        let data = function
            .encode_input(&[Token::Address(spender), Token::Uint(amount)])
            .map_err(|err| BankaiError::InvalidArgument(format!("approve encode failed: {err}")))?;
        self.send_call_data(token, Bytes::from(data)).await
    }

    async fn fetch_erc1155_approval(&self, token: Address, operator: Address) -> Result<bool> {
        let function = build_erc1155_is_approved()?;
        let data = function
            .encode_input(&[
                Token::Address(self.wallet.address()),
                Token::Address(operator),
            ])
            .map_err(|err| {
                BankaiError::InvalidArgument(format!("isApprovedForAll encode failed: {err}"))
            })?;
        let tx = TransactionRequest {
            to: Some(token.into()),
            data: Some(Bytes::from(data)),
            ..Default::default()
        };
        let call: TypedTransaction = tx.into();
        let raw = self
            .provider
            .call(&call, None)
            .await
            .map_err(|err| BankaiError::Rpc(format!("isApprovedForAll call failed: {err}")))?;
        let decoded = function.decode_output(raw.as_ref()).map_err(|err| {
            BankaiError::InvalidArgument(format!("isApprovedForAll decode failed: {err}"))
        })?;
        decode_bool(&decoded)
    }

    async fn send_erc1155_approval(
        &self,
        token: Address,
        operator: Address,
        approved: bool,
    ) -> Result<ethers_core::types::H256> {
        let function = build_erc1155_set_approval()?;
        let data = function
            .encode_input(&[Token::Address(operator), Token::Bool(approved)])
            .map_err(|err| {
                BankaiError::InvalidArgument(format!("setApprovalForAll encode failed: {err}"))
            })?;
        self.send_call_data(token, Bytes::from(data)).await
    }

    async fn send_call_data(
        &self,
        to: Address,
        call_data: Bytes,
    ) -> Result<ethers_core::types::H256> {
        let tx = Eip1559TransactionRequest {
            from: Some(self.wallet.address()),
            to: Some(to.into()),
            data: Some(call_data),
            chain_id: Some(self.wallet.chain_id().into()),
            ..Default::default()
        };
        let mut typed_tx = TypedTransaction::Eip1559(tx);
        let nonce = self
            .provider
            .get_transaction_count(self.wallet.address(), None)
            .await
            .map_err(|err| BankaiError::Rpc(format!("nonce fetch failed: {err}")))?;
        typed_tx.set_nonce(nonce);

        let gas_limit = self
            .provider
            .estimate_gas(&typed_tx, None)
            .await
            .map_err(|err| BankaiError::Rpc(format!("gas estimation failed: {err}")))?;
        typed_tx.set_gas(gas_limit);

        let (max_fee, max_priority) = resolve_fee_data(&self.provider).await?;
        if let Some(inner) = typed_tx.as_eip1559_mut() {
            inner.max_fee_per_gas = Some(max_fee);
            inner.max_priority_fee_per_gas = Some(max_priority);
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
        Ok(pending.tx_hash())
    }

    async fn log_activity(&self, message: String) {
        if let Some(redis) = self.redis.as_ref() {
            let _ = redis.push_activity_log(&message, 12).await;
        }
    }
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

fn build_provider(rpc_url: &str, timeout: Duration) -> Result<Provider<Http>> {
    let url = reqwest::Url::parse(rpc_url.trim())
        .map_err(|_| BankaiError::InvalidArgument("polygon rpc url is invalid".to_string()))?;
    let client = reqwest::Client::builder().timeout(timeout).build()?;
    Ok(Provider::new(Http::new_with_client(url, client)))
}

#[allow(deprecated)]
fn build_erc20_allowance() -> Result<Function> {
    Ok(Function {
        name: "allowance".to_string(),
        inputs: vec![
            Param {
                name: "owner".to_string(),
                kind: ParamType::Address,
                internal_type: None,
            },
            Param {
                name: "spender".to_string(),
                kind: ParamType::Address,
                internal_type: None,
            },
        ],
        outputs: vec![Param {
            name: "amount".to_string(),
            kind: ParamType::Uint(256),
            internal_type: None,
        }],
        constant: None,
        state_mutability: StateMutability::View,
    })
}

#[allow(deprecated)]
fn build_erc20_approve() -> Result<Function> {
    Ok(Function {
        name: "approve".to_string(),
        inputs: vec![
            Param {
                name: "spender".to_string(),
                kind: ParamType::Address,
                internal_type: None,
            },
            Param {
                name: "amount".to_string(),
                kind: ParamType::Uint(256),
                internal_type: None,
            },
        ],
        outputs: vec![Param {
            name: "success".to_string(),
            kind: ParamType::Bool,
            internal_type: None,
        }],
        constant: None,
        state_mutability: StateMutability::NonPayable,
    })
}

#[allow(deprecated)]
fn build_erc1155_is_approved() -> Result<Function> {
    Ok(Function {
        name: "isApprovedForAll".to_string(),
        inputs: vec![
            Param {
                name: "account".to_string(),
                kind: ParamType::Address,
                internal_type: None,
            },
            Param {
                name: "operator".to_string(),
                kind: ParamType::Address,
                internal_type: None,
            },
        ],
        outputs: vec![Param {
            name: "approved".to_string(),
            kind: ParamType::Bool,
            internal_type: None,
        }],
        constant: None,
        state_mutability: StateMutability::View,
    })
}

#[allow(deprecated)]
fn build_erc1155_set_approval() -> Result<Function> {
    Ok(Function {
        name: "setApprovalForAll".to_string(),
        inputs: vec![
            Param {
                name: "operator".to_string(),
                kind: ParamType::Address,
                internal_type: None,
            },
            Param {
                name: "approved".to_string(),
                kind: ParamType::Bool,
                internal_type: None,
            },
        ],
        outputs: vec![],
        constant: None,
        state_mutability: StateMutability::NonPayable,
    })
}

fn decode_u256(tokens: &[Token]) -> Result<U256> {
    match tokens.first() {
        Some(Token::Uint(value)) => Ok(*value),
        _ => Err(BankaiError::InvalidArgument(
            "unexpected allowance output".to_string(),
        )),
    }
}

fn decode_bool(tokens: &[Token]) -> Result<bool> {
    match tokens.first() {
        Some(Token::Bool(value)) => Ok(*value),
        _ => Err(BankaiError::InvalidArgument(
            "unexpected approval output".to_string(),
        )),
    }
}

async fn resolve_fee_data(provider: &Provider<Http>) -> Result<(U256, U256)> {
    let (estimated_max_fee, estimated_priority) = provider
        .estimate_eip1559_fees(None)
        .await
        .map_err(|err| BankaiError::Rpc(format!("fee data fetch failed: {err}")))?;
    let max_fee = if estimated_max_fee.is_zero() {
        gwei_to_wei(DEFAULT_MAX_FEE_GWEI)
    } else {
        estimated_max_fee
    };
    let mut max_priority = if estimated_priority.is_zero() {
        gwei_to_wei(DEFAULT_PRIORITY_FEE_GWEI)
    } else {
        estimated_priority
    };
    if max_priority > max_fee {
        max_priority = max_fee;
    }
    Ok((max_fee, max_priority))
}

fn gwei_to_wei(value: u64) -> U256 {
    U256::from(value) * U256::from(WEI_PER_GWEI)
}

fn to_fixed_u256(value: f64, decimals: u32) -> Result<U256> {
    if value < 0.0 {
        return Err(BankaiError::InvalidArgument(
            "negative amount not allowed".to_string(),
        ));
    }
    let factor = 10_f64.powi(decimals as i32);
    let scaled = (value * factor).floor();
    let scaled = if scaled < 0.0 { 0.0 } else { scaled };
    Ok(U256::from(scaled as u128))
}

fn parse_address(value: &str, field: &str) -> Result<Address> {
    Address::from_str(value.trim())
        .map_err(|_| BankaiError::InvalidArgument(format!("{field} is not a valid address")))
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
    raw.parse::<u64>()
        .map(Some)
        .map_err(|_| BankaiError::InvalidArgument(format!("{key} must be a valid integer")))
}

fn read_env_u32(key: &str) -> Result<Option<u32>> {
    let Some(raw) = read_env_value(key) else {
        return Ok(None);
    };
    raw.parse::<u32>()
        .map(Some)
        .map_err(|_| BankaiError::InvalidArgument(format!("{key} must be a valid integer")))
}

fn format_address(address: Address) -> String {
    format!("{address}").to_lowercase()
}
