/**
 * @description
 * Build script to configure PyO3 for embedding Python.
 *
 * @dependencies
 * - pyo3-build-config: probes Python and emits linker flags
 *
 * @notes
 * - Respects PYO3_PYTHON / PYO3_CONFIG_FILE environment variables.
 */
fn main() {
    pyo3_build_config::use_pyo3_cfgs();
    println!("cargo:rerun-if-env-changed=PYO3_PYTHON");
    println!("cargo:rerun-if-env-changed=PYO3_CONFIG_FILE");
}
