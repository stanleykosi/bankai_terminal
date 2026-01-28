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
