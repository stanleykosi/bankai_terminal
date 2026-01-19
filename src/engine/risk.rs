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
