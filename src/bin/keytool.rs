#![allow(clippy::result_large_err)]

/**
 * @description
 * CLI utility to capture credentials and write an encrypted secrets.enc file.
 *
 * @dependencies
 * - bankai_terminal::security: encryption helpers and password prompts
 * - secrecy: secret handling for input normalization
 *
 * @notes
 * - Avoids printing secrets to stdout or logs.
 */
use bankai_terminal::error::{BankaiError, Result};
use bankai_terminal::security::{
    encrypt_to_file, prompt_new_password, prompt_password, SecretsPayload, DEFAULT_SECRETS_PATH,
};
use secrecy::ExposeSecret;
use std::{env, fs, path::PathBuf};

struct Args {
    output: PathBuf,
    force: bool,
}

fn main() -> Result<()> {
    let args = parse_args()?;
    if args.output.exists() && !args.force {
        return Err(BankaiError::InvalidArgument(format!(
            "refusing to overwrite {}; use --force to continue",
            args.output.display()
        )));
    }

    let payload = SecretsPayload {
        polygon_private_key: prompt_optional_value("Polygon private key (hex, no 0x): ")?,
        polymarket_api_key: prompt_optional_value("Polymarket API key: ")?,
        polymarket_api_secret: prompt_optional_value("Polymarket API secret: ")?,
        polymarket_api_passphrase: prompt_optional_value("Polymarket API passphrase: ")?,
        allora_api_key: prompt_optional_value("Allora API key: ")?,
    };

    let password = prompt_new_password()?;
    encrypt_to_file(&args.output, &password, &payload)?;

    println!("Encrypted secrets written to {}", args.output.display());
    Ok(())
}

fn parse_args() -> Result<Args> {
    let mut output = PathBuf::from(DEFAULT_SECRETS_PATH);
    let mut force = false;
    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" | "-o" => {
                let value = args.next().ok_or_else(|| {
                    BankaiError::InvalidArgument("missing value for --output".to_string())
                })?;
                output = PathBuf::from(value);
            }
            "--force" | "-f" => {
                force = true;
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                return Err(BankaiError::InvalidArgument(format!(
                    "unknown argument: {arg}"
                )));
            }
        }
    }

    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    Ok(Args { output, force })
}

fn print_usage() {
    println!("Usage: keytool [--output <path>] [--force]");
    println!("  --output, -o  Path for secrets.enc (default: config/secrets.enc)");
    println!("  --force, -f   Overwrite existing secrets.enc");
}

fn prompt_optional_value(label: &str) -> Result<Option<String>> {
    let value = prompt_password(label)?;
    let trimmed = value.expose_secret().trim().to_string();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed))
    }
}
