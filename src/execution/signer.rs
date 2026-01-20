/**
 * @description
 * EIP-712 signing helpers for Polymarket authentication and order payloads.
 *
 * @dependencies
 * - ethers-core: typed data structures
 * - ethers-signers: local wallet signer
 * - secrecy: secret key handling
 *
 * @notes
 * - ClobAuth signatures follow the Polymarket CLOB auth domain specification.
 */
use ethers_core::types::transaction::eip712::TypedData;
use ethers_core::types::{Address, Signature};
use ethers_signers::{LocalWallet, Signer};
use secrecy::{ExposeSecret, SecretString};
use serde_json::json;
use std::str::FromStr;

use crate::error::{BankaiError, Result};
use crate::security::Secrets;

const CLOB_AUTH_MESSAGE: &str = "This message attests that I control the given wallet";
const CLOB_AUTH_DOMAIN_NAME: &str = "ClobAuthDomain";
const CLOB_AUTH_DOMAIN_VERSION: &str = "1";

#[derive(Debug, Clone)]
pub struct Eip712Signer {
    wallet: LocalWallet,
    chain_id: u64,
}

impl Eip712Signer {
    pub fn from_private_key(private_key: &SecretString, chain_id: u64) -> Result<Self> {
        let trimmed = private_key.expose_secret().trim();
        if trimmed.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "polygon private key is empty".to_string(),
            ));
        }
        let wallet = LocalWallet::from_str(trimmed)
            .map_err(|err| BankaiError::Crypto(format!("invalid private key: {err}")))?;
        Ok(Self {
            wallet: wallet.with_chain_id(chain_id),
            chain_id,
        })
    }

    pub fn from_secrets(secrets: &Secrets, chain_id: u64) -> Result<Self> {
        let key = secrets.polygon_private_key.as_ref().ok_or_else(|| {
            BankaiError::InvalidArgument("polygon private key missing".to_string())
        })?;
        Self::from_private_key(key, chain_id)
    }

    pub fn address(&self) -> Address {
        self.wallet.address()
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    pub async fn sign_typed_data(&self, typed_data: &TypedData) -> Result<Signature> {
        self.wallet
            .sign_typed_data(typed_data)
            .await
            .map_err(|err| BankaiError::Crypto(format!("eip712 signing failed: {err}")))
    }

    pub async fn sign_typed_data_value(&self, value: serde_json::Value) -> Result<Signature> {
        let typed_data: TypedData = serde_json::from_value(value)?;
        self.sign_typed_data(&typed_data).await
    }

    pub fn clob_auth_typed_data(&self, timestamp: &str, nonce: u64) -> Result<TypedData> {
        build_clob_auth_typed_data(self.wallet.address(), self.chain_id, timestamp, nonce)
    }

    pub async fn sign_clob_auth(&self, timestamp: &str, nonce: u64) -> Result<Signature> {
        let typed_data = self.clob_auth_typed_data(timestamp, nonce)?;
        self.sign_typed_data(&typed_data).await
    }
}

fn build_clob_auth_typed_data(
    address: Address,
    chain_id: u64,
    timestamp: &str,
    nonce: u64,
) -> Result<TypedData> {
    if timestamp.trim().is_empty() {
        return Err(BankaiError::InvalidArgument(
            "timestamp is required for clob auth".to_string(),
        ));
    }

    let typed_data_value = json!({
        "types": {
            "EIP712Domain": [
                { "name": "name", "type": "string" },
                { "name": "version", "type": "string" },
                { "name": "chainId", "type": "uint256" }
            ],
            "ClobAuth": [
                { "name": "address", "type": "address" },
                { "name": "timestamp", "type": "string" },
                { "name": "nonce", "type": "uint256" },
                { "name": "message", "type": "string" }
            ]
        },
        "primaryType": "ClobAuth",
        "domain": {
            "name": CLOB_AUTH_DOMAIN_NAME,
            "version": CLOB_AUTH_DOMAIN_VERSION,
            "chainId": chain_id
        },
        "message": {
            "address": format!("{address}"),
            "timestamp": timestamp,
            "nonce": nonce,
            "message": CLOB_AUTH_MESSAGE
        }
    });

    Ok(serde_json::from_value(typed_data_value)?)
}
