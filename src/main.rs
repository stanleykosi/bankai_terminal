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
use bankai_terminal::config::ConfigManager;
use bankai_terminal::error::Result;
use bankai_terminal::security::{self, DEFAULT_SECRETS_PATH};
use tracing_subscriber::{fmt, EnvFilter};

fn init_logging() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_level(true)
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();
    tracing::info!("bankai terminal booting");

    let config_manager = ConfigManager::new("config/config.json")?;
    let _watcher = config_manager.spawn_watcher()?;
    let config = config_manager.current();

    tracing::info!(?config, "config loaded");
    let _secrets = security::load_secrets_interactive(DEFAULT_SECRETS_PATH)?;
    tracing::info!("secrets loaded");
    Ok(())
}
