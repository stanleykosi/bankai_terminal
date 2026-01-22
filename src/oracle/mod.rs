pub mod allora;
/**
 * @description
 * Oracle modules for market data ingestion.
 *
 * @dependencies
 * - None (module declarations only)
 *
 * @notes
 * - Each oracle should emit MarketUpdate events for the engine.
 */
pub mod binance;
pub mod polymarket_discovery;
pub mod polymarket_rtds;
