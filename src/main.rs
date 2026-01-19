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
use bankai_terminal::telemetry::{logging, metrics};

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
    Ok(())
}
