/**
 * @description
 * Python host for strategy evaluation via embedded PyO3.
 *
 * @dependencies
 * - pyo3: Python bindings and GIL management
 *
 * @notes
 * - Ensure `python/` is on sys.path before importing strategy modules.
 */
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};
use std::path::{Path, PathBuf};

use crate::error::{BankaiError, Result};

pub trait StrategyInterface {
    fn calculate_kelly(&self, win_prob: f64, odds: f64) -> Result<f64>;
    fn validate_signal(&self, inference_json: &str, current_price: f64) -> Result<bool>;
    fn calculate_staleness(
        &self,
        signal_timestamp_ms: i64,
        candle_end_timestamp_ms: i64,
        now_timestamp_ms: i64,
    ) -> Result<f64>;
}

pub struct PythonHost {
    module_name: String,
    module: Py<PyModule>,
}

impl PythonHost {
    pub fn initialize() {
        pyo3::prepare_freethreaded_python();
    }

    pub fn new(module_name: &str) -> Result<Self> {
        let module_dir = default_python_dir()?;
        Self::new_with_path(module_name, &module_dir)
    }

    pub fn new_with_path(module_name: &str, module_dir: &Path) -> Result<Self> {
        Python::with_gil(|py| {
            ensure_sys_path(py, module_dir)?;
            let module = py.import_bound(module_name)?;
            Ok(Self {
                module_name: module_name.to_string(),
                module: module.into(),
            })
        })
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }
}

impl StrategyInterface for PythonHost {
    fn calculate_kelly(&self, win_prob: f64, odds: f64) -> Result<f64> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let value = module.getattr("calculate_kelly")?.call1((win_prob, odds))?;
            Ok(value.extract::<f64>()?)
        })
    }

    fn validate_signal(&self, inference_json: &str, current_price: f64) -> Result<bool> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let value = module
                .getattr("validate_signal")?
                .call1((inference_json, current_price))?;
            Ok(value.extract::<bool>()?)
        })
    }

    fn calculate_staleness(
        &self,
        signal_timestamp_ms: i64,
        candle_end_timestamp_ms: i64,
        now_timestamp_ms: i64,
    ) -> Result<f64> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let value = module.getattr("calculate_staleness")?.call1((
                signal_timestamp_ms,
                candle_end_timestamp_ms,
                now_timestamp_ms,
            ))?;
            Ok(value.extract::<f64>()?)
        })
    }
}

fn default_python_dir() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    Ok(cwd.join("python"))
}

fn ensure_sys_path(py: Python<'_>, module_dir: &Path) -> Result<()> {
    let sys = py.import_bound("sys")?;
    let path_any = sys.getattr("path")?;
    let path = path_any
        .downcast::<PyList>()
        .map_err(|err| BankaiError::Python(err.into()))?;
    let module_dir_str = module_dir
        .to_str()
        .ok_or_else(|| BankaiError::InvalidArgument("python path is not utf-8".to_string()))?;
    let already_present = path
        .iter()
        .filter_map(|item| item.extract::<String>().ok())
        .any(|entry| entry == module_dir_str);
    if !already_present {
        path.insert(0, module_dir_str)?;
    }
    Ok(())
}
