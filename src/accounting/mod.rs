pub mod bankroll_refresh;
pub mod keys;
pub mod no_money;
pub mod open_orders_refresh;
/**
 * @purpose
 * Accounting modules for redemption and recovery workflows.
 *
 * @dependencies
 * - None (module declarations only)
 *
 * @notes
 * - Redemption handles post-resolution capital recycling.
 * - Recovery rehydrates balances and open orders on startup.
 */
pub mod pnl;
pub mod reconcile;
pub mod recovery;
pub mod redemption;
pub mod trade_events;
pub mod utils;
