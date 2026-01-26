pub mod recovery;
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
pub mod keys;
pub mod trade_events;
pub mod reconcile;
pub mod redemption;
