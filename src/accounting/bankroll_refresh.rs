/**
 * @purpose
 * Periodic collateral balance refresh into Redis bankroll key.
 *
 * @notes
 * - Uses ERC20 balanceOf on the Polymarket collateral token.
 * - Intended to keep sys:bankroll:usdc current while trading.
 */
use ethers_core::abi::{Function, Param, ParamType, StateMutability, Token};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_core::types::{Address, Bytes, TransactionRequest, U256};
use ethers_providers::{Http, Middleware, Provider};
use std::time::Duration;

use crate::accounting::utils::scale_u256;
use crate::config::Config;
use crate::error::{BankaiError, Result};
use crate::execution::signer::Eip712Signer;
use crate::security::Secrets;
use crate::storage::redis::RedisManager;

const DEFAULT_CHAIN_ID: u64 = 137;
const DEFAULT_COLLATERAL_DECIMALS: u32 = 6;
const DEFAULT_REQUEST_TIMEOUT_MS: u64 = 4_000;
const BANKROLL_REDIS_KEY: &str = "sys:bankroll:usdc";

const ENV_CHAIN_ID: &str = "POLYGON_CHAIN_ID";
const ENV_COLLATERAL_ADDRESS: &str = "POLYMARKET_COLLATERAL_ADDRESS";
const ENV_COLLATERAL_DECIMALS: &str = "POLYMARKET_COLLATERAL_DECIMALS";

pub struct BankrollRefresher {
    provider: Provider<Http>,
    collateral_token: Address,
    collateral_decimals: u32,
    wallet_address: Address,
    erc20_balance_of: Function,
    redis: RedisManager,
    interval: Duration,
}

impl BankrollRefresher {
    pub fn from_env(
        config: &Config,
        secrets: &Secrets,
        redis: RedisManager,
    ) -> Result<Option<Self>> {
        let collateral_token = read_env_value(ENV_COLLATERAL_ADDRESS);
        if collateral_token.is_none() {
            tracing::warn!("{ENV_COLLATERAL_ADDRESS} missing; bankroll refresh disabled");
            return Ok(None);
        }
        let chain_id = read_env_u64(ENV_CHAIN_ID)?.unwrap_or(DEFAULT_CHAIN_ID);
        let wallet = match Eip712Signer::from_secrets(secrets, chain_id) {
            Ok(signer) => signer.address(),
            Err(error) => {
                tracing::warn!(
                    ?error,
                    "failed to derive wallet address; bankroll refresh disabled"
                );
                return Ok(None);
            }
        };
        let collateral_decimals =
            read_env_u32(ENV_COLLATERAL_DECIMALS)?.unwrap_or(DEFAULT_COLLATERAL_DECIMALS);
        let provider = build_provider(config)?;
        let collateral_token =
            parse_address(collateral_token.as_ref().unwrap(), "collateral token")?;
        Ok(Some(Self {
            provider,
            collateral_token,
            collateral_decimals,
            wallet_address: wallet,
            erc20_balance_of: build_erc20_balance_of_function(),
            redis,
            interval: Duration::from_secs(config.execution.bankroll_refresh_secs.max(10)),
        }))
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run().await {
                tracing::error!(?error, "bankroll refresher stopped");
            }
        })
    }

    async fn run(self) -> Result<()> {
        let mut interval = tokio::time::interval(self.interval);
        loop {
            interval.tick().await;
            if let Err(error) = self.refresh().await {
                tracing::warn!(?error, "bankroll refresh failed");
            }
        }
    }

    async fn refresh(&self) -> Result<()> {
        let balance = self.fetch_collateral_balance().await?;
        let scaled = scale_u256(balance, self.collateral_decimals)?;
        self.redis.set_float(BANKROLL_REDIS_KEY, scaled).await?;
        Ok(())
    }

    async fn fetch_collateral_balance(&self) -> Result<U256> {
        let data = self
            .erc20_balance_of
            .encode_input(&[Token::Address(self.wallet_address)])
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
        let decoded = self
            .erc20_balance_of
            .decode_output(raw.as_ref())
            .map_err(|err| {
                BankaiError::InvalidArgument(format!("balanceOf decode failed: {err}"))
            })?;
        decode_balance(&decoded)
    }
}

fn build_provider(config: &Config) -> Result<Provider<Http>> {
    let url = reqwest::Url::parse(config.endpoints.polygon_rpc.trim())
        .map_err(|_| BankaiError::InvalidArgument("polygon rpc url is invalid".to_string()))?;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(DEFAULT_REQUEST_TIMEOUT_MS))
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

fn parse_address(value: &str, field: &str) -> Result<Address> {
    use std::str::FromStr;
    Address::from_str(value.trim())
        .map_err(|_| BankaiError::InvalidArgument(format!("{field} is invalid")))
}

fn read_env_value(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

fn read_env_u64(key: &str) -> Result<Option<u64>> {
    match std::env::var(key) {
        Ok(value) => value
            .parse::<u64>()
            .map(Some)
            .map_err(|_| BankaiError::InvalidArgument(format!("{key} must be numeric"))),
        Err(_) => Ok(None),
    }
}

fn read_env_u32(key: &str) -> Result<Option<u32>> {
    match std::env::var(key) {
        Ok(value) => value
            .parse::<u32>()
            .map(Some)
            .map_err(|_| BankaiError::InvalidArgument(format!("{key} must be numeric"))),
        Err(_) => Ok(None),
    }
}
