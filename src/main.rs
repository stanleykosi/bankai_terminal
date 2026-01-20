/**
 * @description
 * Bankai Terminal entry point that bootstraps the async runtime and logging.
 *
 * @dependencies
 * - tokio: async runtime
 * - tracing: structured logging macros
 * - tracing-subscriber: log formatting and filtering
 *
 * @notes
 * - Logging defaults to info unless RUST_LOG is set.
 */
use bankai_terminal::config::{Config, ConfigManager};
use bankai_terminal::engine::core::EngineCore;
use bankai_terminal::engine::risk::{KillSwitchConfig, RiskState};
use bankai_terminal::engine::types::MarketUpdate;
use bankai_terminal::oracle::allora::{AlloraConsumerTopic, AlloraOracle, AlloraOracleConfig};
use bankai_terminal::oracle::binance::{BinanceOracle, BinanceOracleConfig};
use bankai_terminal::oracle::polymarket_discovery::{
    PolymarketDiscovery, PolymarketDiscoveryConfig,
};
use bankai_terminal::oracle::polymarket_rtds::{PolymarketRtds, PolymarketRtdsConfig};
use bankai_terminal::error::Result;
use bankai_terminal::security::{self, DEFAULT_SECRETS_PATH};
use bankai_terminal::storage::orderbook::OrderBookStore;
use bankai_terminal::storage::redis::RedisManager;
use bankai_terminal::telemetry::health::HealthMonitor;
use bankai_terminal::telemetry::{logging, metrics};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_tracing();
    metrics::init_metrics();
    tracing::info!("bankai terminal booting");

    let config_manager = ConfigManager::new("config/config.json")?;
    let _watcher = config_manager.spawn_watcher()?;
    let config = config_manager.current();

    tracing::info!(?config, "config loaded");
    let _secrets = security::load_secrets_interactive(DEFAULT_SECRETS_PATH)?;
    tracing::info!("secrets loaded");

    let risk = Arc::new(RiskState::new(KillSwitchConfig::from_trading(
        &config.trading,
    )));
    let _health = HealthMonitor::from_config(risk.clone(), &config.health)?.spawn();

    let (market_tx, _) = broadcast::channel(1024);
    let engine = EngineCore::new(config.clone(), risk.clone());
    let _engine_handle = engine.spawn(market_tx.subscribe());

    spawn_binance_oracle(&config, market_tx.clone());
    spawn_allora_oracle(&config, market_tx.clone())?;
    spawn_polymarket_oracles(&config).await?;

    tracing::info!("engine running");
    tokio::signal::ctrl_c().await?;
    tracing::info!("shutdown signal received");
    Ok(())
}

fn spawn_binance_oracle(config: &Arc<Config>, sender: broadcast::Sender<MarketUpdate>) {
    let symbols = derive_binance_symbols(config);
    let binance_config = BinanceOracleConfig {
        endpoint: config.endpoints.binance_ws.clone(),
        symbols,
        candle_interval: Duration::from_secs(60),
    };
    let oracle = BinanceOracle::new(binance_config);
    let _handle = oracle.spawn(sender);
}

fn spawn_allora_oracle(
    config: &Arc<Config>,
    sender: broadcast::Sender<MarketUpdate>,
) -> Result<()> {
    let Some(allora) = config.allora_consumer.as_ref() else {
        tracing::warn!("allora consumer config missing; allora oracle disabled");
        return Ok(());
    };

    let topics = allora
        .topics
        .iter()
        .map(|topic| AlloraConsumerTopic {
            asset: topic.asset.clone(),
            timeframe: topic.timeframe.clone(),
        })
        .collect();
    let oracle_config = AlloraOracleConfig {
        base_url: allora.base_url.clone(),
        chain: allora.chain.clone(),
        topics,
        poll_interval: Duration::from_secs(allora.poll_interval_secs),
        timeout: Duration::from_millis(allora.timeout_ms),
        api_key: None,
    };
    let oracle = AlloraOracle::new(oracle_config)?;
    let _handle = oracle.spawn(sender);
    Ok(())
}

async fn spawn_polymarket_oracles(config: &Arc<Config>) -> Result<()> {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; polymarket oracles disabled");
            return Ok(());
        }
    };

    let redis = RedisManager::new(&redis_url).await?;
    let discovery_config = PolymarketDiscoveryConfig::new(config.endpoints.polymarket_gamma.clone());
    let discovery = PolymarketDiscovery::new(discovery_config, redis.clone())?;
    let _discovery_handle = discovery.spawn();

    let mut rtds_config = PolymarketRtdsConfig::new(
        config.endpoints.polymarket_ws.clone(),
        config.endpoints.relayer_http.clone(),
        config.polymarket.asset_ids.clone(),
    );
    rtds_config.ping_interval = Duration::from_secs(config.polymarket.ping_interval_secs);
    rtds_config.asset_refresh_interval =
        Duration::from_secs(config.polymarket.asset_refresh_interval_secs);
    rtds_config.reconnect_delay = Duration::from_secs(config.polymarket.reconnect_delay_secs);
    rtds_config.snapshot_timeout = Duration::from_millis(config.polymarket.snapshot_timeout_ms);

    let orderbook = OrderBookStore::new(redis);
    let rtds = PolymarketRtds::new(rtds_config, orderbook)?;
    let _rtds_handle = rtds.spawn();
    Ok(())
}

fn derive_binance_symbols(config: &Arc<Config>) -> Vec<String> {
    let mut symbols = Vec::new();
    let mut seen = HashSet::new();

    if let Some(allora) = config.allora_consumer.as_ref() {
        for topic in &allora.topics {
            if let Some(symbol) = asset_to_binance_symbol(&topic.asset) {
                if seen.insert(symbol.clone()) {
                    symbols.push(symbol);
                }
            }
        }
    }

    if symbols.is_empty() {
        let defaults = ["btcusdt", "ethusdt", "solusdt", "xrpusdt"];
        for symbol in defaults {
            symbols.push(symbol.to_string());
        }
    }

    symbols
}

fn asset_to_binance_symbol(asset: &str) -> Option<String> {
    let trimmed = asset.trim().to_ascii_lowercase();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.ends_with("usdt") {
        return Some(trimmed);
    }
    Some(format!("{trimmed}usdt"))
}
