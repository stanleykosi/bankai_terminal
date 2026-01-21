/**
 * @description
 * Kill switch and risk state tracking for latency, clock drift, and losses.
 *
 * @dependencies
 * - arc-swap: atomic config updates for kill switch thresholds
 *
 * @notes
 * - Engine components should respect `is_halted()` and stop trading immediately.
 */
use arc_swap::ArcSwap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU32, AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;

use crate::config::TradingConfig;
use crate::error::{BankaiError, Result};

#[derive(Debug, Clone)]
pub struct KillSwitchConfig {
    pub latency_ms: u64,
    pub clock_drift_ms: i64,
    pub consecutive_losses: u32,
}

impl KillSwitchConfig {
    pub fn from_trading(trading: &TradingConfig) -> Self {
        Self {
            latency_ms: trading.kill_switch_latency_ms,
            clock_drift_ms: trading.kill_switch_clock_drift_ms as i64,
            consecutive_losses: trading.kill_switch_consecutive_losses,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HaltReason {
    None = 0,
    Latency = 1,
    ClockDrift = 2,
    ConsecutiveLosses = 3,
    Manual = 4,
}

impl HaltReason {
    fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Latency,
            2 => Self::ClockDrift,
            3 => Self::ConsecutiveLosses,
            4 => Self::Manual,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RiskSnapshot {
    pub halted: bool,
    pub reason: HaltReason,
    pub last_latency_ms: u64,
    pub clock_drift_ms: i64,
    pub consecutive_losses: u32,
}

pub struct RiskState {
    config: ArcSwap<KillSwitchConfig>,
    halted: AtomicBool,
    halt_reason: AtomicU8,
    last_latency_ms: AtomicU64,
    clock_drift_ms: AtomicI64,
    consecutive_losses: AtomicU32,
}

impl RiskState {
    pub fn new(config: KillSwitchConfig) -> Self {
        Self {
            config: ArcSwap::from_pointee(config),
            halted: AtomicBool::new(false),
            halt_reason: AtomicU8::new(HaltReason::None as u8),
            last_latency_ms: AtomicU64::new(0),
            clock_drift_ms: AtomicI64::new(0),
            consecutive_losses: AtomicU32::new(0),
        }
    }

    pub fn update_config(&self, config: KillSwitchConfig) {
        self.config.store(Arc::new(config));
    }

    pub fn record_latency_ms(&self, latency_ms: u64) -> bool {
        self.last_latency_ms.store(latency_ms, Ordering::Relaxed);
        self.evaluate()
    }

    pub fn record_clock_drift_ms(&self, drift_ms: i64) -> bool {
        self.clock_drift_ms.store(drift_ms, Ordering::Relaxed);
        self.evaluate()
    }

    pub fn record_loss(&self) -> bool {
        let losses = self.consecutive_losses.fetch_add(1, Ordering::Relaxed) + 1;
        let config = self.config.load();
        if losses > config.consecutive_losses {
            self.trigger_halt(HaltReason::ConsecutiveLosses);
        }
        self.is_halted()
    }

    pub fn record_win(&self) {
        self.consecutive_losses.store(0, Ordering::Relaxed);
    }

    pub fn manual_halt(&self) {
        self.trigger_halt(HaltReason::Manual);
    }

    pub fn clear_halt(&self) {
        self.halted.store(false, Ordering::SeqCst);
        self.halt_reason
            .store(HaltReason::None as u8, Ordering::SeqCst);
    }

    pub fn is_halted(&self) -> bool {
        self.halted.load(Ordering::Relaxed)
    }

    pub fn halt_reason(&self) -> HaltReason {
        HaltReason::from_u8(self.halt_reason.load(Ordering::Relaxed))
    }

    pub fn snapshot(&self) -> RiskSnapshot {
        RiskSnapshot {
            halted: self.is_halted(),
            reason: self.halt_reason(),
            last_latency_ms: self.last_latency_ms.load(Ordering::Relaxed),
            clock_drift_ms: self.clock_drift_ms.load(Ordering::Relaxed),
            consecutive_losses: self.consecutive_losses.load(Ordering::Relaxed),
        }
    }

    fn evaluate(&self) -> bool {
        let config = self.config.load();
        let latency_ms = self.last_latency_ms.load(Ordering::Relaxed);
        if latency_ms > config.latency_ms {
            self.trigger_halt(HaltReason::Latency);
            return true;
        }

        let drift_ms = self.clock_drift_ms.load(Ordering::Relaxed).abs();
        if drift_ms > config.clock_drift_ms {
            self.trigger_halt(HaltReason::ClockDrift);
            return true;
        }

        let losses = self.consecutive_losses.load(Ordering::Relaxed);
        if losses > config.consecutive_losses {
            self.trigger_halt(HaltReason::ConsecutiveLosses);
            return true;
        }

        self.is_halted()
    }

    fn trigger_halt(&self, reason: HaltReason) {
        if self
            .halted
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            self.halt_reason.store(reason as u8, Ordering::SeqCst);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StalenessCheck {
    pub ratio: f64,
    pub is_stale: bool,
}

pub fn calculate_staleness_ratio(
    now_ms: u64,
    signal_timestamp_ms: u64,
    candle_end_timestamp_ms: u64,
) -> Result<f64> {
    if candle_end_timestamp_ms <= now_ms {
        return Err(BankaiError::InvalidArgument(
            "candle_end_timestamp_ms must be in the future".to_string(),
        ));
    }
    let elapsed = now_ms.saturating_sub(signal_timestamp_ms);
    let remaining = candle_end_timestamp_ms - now_ms;
    if remaining == 0 {
        return Ok(1.0);
    }
    Ok(elapsed as f64 / remaining as f64)
}

pub fn evaluate_staleness(
    now_ms: u64,
    signal_timestamp_ms: u64,
    candle_end_timestamp_ms: u64,
    max_ratio: f64,
) -> Result<StalenessCheck> {
    let ratio = calculate_staleness_ratio(now_ms, signal_timestamp_ms, candle_end_timestamp_ms)?;
    Ok(StalenessCheck {
        ratio,
        is_stale: ratio > max_ratio,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> KillSwitchConfig {
        KillSwitchConfig {
            latency_ms: 100,
            clock_drift_ms: 50,
            consecutive_losses: 2,
        }
    }

    #[test]
    fn it_triggers_halt_on_latency_and_records_reason() {
        let state = RiskState::new(test_config());

        assert!(!state.is_halted());
        assert!(!state.record_latency_ms(80));
        assert!(!state.is_halted());

        assert!(state.record_latency_ms(150));
        assert!(state.is_halted());
        assert_eq!(state.halt_reason(), HaltReason::Latency);
    }

    #[test]
    fn it_triggers_halt_on_clock_drift_and_resets() {
        let state = RiskState::new(test_config());

        assert!(state.record_clock_drift_ms(75));
        assert!(state.is_halted());
        assert_eq!(state.halt_reason(), HaltReason::ClockDrift);

        state.clear_halt();
        assert!(!state.is_halted());
        assert_eq!(state.halt_reason(), HaltReason::None);
    }

    #[test]
    fn it_triggers_halt_on_consecutive_losses() {
        let state = RiskState::new(test_config());

        assert!(!state.record_loss());
        assert!(!state.record_loss());
        assert!(state.record_loss());
        assert!(state.is_halted());
        assert_eq!(state.halt_reason(), HaltReason::ConsecutiveLosses);
    }

    #[test]
    fn it_calculates_staleness_ratio_and_flags_stale() {
        let result = evaluate_staleness(1_000, 800, 1_400, 0.4).expect("staleness computed");
        assert!(result.ratio > 0.0);
        assert!(result.is_stale);
    }

    #[test]
    fn it_errors_when_candle_end_not_in_future() {
        let err = evaluate_staleness(1_500, 1_000, 1_500, 0.1).unwrap_err();
        match err {
            BankaiError::InvalidArgument(msg) => {
                assert!(msg.contains("candle_end_timestamp_ms must be in the future"))
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
