/**
 * @description
 * Encryption and runtime secret management for API keys and private keys.
 *
 * @dependencies
 * - aes-gcm: AES-256-GCM authenticated encryption
 * - argon2: password-based key derivation
 * - base64: encoding for encrypted payload storage
 * - secrecy: secret string handling with zeroize on drop
 * - rpassword: hidden password input from TTY
 *
 * @notes
 * - Secrets are persisted only in encrypted form at rest.
 * - Decrypted secrets remain in memory as SecretString values and drop on exit.
 */
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Nonce};
use argon2::Argon2;
use base64::{engine::general_purpose, Engine as _};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use crate::error::{BankaiError, Result};

const SECRETS_VERSION: u8 = 1;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;

pub const DEFAULT_SECRETS_PATH: &str = "config/secrets.enc";

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedSecrets {
    version: u8,
    salt: String,
    nonce: String,
    ciphertext: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsPayload {
    pub polygon_private_key: Option<String>,
    pub polymarket_api_key: Option<String>,
    pub polymarket_api_secret: Option<String>,
    pub polymarket_api_passphrase: Option<String>,
    pub allora_api_key: Option<String>,
}

pub struct Secrets {
    pub polygon_private_key: Option<SecretString>,
    pub polymarket_api_key: Option<SecretString>,
    pub polymarket_api_secret: Option<SecretString>,
    pub polymarket_api_passphrase: Option<SecretString>,
    pub allora_api_key: Option<SecretString>,
}

impl Secrets {
    fn from_payload(payload: SecretsPayload) -> Self {
        Self {
            polygon_private_key: map_secret(payload.polygon_private_key),
            polymarket_api_key: map_secret(payload.polymarket_api_key),
            polymarket_api_secret: map_secret(payload.polymarket_api_secret),
            polymarket_api_passphrase: map_secret(payload.polymarket_api_passphrase),
            allora_api_key: map_secret(payload.allora_api_key),
        }
    }
}

fn map_secret(value: Option<String>) -> Option<SecretString> {
    value.and_then(|item| {
        if item.is_empty() {
            None
        } else {
            Some(SecretString::new(item))
        }
    })
}

pub fn prompt_password(prompt: &str) -> Result<SecretString> {
    print!("{prompt}");
    io::stdout().flush()?;
    let password = rpassword::read_password()?;
    Ok(SecretString::new(password))
}

pub fn prompt_new_password() -> Result<SecretString> {
    let password = prompt_password("Create secrets password: ")?;
    let confirmation = prompt_password("Confirm secrets password: ")?;
    if password.expose_secret() != confirmation.expose_secret() {
        return Err(BankaiError::InvalidArgument(
            "password confirmation does not match".to_string(),
        ));
    }
    Ok(password)
}

pub fn encrypt_to_file(
    path: impl AsRef<Path>,
    password: &SecretString,
    payload: &SecretsPayload,
) -> Result<()> {
    let encrypted = encrypt_payload(password, payload)?;
    write_encrypted_file(path.as_ref(), &encrypted)
}

pub fn load_secrets(path: impl AsRef<Path>, password: &SecretString) -> Result<Secrets> {
    let encrypted = read_encrypted_file(path.as_ref())?;
    let payload = decrypt_payload(password, &encrypted)?;
    Ok(Secrets::from_payload(payload))
}

pub fn load_secrets_interactive(path: impl AsRef<Path>) -> Result<Secrets> {
    let password = prompt_password("Enter secrets password: ")?;
    load_secrets(path, &password)
}

fn encrypt_payload(password: &SecretString, payload: &SecretsPayload) -> Result<EncryptedSecrets> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);

    let key = derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|_| BankaiError::Crypto("invalid key length".to_string()))?;
    let plaintext = serde_json::to_vec(payload)?;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_ref())
        .map_err(|_| BankaiError::Crypto("encryption failed".to_string()))?;

    Ok(EncryptedSecrets {
        version: SECRETS_VERSION,
        salt: general_purpose::STANDARD.encode(salt),
        nonce: general_purpose::STANDARD.encode(nonce_bytes),
        ciphertext: general_purpose::STANDARD.encode(ciphertext),
    })
}

fn decrypt_payload(password: &SecretString, encrypted: &EncryptedSecrets) -> Result<SecretsPayload> {
    if encrypted.version != SECRETS_VERSION {
        return Err(BankaiError::InvalidArgument(format!(
            "unsupported secrets version {}",
            encrypted.version
        )));
    }

    let salt = general_purpose::STANDARD.decode(&encrypted.salt)?;
    let nonce_bytes = general_purpose::STANDARD.decode(&encrypted.nonce)?;
    let ciphertext = general_purpose::STANDARD.decode(&encrypted.ciphertext)?;

    if salt.len() != SALT_LEN {
        return Err(BankaiError::InvalidArgument(
            "invalid salt length".to_string(),
        ));
    }
    if nonce_bytes.len() != NONCE_LEN {
        return Err(BankaiError::InvalidArgument(
            "invalid nonce length".to_string(),
        ));
    }

    let key = derive_key(password, &salt)?;
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|_| BankaiError::Crypto("invalid key length".to_string()))?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce_bytes), ciphertext.as_ref())
        .map_err(|_| BankaiError::Crypto("decryption failed".to_string()))?;

    Ok(serde_json::from_slice(&plaintext)?)
}

fn derive_key(password: &SecretString, salt: &[u8]) -> Result<[u8; 32]> {
    let mut key = [0u8; 32];
    Argon2::default().hash_password_into(
        password.expose_secret().as_bytes(),
        salt,
        &mut key,
    )?;
    Ok(key)
}

fn read_encrypted_file(path: &Path) -> Result<EncryptedSecrets> {
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(raw.trim())?)
}

fn write_encrypted_file(path: &Path, encrypted: &EncryptedSecrets) -> Result<()> {
    let serialized = serde_json::to_vec(encrypted)?;
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, serialized)?;
    Ok(())
}
