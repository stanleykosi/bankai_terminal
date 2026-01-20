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
pub mod nonce;
pub mod direct;
pub mod relayer;
pub mod signer;
pub mod orchestrator;
