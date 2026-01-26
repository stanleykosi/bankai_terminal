/**
 * @purpose
 * Bankai Terminal entry point that bootstraps the async runtime and logging.
 *
 * @dependencies
 * - tokio: async runtime
 * - tracing: structured logging macros
 * - tracing-subscriber: log formatting and filtering
 *
 * @notes
 * - Logging defaults to info unless RUST_LOG is set.
 * - Startup recovery reconciles balances and open orders before trading.
 */
use bankai_terminal::accounting::recovery::StartupRecovery;
use bankai_terminal::config::{Config, ConfigManager};
use bankai_terminal::engine::core::EngineCore;
use bankai_terminal::engine::risk::{KillSwitchConfig, RiskState};
use bankai_terminal::engine::types::MarketUpdate;
use bankai_terminal::error::Result;
use bankai_terminal::oracle::allora::{AlloraConsumerTopic, AlloraOracle, AlloraOracleConfig};
use bankai_terminal::oracle::binance::{BinanceOracle, BinanceOracleConfig};
use bankai_terminal::oracle::polymarket_discovery::{
    PolymarketDiscovery, PolymarketDiscoveryConfig,
};
use bankai_terminal::oracle::polymarket_rtds::{PolymarketRtds, PolymarketRtdsConfig};
use bankai_terminal::security::{self, DEFAULT_SECRETS_PATH};
use bankai_terminal::storage::orderbook::OrderBookStore;
use bankai_terminal::storage::redis::RedisManager;
use bankai_terminal::telemetry::health::HealthMonitor;
use bankai_terminal::telemetry::{logging, metrics, preflight};
use bankai_terminal::ui;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_tracing();
    metrics::init_metrics();
    tracing::info!("bankai terminal booting");
    let mut preflight_ok = true;

    let config_manager = ConfigManager::new("config/config.json")?;
    let _watcher = config_manager.spawn_watcher()?;
    let config_state = config_manager.state();
    let config = config_manager.current();

    tracing::info!(?config, "config loaded");
    let secrets = security::load_secrets_interactive(DEFAULT_SECRETS_PATH)?;
    tracing::info!("secrets loaded");

    if config.preflight.enabled {
        let report = preflight::run(&config).await?;
        preflight::log_report(&report);
        if config.preflight.fail_fast && report.has_failures() {
            preflight_ok = false;
            tracing::warn!(
                "[FAIL] preflight checks failed; trading halted but TUI will start for inspection"
            );
        } else if report.has_failures() {
            preflight_ok = false;
            tracing::warn!("[WARN] preflight checks failed; trading halted but continuing to UI");
        } else {
            tracing::info!("[OK] preflight checks passed");
        }
    } else {
        tracing::info!("startup preflight checks disabled");
    }

    let mut bankroll_ready = false;
    if let Ok(redis_url) = std::env::var("REDIS_URL") {
        match RedisManager::new(&redis_url).await {
            Ok(redis) => match StartupRecovery::from_env(&config, &secrets, redis) {
                Ok(Some(recovery)) => match recovery.run().await {
                    Ok(report) => {
                        bankroll_ready = report.collateral_balance > 0.0;
                        tracing::info!(
                            bankroll_ready,
                            balance_usdc = report.collateral_balance,
                            "startup recovery completed"
                        );
                    }
                    Err(error) => {
                        tracing::warn!(?error, "startup recovery failed");
                    }
                },
                Ok(None) => {
                    tracing::info!("startup recovery skipped");
                }
                Err(error) => {
                    tracing::warn!(?error, "startup recovery initialization failed");
                }
            },
            Err(error) => {
                tracing::warn!(?error, "redis unavailable; skipping startup recovery");
            }
        }
    } else {
        tracing::warn!("REDIS_URL not set; startup recovery skipped");
    }

    let risk = Arc::new(RiskState::new(KillSwitchConfig::from_trading(
        &config.trading,
    )));
    if !preflight_ok {
        risk.manual_halt();
    }
    if !bankroll_ready {
        tracing::warn!(
            "bankroll missing or zero at startup; trading halted until funds are detected"
        );
        risk.manual_halt();
    }
    let _health = HealthMonitor::from_config(risk.clone(), &config.health)?.spawn();

    let (market_tx, _) = broadcast::channel(1024);
    let tui_handle = spawn_tui_if_enabled(config_state.clone(), risk.clone(), market_tx.clone())
        .await
        .ok()
        .flatten();
    let engine = EngineCore::new(config_state, risk.clone());
    let _engine_handle = engine.spawn(market_tx.subscribe());

    spawn_binance_oracle(&config, market_tx.clone()).await?;
    spawn_allora_oracle(&config, market_tx.clone())?;
    spawn_polymarket_oracles(&config).await?;

    tracing::info!("engine running");
    tokio::signal::ctrl_c().await?;
    tracing::info!("shutdown signal received");
    if let Some(handle) = tui_handle {
        handle.shutdown();
    }
    Ok(())
}

async fn spawn_binance_oracle(
    config: &Arc<Config>,
    sender: broadcast::Sender<MarketUpdate>,
) -> Result<()> {
    let symbols = derive_binance_symbols(config);
    let redis = match std::env::var("REDIS_URL") {
        Ok(url) => match RedisManager::new(&url).await {
            Ok(manager) => Some(manager),
            Err(error) => {
                tracing::warn!(?error, "redis unavailable; binance window alignment disabled");
                None
            }
        },
        Err(_) => None,
    };
    let binance_config = BinanceOracleConfig {
        endpoint: config.endpoints.binance_ws.clone(),
        symbols,
        candle_interval: Duration::from_secs(60),
        window_refresh_interval: Duration::from_secs(5),
        redis,
    };
    let oracle = BinanceOracle::new(binance_config);
    let _handle = oracle.spawn(sender);
    Ok(())
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
            topic_id: topic.topic_id,
        })
        .collect();
    let oracle_config = AlloraOracleConfig {
        base_url: allora.base_url.clone(),
        chain: allora.chain.clone(),
        topics,
        poll_interval: Duration::from_secs(allora.poll_interval_secs),
        timeout: Duration::from_millis(allora.timeout_ms),
        api_key: allora
            .api_key
            .clone()
            .or_else(|| std::env::var("ALLORA_API_KEY").ok()),
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
    let discovery_config =
        PolymarketDiscoveryConfig::new(config.endpoints.polymarket_gamma.clone());
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

async fn spawn_tui_if_enabled(
    config: Arc<arc_swap::ArcSwap<Config>>,
    risk: Arc<RiskState>,
    sender: broadcast::Sender<MarketUpdate>,
) -> Result<Option<ui::TuiHandle>> {
    if !ui::is_tui_enabled() {
        return Ok(None);
    }

    let redis = match std::env::var("REDIS_URL") {
        Ok(url) => match RedisManager::new(&url).await {
            Ok(manager) => Some(manager),
            Err(error) => {
                tracing::warn!(?error, "redis unavailable; ui bankroll disabled");
                None
            }
        },
        Err(_) => None,
    };

    match ui::spawn_tui(config, risk, sender.subscribe(), redis) {
        Ok(handle) => Ok(Some(handle)),
        Err(error) => {
            tracing::warn!(?error, "failed to start tui");
            Ok(None)
        }
    }
}
