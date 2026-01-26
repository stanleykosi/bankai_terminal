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
use ethers_core::types::{Address, Signature, U256};
use ethers_signers::{LocalWallet, Signer};
use secrecy::{ExposeSecret, SecretString};
use serde_json::json;
use std::str::FromStr;

use crate::error::{BankaiError, Result};
use crate::security::Secrets;

const CLOB_AUTH_MESSAGE: &str = "This message attests that I control the given wallet";
const CLOB_AUTH_DOMAIN_NAME: &str = "ClobAuthDomain";
const CLOB_AUTH_DOMAIN_VERSION: &str = "1";
const CTF_EXCHANGE_DOMAIN_NAME: &str = "Polymarket CTF Exchange";
const CTF_EXCHANGE_DOMAIN_VERSION: &str = "1";

#[derive(Debug, Clone)]
pub struct OrderSignaturePayload {
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
}

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

    pub fn order_typed_data(
        &self,
        order: &OrderSignaturePayload,
        verifying_contract: Address,
    ) -> Result<TypedData> {
        build_order_typed_data(
            order,
            verifying_contract,
            self.chain_id,
            CTF_EXCHANGE_DOMAIN_NAME,
            CTF_EXCHANGE_DOMAIN_VERSION,
        )
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

fn build_order_typed_data(
    order: &OrderSignaturePayload,
    verifying_contract: Address,
    chain_id: u64,
    name: &str,
    version: &str,
) -> Result<TypedData> {
    let typed_data_value = json!({
        "types": {
            "EIP712Domain": [
                { "name": "name", "type": "string" },
                { "name": "version", "type": "string" },
                { "name": "chainId", "type": "uint256" },
                { "name": "verifyingContract", "type": "address" }
            ],
            "Order": [
                { "name": "salt", "type": "uint256" },
                { "name": "maker", "type": "address" },
                { "name": "signer", "type": "address" },
                { "name": "taker", "type": "address" },
                { "name": "tokenId", "type": "uint256" },
                { "name": "makerAmount", "type": "uint256" },
                { "name": "takerAmount", "type": "uint256" },
                { "name": "expiration", "type": "uint256" },
                { "name": "nonce", "type": "uint256" },
                { "name": "feeRateBps", "type": "uint256" },
                { "name": "side", "type": "uint8" },
                { "name": "signatureType", "type": "uint8" }
            ]
        },
        "primaryType": "Order",
        "domain": {
            "name": name,
            "version": version,
            "chainId": chain_id,
            "verifyingContract": format!("{verifying_contract}")
        },
        "message": {
            "salt": format!("{}", order.salt),
            "maker": format!("{}", order.maker),
            "signer": format!("{}", order.signer),
            "taker": format!("{}", order.taker),
            "tokenId": format!("{}", order.token_id),
            "makerAmount": format!("{}", order.maker_amount),
            "takerAmount": format!("{}", order.taker_amount),
            "expiration": format!("{}", order.expiration),
            "nonce": format!("{}", order.nonce),
            "feeRateBps": format!("{}", order.fee_rate_bps),
            "side": order.side,
            "signatureType": order.signature_type
        }
    });

    Ok(serde_json::from_value(typed_data_value)?)
}
