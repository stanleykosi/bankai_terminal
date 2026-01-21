/**
 * @description
 * Allora oracle polling via RPC (abci_query) for combined inferences.
 *
 * @dependencies
 * - reqwest: HTTP client for REST calls
 * - serde_json: JSON parsing
 *
 * @notes
 * - Implements the consumer API flow described in docs/allora_documentation.md.
 * - Emits MarketUpdate::Allora for each topic.
 */
use reqwest::{header, Client};
use serde_json::Value;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::broadcast;

use crate::engine::types::{AlloraMarketUpdate, MarketUpdate};
use crate::error::{BankaiError, Result};

#[derive(Debug, Clone)]
pub struct AlloraOracleConfig {
    pub base_url: String,
    pub chain: String,
    pub topics: Vec<AlloraConsumerTopic>,
    pub poll_interval: Duration,
    pub timeout: Duration,
    pub api_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AlloraConsumerTopic {
    pub asset: String,
    pub timeframe: String,
    pub topic_id: Option<u64>,
}

pub struct AlloraOracle {
    config: AlloraOracleConfig,
    client: Client,
}

impl AlloraOracle {
    pub fn new(config: AlloraOracleConfig) -> Result<Self> {
        let mut client_builder = Client::builder().timeout(config.timeout);
        if let Some(api_key) = config.api_key.as_ref() {
            let mut headers = header::HeaderMap::new();
            headers.insert(
                "x-api-key",
                header::HeaderValue::from_str(api_key).map_err(|_| {
                    BankaiError::InvalidArgument("invalid allora api key".to_string())
                })?,
            );
            headers.insert(
                header::ACCEPT,
                header::HeaderValue::from_static("application/json"),
            );
            client_builder = client_builder.default_headers(headers);
        }
        let client = client_builder.build()?;
        Ok(Self { config, client })
    }

    pub fn spawn(self, sender: broadcast::Sender<MarketUpdate>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            if let Err(error) = self.run(sender).await {
                tracing::error!(?error, "allora oracle stopped");
            }
        })
    }

    async fn run(self, sender: broadcast::Sender<MarketUpdate>) -> Result<()> {
        if self.config.topics.is_empty() {
            return Err(BankaiError::InvalidArgument(
                "allora oracle requires at least one topic".to_string(),
            ));
        }

        loop {
            for topic in &self.config.topics {
                match self.fetch_topic(topic).await {
                    Ok(update) => {
                        let _ = sender.send(MarketUpdate::Allora(update));
                    }
                    Err(error) => {
                        tracing::warn!(
                            ?error,
                            asset = %topic.asset,
                            timeframe = %topic.timeframe,
                            "failed to fetch allora inference"
                        );
                    }
                }
            }
            tokio::time::sleep(self.config.poll_interval).await;
        }
    }

    async fn fetch_topic(&self, topic: &AlloraConsumerTopic) -> Result<AlloraMarketUpdate> {
        let url = build_consumer_url(&self.config.base_url, &self.config.chain, topic)?;
        let response = self.client.get(url).send().await?.error_for_status()?;
        let parsed: Value = response.json().await?;

        let status = parsed
            .get("status")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        if !status {
            return Err(BankaiError::InvalidArgument(
                "allora consumer api returned status=false".to_string(),
            ));
        }
        let data = parsed
            .get("data")
            .ok_or_else(|| BankaiError::InvalidArgument("consumer data missing".to_string()))?;
        let inference = data.get("inference_data").ok_or_else(|| {
            BankaiError::InvalidArgument("consumer inference_data missing".to_string())
        })?;

        let parsed_inference = parse_inference_value(inference, data)?;
        let topic_id = parse_topic_id(inference)?;
        let signal_timestamp_ms = parse_timestamp_ms(inference.get("timestamp"))?;
        let received_at_ms = now_ms()?;
        let signature = data
            .get("signature")
            .and_then(|value| value.as_str())
            .map(|value| value.to_string());
        let request_id = parsed
            .get("request_id")
            .and_then(|value| value.as_str())
            .map(|value| value.to_string());

        Ok(AlloraMarketUpdate {
            topic_id,
            inference_value: parsed_inference.normalized,
            inference_raw: parsed_inference.raw_value,
            token_decimals: parsed_inference.token_decimals,
            signature,
            request_id,
            confidence_intervals: Vec::new(),
            signal_timestamp_ms,
            received_at_ms,
            asset: topic.asset.clone(),
            timeframe: topic.timeframe.clone(),
        })
    }
}

fn build_consumer_url(base_url: &str, chain: &str, topic: &AlloraConsumerTopic) -> Result<String> {
    let base = base_url.trim_end_matches('/');
    let chain_path = chain.trim_matches('/');
    if base.is_empty() || chain_path.is_empty() {
        return Err(BankaiError::InvalidArgument(
            "allora base_url/chain cannot be empty".to_string(),
        ));
    }

    if let Some(topic_id) = topic.topic_id {
        return Ok(format!("{}/{}?allora_topic_id={}", base, chain_path, topic_id));
    }

    if topic.asset.trim().is_empty() || topic.timeframe.trim().is_empty() {
        return Err(BankaiError::InvalidArgument(
            "allora topic asset/timeframe cannot be empty".to_string(),
        ));
    }

    let chain_path = if chain_path.contains('/') {
        chain_path.to_string()
    } else {
        format!("price/{chain_path}")
    };
    Ok(format!(
        "{}/{}/{}/{}",
        base, chain_path, topic.asset, topic.timeframe
    ))
}

struct ParsedInference {
    normalized: f64,
    raw_value: Option<String>,
    token_decimals: Option<u32>,
}

fn parse_inference_value(inference: &Value, data: &Value) -> Result<ParsedInference> {
    let raw_value = inference
        .get("network_inference")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());
    let token_decimals = data
        .get("token_decimals")
        .and_then(|value| value.as_u64())
        .map(|value| value as u32);

    if let Some(normalized) = inference
        .get("network_inference_normalized")
        .and_then(|value| value.as_str())
    {
        let parsed = normalized.parse::<f64>().map_err(|_| {
            BankaiError::InvalidArgument("normalized inference not numeric".to_string())
        })?;
        return Ok(ParsedInference {
            normalized: parsed,
            raw_value,
            token_decimals,
        });
    }

    let raw_value = raw_value.ok_or_else(|| {
        BankaiError::InvalidArgument("network_inference missing".to_string())
    })?;
    let decimals = token_decimals.ok_or_else(|| {
        BankaiError::InvalidArgument("token_decimals missing for inference".to_string())
    })?;
    let raw = raw_value
        .parse::<f64>()
        .map_err(|_| BankaiError::InvalidArgument("network_inference not numeric".to_string()))?;
    let divisor = 10_f64.powi(decimals as i32);
    Ok(ParsedInference {
        normalized: raw / divisor,
        raw_value: Some(raw_value),
        token_decimals: Some(decimals),
    })
}

fn parse_topic_id(inference: &Value) -> Result<u64> {
    let raw = inference
        .get("topic_id")
        .ok_or_else(|| BankaiError::InvalidArgument("topic_id missing".to_string()))?;
    if let Some(value) = raw.as_u64() {
        return Ok(value);
    }
    if let Some(value) = raw.as_str() {
        return value
            .parse::<u64>()
            .map_err(|_| BankaiError::InvalidArgument("topic_id not numeric".to_string()));
    }
    Err(BankaiError::InvalidArgument(
        "topic_id invalid".to_string(),
    ))
}

fn parse_timestamp_ms(value: Option<&Value>) -> Result<u64> {
    let raw = value.ok_or_else(|| BankaiError::InvalidArgument("timestamp missing".to_string()))?;
    let parsed = if let Some(value) = raw.as_u64() {
        value
    } else if let Some(value) = raw.as_str() {
        value
            .parse::<u64>()
            .map_err(|_| BankaiError::InvalidArgument("timestamp not numeric".to_string()))?
    } else {
        return Err(BankaiError::InvalidArgument(
            "timestamp invalid".to_string(),
        ));
    };

    if parsed > 1_000_000_000_000 {
        Ok(parsed)
    } else {
        Ok(parsed * 1000)
    }
}

fn now_ms() -> Result<u64> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(now.as_millis() as u64)
}
