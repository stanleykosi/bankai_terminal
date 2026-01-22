pub mod accounting;
/**
 * @description
 * Shared library exports for the Bankai Terminal binaries.
 *
 * @dependencies
 * - None (module re-exports)
 *
 * @notes
 * - Keep module exports aligned with the src/ layout.
 */
pub mod config;
pub mod engine;
pub mod error;
pub mod execution;
pub mod oracle;
pub mod security;
pub mod storage;
pub mod telemetry;
pub mod ui;
