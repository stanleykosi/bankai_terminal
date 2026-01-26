pub mod direct;
/**
 * @purpose
 * Execution rail modules for signing and nonce coordination.
 *
 * @dependencies
 * - None (module declarations only)
 *
 * @notes
 * - Keep execution helpers focused on signing and chain state.
 */
pub mod allowances;
pub mod nonce;
pub mod orchestrator;
pub mod payload_builder;
pub mod relayer;
pub mod signer;
