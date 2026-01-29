#![allow(
    clippy::large_enum_variant,
    clippy::manual_ignore_case_cmp,
    clippy::new_without_default,
    clippy::result_large_err,
    clippy::single_match,
    clippy::too_many_arguments,
    clippy::uninlined_format_args,
    clippy::vec_init_then_push
)]

use arc_swap::ArcSwap;
use bankai_terminal::accounting::bankroll_refresh::BankrollRefresher;
use bankai_terminal::accounting::no_money::spawn_no_money_tracker;
use bankai_terminal::accounting::open_orders_refresh::OpenOrdersRefresher;
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
use bankai_terminal::accounting::pnl::PnlMonitor;
use bankai_terminal::accounting::reconcile::TradeReconciler;
use bankai_terminal::accounting::recovery::StartupRecovery;
use bankai_terminal::accounting::redemption::{
    RedemptionClient, RedemptionConfig, RedemptionListener, RedisPositionResolver,
};
use bankai_terminal::config::{Config, ConfigManager};
use bankai_terminal::engine::core::EngineCore;
use bankai_terminal::engine::risk::{KillSwitchConfig, RiskState};
use bankai_terminal::engine::trader::TradingEngine;
use bankai_terminal::engine::types::MarketUpdate;
use bankai_terminal::error::Result;
use bankai_terminal::execution::allowances::AllowanceManager;
use bankai_terminal::execution::orchestrator::{
    ExecutionOrchestrator, ExecutionOrchestratorConfig,
};
use bankai_terminal::execution::payload_builder::PolymarketPayloadBuilder;
use bankai_terminal::execution::relayer::{RelayerClient, RelayerConfig};
use bankai_terminal::execution::signer::Eip712Signer;
use bankai_terminal::oracle::allora::{AlloraConsumerTopic, AlloraOracle, AlloraOracleConfig};
use bankai_terminal::oracle::chainlink::{ChainlinkOracle, ChainlinkOracleConfig};
use bankai_terminal::oracle::polymarket_discovery::{
    PolymarketDiscovery, PolymarketDiscoveryConfig,
};
use bankai_terminal::oracle::polymarket_rtds::{PolymarketRtds, PolymarketRtdsConfig};
use bankai_terminal::oracle::polymarket_user_ws::{
    PolymarketUserAuth, PolymarketUserWs, PolymarketUserWsConfig,
};
use bankai_terminal::security::{self, DEFAULT_SECRETS_PATH};
use bankai_terminal::storage::orderbook::OrderBookStore;
use bankai_terminal::storage::redis::RedisManager;
use bankai_terminal::telemetry::health::HealthMonitor;
use bankai_terminal::telemetry::{logging, metrics, preflight};
use bankai_terminal::ui;
use secrecy::ExposeSecret;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, mpsc};

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
    if !bankroll_ready && !config.execution.no_money_mode {
        tracing::warn!(
            "bankroll missing or zero at startup; trading halted until funds are detected"
        );
        risk.manual_halt();
    }
    let _health = HealthMonitor::from_config(risk.clone(), &config.health)?.spawn();

    let (market_tx, _) = broadcast::channel(1024);
    let chain_id = read_env_u64("POLYGON_CHAIN_ID").unwrap_or(137);
    let wallet_key_for_ui = Eip712Signer::from_secrets(&secrets, chain_id)
        .map(|signer| format!("{}", signer.address()).to_ascii_lowercase())
        .ok();
    let tui_handle = spawn_tui_if_enabled(
        config_state.clone(),
        risk.clone(),
        market_tx.clone(),
        wallet_key_for_ui,
    )
    .await
    .ok()
    .flatten();
    let engine = EngineCore::new(config_state.clone(), risk.clone());
    let _engine_handle = engine.spawn(market_tx.subscribe());

    spawn_chainlink_oracle(&config, market_tx.clone()).await?;
    spawn_allora_oracle(&config, market_tx.clone())?;
    spawn_polymarket_oracles(&config).await?;
    let user_ws_enabled = spawn_polymarket_user_ws(&config, &secrets).await?;
    spawn_execution_pipeline(
        &config,
        config_state.clone(),
        risk.clone(),
        &secrets,
        market_tx,
        user_ws_enabled,
    )
    .await?;
    spawn_allowance_manager(&config, &secrets).await?;
    spawn_bankroll_refresher(&config, &secrets).await?;
    spawn_trade_reconciler(&config, &secrets, user_ws_enabled).await?;
    spawn_open_orders_refresher(&config, &secrets).await?;
    spawn_pnl_monitor(&config, &secrets).await?;
    spawn_redemption_listener(&config, &secrets).await?;
    spawn_no_money(&config, config_state.clone()).await?;

    tracing::info!("engine running");
    tokio::signal::ctrl_c().await?;
    tracing::info!("shutdown signal received");
    if let Some(handle) = tui_handle {
        handle.shutdown();
    }
    Ok(())
}

async fn spawn_chainlink_oracle(
    config: &Arc<Config>,
    sender: broadcast::Sender<MarketUpdate>,
) -> Result<()> {
    let symbols = derive_chainlink_symbols(config);
    let redis = match std::env::var("REDIS_URL") {
        Ok(url) => match RedisManager::new(&url).await {
            Ok(manager) => Some(manager),
            Err(error) => {
                tracing::warn!(
                    ?error,
                    "redis unavailable; chainlink window alignment disabled"
                );
                None
            }
        },
        Err(_) => None,
    };
    let chainlink_config = ChainlinkOracleConfig {
        endpoint: config.endpoints.chainlink_ws.clone(),
        symbols,
        candle_interval: Duration::from_secs(60),
        window_refresh_interval: Duration::from_secs(5),
        redis,
    };
    let oracle = ChainlinkOracle::new(chainlink_config);
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
    let discovery_config = PolymarketDiscoveryConfig::new(
        config.endpoints.polymarket_gamma.clone(),
        config.endpoints.relayer_http.clone(),
    );
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

async fn spawn_polymarket_user_ws(
    config: &Arc<Config>,
    secrets: &security::Secrets,
) -> Result<bool> {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; polymarket user ws disabled");
            return Ok(false);
        }
    };
    let api_key = match secrets.polymarket_api_key.as_ref() {
        Some(value) => value.expose_secret().to_string(),
        None => {
            tracing::warn!("polymarket api key missing; user ws disabled");
            return Ok(false);
        }
    };
    let api_secret = match secrets.polymarket_api_secret.as_ref() {
        Some(value) => value.expose_secret().to_string(),
        None => {
            tracing::warn!("polymarket api secret missing; user ws disabled");
            return Ok(false);
        }
    };
    let api_passphrase = match secrets.polymarket_api_passphrase.as_ref() {
        Some(value) => value.expose_secret().to_string(),
        None => {
            tracing::warn!("polymarket api passphrase missing; user ws disabled");
            return Ok(false);
        }
    };

    let chain_id = read_env_u64("POLYGON_CHAIN_ID").unwrap_or(137);
    let wallet_key = match Eip712Signer::from_secrets(secrets, chain_id) {
        Ok(signer) => format!("{}", signer.address()).to_ascii_lowercase(),
        Err(error) => {
            tracing::warn!(?error, "failed to derive wallet address; user ws disabled");
            return Ok(false);
        }
    };

    let redis = RedisManager::new(&redis_url).await?;
    let ws_endpoint = config
        .endpoints
        .polymarket_user_ws
        .clone()
        .or_else(|| derive_user_ws(&config.endpoints.polymarket_ws))
        .ok_or_else(|| {
            bankai_terminal::error::BankaiError::InvalidArgument(
                "polymarket user ws endpoint missing".to_string(),
            )
        })?;
    let user_ws = PolymarketUserWs::new(
        PolymarketUserWsConfig {
            ws_endpoint,
            ping_interval: Duration::from_secs(10),
            reconnect_delay: Duration::from_secs(3),
            auth: PolymarketUserAuth {
                api_key,
                api_secret,
                api_passphrase,
            },
            markets: Vec::new(),
        },
        redis,
        wallet_key,
    );
    let _handle = user_ws.spawn();
    Ok(true)
}

fn derive_user_ws(market_ws: &str) -> Option<String> {
    let trimmed = market_ws.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.ends_with("/ws/market") {
        return Some(trimmed.replace("/ws/market", "/ws/user"));
    }
    if trimmed.ends_with("/ws/market/") {
        return Some(trimmed.replace("/ws/market/", "/ws/user"));
    }
    None
}

fn derive_chainlink_symbols(config: &Arc<Config>) -> Vec<String> {
    let mut symbols = Vec::new();
    let mut seen = HashSet::new();

    if let Some(allora) = config.allora_consumer.as_ref() {
        for topic in &allora.topics {
            if let Some(symbol) = asset_to_chainlink_symbol(&topic.asset) {
                if seen.insert(symbol.clone()) {
                    symbols.push(symbol);
                }
            }
        }
    }

    if symbols.is_empty() {
        let defaults = ["btc/usd", "eth/usd", "sol/usd"];
        for symbol in defaults {
            symbols.push(symbol.to_string());
        }
    }

    symbols
}

fn asset_to_chainlink_symbol(asset: &str) -> Option<String> {
    match asset.trim().to_ascii_uppercase().as_str() {
        "BTC" => Some("btc/usd".to_string()),
        "ETH" => Some("eth/usd".to_string()),
        "SOL" => Some("sol/usd".to_string()),
        _ => None,
    }
}

async fn spawn_execution_pipeline(
    config: &Arc<Config>,
    config_state: Arc<ArcSwap<Config>>,
    risk: Arc<RiskState>,
    secrets: &security::Secrets,
    market_tx: broadcast::Sender<MarketUpdate>,
    user_ws_enabled: bool,
) -> Result<()> {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; execution pipeline disabled");
            return Ok(());
        }
    };
    let exchange_address = match std::env::var("POLYMARKET_EXCHANGE_ADDRESS") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("POLYMARKET_EXCHANGE_ADDRESS missing; execution pipeline disabled");
            return Ok(());
        }
    };

    let redis = RedisManager::new(&redis_url).await?;
    let orderbook = OrderBookStore::new(redis.clone());
    let (intent_tx, intent_rx) = mpsc::channel(256);

    let chain_id = read_env_u64("POLYGON_CHAIN_ID").unwrap_or(137);
    let wallet_key = match Eip712Signer::from_secrets(secrets, chain_id) {
        Ok(signer) => Some(format!("{}", signer.address()).to_ascii_lowercase()),
        Err(error) => {
            tracing::warn!(
                ?error,
                "failed to derive wallet address; position tracking disabled"
            );
            None
        }
    };

    let trading_engine = TradingEngine::new(
        config_state.clone(),
        risk,
        redis.clone(),
        orderbook.clone(),
        intent_tx,
        wallet_key.clone(),
    );
    let _trading_handle = trading_engine.spawn(market_tx.subscribe());

    let exchange_address = parse_address(&exchange_address)?;
    let builder = PolymarketPayloadBuilder::new(
        config_state,
        redis.clone(),
        orderbook,
        secrets,
        exchange_address,
        chain_id,
    )?;
    let relayer = RelayerClient::new(RelayerConfig::new(config.endpoints.relayer_http.clone()))?;

    let database = match std::env::var("DATABASE_URL") {
        Ok(url) => match bankai_terminal::storage::database::DatabaseManager::new(&url, 5).await {
            Ok(db) => Some(db),
            Err(error) => {
                tracing::warn!(
                    ?error,
                    "failed to connect to database; execution logging disabled"
                );
                None
            }
        },
        Err(_) => None,
    };

    let cancel_client = wallet_key.as_ref().and_then(|address| {
        let config = bankai_terminal::execution::cancel::CancelClientConfig::from_env(
            config.endpoints.relayer_http.clone(),
        );
        match bankai_terminal::execution::cancel::CancelClient::from_env(config, secrets, address) {
            Ok(value) => value,
            Err(error) => {
                tracing::warn!(?error, "cancel client disabled");
                None
            }
        }
    });

    let orchestrator = ExecutionOrchestrator::new(
        ExecutionOrchestratorConfig {
            prefer_ws_reconcile: config.execution.prefer_ws_reconcile && user_ws_enabled,
            max_retries: config.execution.relayer_max_retries,
            backoff_ms: config.execution.relayer_backoff_ms,
            backoff_max_ms: config.execution.relayer_backoff_max_ms,
            idempotency_ttl_secs: config.execution.idempotency_ttl_secs,
            cancel_before_replace: config.execution.cancel_before_replace,
            no_money_mode: config.execution.no_money_mode,
            ..Default::default()
        },
        relayer,
        None,
        cancel_client,
        database,
        None,
        Some(redis),
        wallet_key,
        Arc::new(builder),
    )?;
    let _exec_handle = orchestrator.spawn(intent_rx);

    Ok(())
}

async fn spawn_allowance_manager(config: &Arc<Config>, secrets: &security::Secrets) -> Result<()> {
    let redis = match std::env::var("REDIS_URL") {
        Ok(url) => match RedisManager::new(&url).await {
            Ok(manager) => Some(manager),
            Err(error) => {
                tracing::warn!(?error, "redis unavailable; allowance logging disabled");
                None
            }
        },
        Err(_) => None,
    };
    let Some(manager) = AllowanceManager::from_env(config, secrets, redis)? else {
        return Ok(());
    };
    let _handle = manager.spawn();
    Ok(())
}

async fn spawn_no_money(config: &Arc<Config>, config_state: Arc<ArcSwap<Config>>) -> Result<()> {
    if !config.execution.no_money_mode {
        return Ok(());
    }
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; no-money tracking disabled");
            return Ok(());
        }
    };
    let redis = RedisManager::new(&redis_url).await?;
    let _handle = spawn_no_money_tracker(config_state, redis);
    Ok(())
}

async fn spawn_bankroll_refresher(config: &Arc<Config>, secrets: &security::Secrets) -> Result<()> {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; bankroll refresher disabled");
            return Ok(());
        }
    };
    let redis = RedisManager::new(&redis_url).await?;
    let Some(refresher) = BankrollRefresher::from_env(config, secrets, redis)? else {
        return Ok(());
    };
    let _handle = refresher.spawn();
    Ok(())
}

async fn spawn_trade_reconciler(
    config: &Arc<Config>,
    secrets: &security::Secrets,
    user_ws_enabled: bool,
) -> Result<()> {
    if config.execution.prefer_ws_reconcile && user_ws_enabled {
        tracing::info!("user ws enabled; trade reconciler disabled");
        return Ok(());
    }
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; trade reconciler disabled");
            return Ok(());
        }
    };
    let redis = RedisManager::new(&redis_url).await?;
    let Some(reconciler) = TradeReconciler::from_env(config, secrets, redis)? else {
        return Ok(());
    };
    let _handle = reconciler.spawn();
    Ok(())
}

async fn spawn_open_orders_refresher(
    config: &Arc<Config>,
    secrets: &security::Secrets,
) -> Result<()> {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; open orders refresher disabled");
            return Ok(());
        }
    };
    let redis = RedisManager::new(&redis_url).await?;
    let Some(refresher) = OpenOrdersRefresher::from_env(config, secrets, redis)? else {
        return Ok(());
    };
    let _handle = refresher.spawn();
    Ok(())
}

async fn spawn_pnl_monitor(config: &Arc<Config>, secrets: &security::Secrets) -> Result<()> {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; pnl monitor disabled");
            return Ok(());
        }
    };
    let chain_id = read_env_u64("POLYGON_CHAIN_ID").unwrap_or(137);
    let wallet_key = match Eip712Signer::from_secrets(secrets, chain_id) {
        Ok(signer) => Some(format!("{}", signer.address()).to_ascii_lowercase()),
        Err(error) => {
            tracing::warn!(
                ?error,
                "failed to derive wallet address; pnl monitor disabled"
            );
            None
        }
    };
    let Some(wallet_key) = wallet_key else {
        return Ok(());
    };
    let redis = RedisManager::new(&redis_url).await?;
    let orderbook = OrderBookStore::new(redis.clone());
    let interval = Duration::from_secs(config.execution.trade_reconcile_interval_secs.max(3));
    let monitor = PnlMonitor::new(redis, orderbook, wallet_key, interval);
    let _handle = monitor.spawn();
    Ok(())
}

async fn spawn_redemption_listener(
    config: &Arc<Config>,
    secrets: &security::Secrets,
) -> Result<()> {
    let redis_url = match std::env::var("REDIS_URL") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("REDIS_URL not set; redemption listener disabled");
            return Ok(());
        }
    };
    let ctf_address = match std::env::var("POLYMARKET_CTF_ADDRESS") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("POLYMARKET_CTF_ADDRESS missing; redemption listener disabled");
            return Ok(());
        }
    };
    let collateral_address = match std::env::var("POLYMARKET_COLLATERAL_ADDRESS") {
        Ok(value) => value,
        Err(_) => {
            tracing::warn!("POLYMARKET_COLLATERAL_ADDRESS missing; redemption listener disabled");
            return Ok(());
        }
    };
    let chain_id = read_env_u64("POLYGON_CHAIN_ID").unwrap_or(137);

    let mut redemption_config = RedemptionConfig::new(
        config.endpoints.polygon_rpc.clone(),
        ctf_address,
        collateral_address,
        chain_id,
    );
    if let Some(decimals) = read_env_u32("POLYMARKET_COLLATERAL_DECIMALS") {
        redemption_config.collateral_decimals = decimals;
    }

    let client = match RedemptionClient::new(redemption_config, secrets) {
        Ok(client) => client,
        Err(error) => {
            tracing::warn!(?error, "failed to initialize redemption client");
            return Ok(());
        }
    };

    let wallet_key = format!("{}", client.wallet_address()).to_ascii_lowercase();
    let redis = RedisManager::new(&redis_url).await?;
    let resolver = RedisPositionResolver::new(redis.clone(), wallet_key);
    let listener = RedemptionListener::new(client, redis, resolver);
    let _handle = listener.spawn();
    Ok(())
}

fn parse_address(value: &str) -> Result<ethers_core::types::Address> {
    use std::str::FromStr;
    ethers_core::types::Address::from_str(value.trim()).map_err(|_| {
        bankai_terminal::error::BankaiError::InvalidArgument("invalid exchange address".to_string())
    })
}

fn read_env_u64(key: &str) -> Option<u64> {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
}

fn read_env_u32(key: &str) -> Option<u32> {
    std::env::var(key)
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
}

async fn spawn_tui_if_enabled(
    config: Arc<arc_swap::ArcSwap<Config>>,
    risk: Arc<RiskState>,
    sender: broadcast::Sender<MarketUpdate>,
    wallet_key: Option<String>,
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

    match ui::spawn_tui(config, risk, sender.subscribe(), redis, wallet_key) {
        Ok(handle) => Ok(Some(handle)),
        Err(error) => {
            tracing::warn!(?error, "failed to start tui");
            Ok(None)
        }
    }
}
