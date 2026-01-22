/**
 * @description
 * Storage layer modules for Redis hot state and TimescaleDB persistence.
 *
 * @dependencies
 * - None (module declarations only)
 *
 * @notes
 * - Keep storage APIs async and non-blocking.
 */
pub mod database;
pub mod orderbook;
pub mod redis;
