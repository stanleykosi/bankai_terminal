use ethers_core::types::U256;

use crate::error::{BankaiError, Result};

pub fn scale_u256(value: U256, decimals: u32) -> Result<f64> {
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

    scaled
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("failed to parse scaled balance".to_string()))
}
