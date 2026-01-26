/**
 * @purpose
 * Shared Redis keys for PnL and trade tracking.
 *
 * @notes
 * - Keep constants centralized to avoid drift across modules.
 */
pub const REALIZED_PNL_KEY: &str = "sys:pnl:realized";
pub const REALIZED_PNL_24H_KEY: &str = "sys:pnl:realized_24h";
pub const UNREALIZED_PNL_KEY: &str = "sys:pnl:unrealized";
pub const PNL_24H_KEY: &str = "sys:pnl:24h";

pub const REALIZED_EVENTS_PREFIX: &str = "pnl:realized:events:";
pub const SEEN_TRADES_PREFIX: &str = "trades:seen:";
