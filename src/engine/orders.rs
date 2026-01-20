/**
 * @description
 * Order lifecycle management for active orders, cancellations, and FOK emulation.
 *
 * @dependencies
 * - std: collections for in-memory tracking
 *
 * @notes
 * - Ladder orders cancel within the configured grace window when conditions degrade.
 * - FOK/FAK/IOC behaviors are emulated until relayer integration is added.
 */
use std::collections::HashMap;

use crate::engine::types::TradeMode;
use crate::error::{BankaiError, Result};

const DEFAULT_LADDER_CANCEL_GRACE_MS: u64 = 250;
const DEFAULT_FILL_CHECK_DELAY_MS: u64 = 25;

#[derive(Debug, Clone)]
pub struct OrderLifecycleConfig {
    pub ladder_cancel_grace_ms: u64,
    pub fill_check_delay_ms: u64,
}

impl Default for OrderLifecycleConfig {
    fn default() -> Self {
        Self {
            ladder_cancel_grace_ms: DEFAULT_LADDER_CANCEL_GRACE_MS,
            fill_check_delay_ms: DEFAULT_FILL_CHECK_DELAY_MS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Active,
    PendingCancel,
    Cancelled,
    Filled,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelReason {
    MarketDegraded,
    UnfilledImmediate,
    UnfilledFok,
    Expired,
    Manual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Gtc,
    Ioc,
    Fok,
    Fak,
    Gtd { expires_at_ms: u64 },
}

#[derive(Debug, Clone)]
pub struct CancelRequest {
    pub order_id: String,
    pub reason: CancelReason,
}

#[derive(Debug, Clone)]
pub struct FokCheck {
    pub order_id: String,
}

#[derive(Debug, Clone)]
pub enum OrderAction {
    Cancel(CancelRequest),
    CheckFill(FokCheck),
}

#[derive(Debug, Clone)]
pub struct OrderInit {
    pub order_id: String,
    pub market_id: String,
    pub asset_id: String,
    pub mode: TradeMode,
    pub requested_qty: f64,
    pub created_at_ms: u64,
    pub order_type: OrderType,
}

#[derive(Debug, Clone)]
pub struct TrackedOrder {
    pub order_id: String,
    pub market_id: String,
    pub asset_id: String,
    pub mode: TradeMode,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub requested_qty: f64,
    pub filled_qty: f64,
    pub created_at_ms: u64,
    pub last_update_ms: u64,
    degraded_at_ms: Option<u64>,
    fok_check_at_ms: Option<u64>,
    fok_checked: bool,
}

pub struct OrderLifecycleManager {
    config: OrderLifecycleConfig,
    orders: HashMap<String, TrackedOrder>,
}

impl OrderLifecycleManager {
    /// Create a new order lifecycle manager.
    pub fn new(config: OrderLifecycleConfig) -> Self {
        Self {
            config,
            orders: HashMap::new(),
        }
    }

    /// Track a new order for lifecycle management.
    pub fn track_order(&mut self, init: OrderInit) -> Result<()> {
        if init.order_id.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "order_id is required".to_string(),
            ));
        }
        if self.orders.contains_key(&init.order_id) {
            return Err(BankaiError::InvalidArgument(
                "order_id already tracked".to_string(),
            ));
        }
        if init.market_id.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "market_id is required".to_string(),
            ));
        }
        if init.asset_id.trim().is_empty() {
            return Err(BankaiError::InvalidArgument(
                "asset_id is required".to_string(),
            ));
        }
        if init.requested_qty <= 0.0 {
            return Err(BankaiError::InvalidArgument(
                "requested_qty must be positive".to_string(),
            ));
        }

        let fill_check_at_ms = match init.order_type {
            OrderType::Gtc => None,
            OrderType::Ioc => Some(init.created_at_ms),
            OrderType::Fok | OrderType::Fak => {
                Some(init.created_at_ms + self.config.fill_check_delay_ms)
            }
            OrderType::Gtd { expires_at_ms } => {
                if expires_at_ms <= init.created_at_ms {
                    return Err(BankaiError::InvalidArgument(
                        "gtd expires_at_ms must be after created_at_ms".to_string(),
                    ));
                }
                None
            }
        };

        let tracked = TrackedOrder {
            order_id: init.order_id.clone(),
            market_id: init.market_id,
            asset_id: init.asset_id,
            mode: init.mode,
            order_type: init.order_type,
            status: OrderStatus::Active,
            requested_qty: init.requested_qty,
            filled_qty: 0.0,
            created_at_ms: init.created_at_ms,
            last_update_ms: init.created_at_ms,
            degraded_at_ms: None,
            fok_check_at_ms: fill_check_at_ms,
            fok_checked: false,
        };

        self.orders.insert(init.order_id, tracked);
        Ok(())
    }

    /// Update the filled quantity for an order.
    pub fn update_fill(&mut self, order_id: &str, filled_qty: f64, event_time_ms: u64) {
        if let Some(order) = self.orders.get_mut(order_id) {
            order.filled_qty = filled_qty.max(0.0);
            order.last_update_ms = event_time_ms;
            if order.filled_qty >= order.requested_qty {
                order.status = OrderStatus::Filled;
            }
        }
    }

    /// Mark the market as degraded, triggering ladder cancellations.
    pub fn mark_market_degraded(&mut self, market_id: &str, now_ms: u64) {
        for order in self.orders.values_mut() {
            if order.market_id == market_id
                && order.mode == TradeMode::Ladder
                && order.status == OrderStatus::Active
                && order.degraded_at_ms.is_none()
            {
                order.degraded_at_ms = Some(now_ms);
            }
        }
    }

    /// Clear degraded marker when market conditions recover.
    pub fn clear_market_degraded(&mut self, market_id: &str) {
        for order in self.orders.values_mut() {
            if order.market_id == market_id
                && order.mode == TradeMode::Ladder
                && order.status == OrderStatus::Active
            {
                order.degraded_at_ms = None;
            }
        }
    }

    /// Mark that a FOK fill check has been performed.
    pub fn mark_fok_checked(&mut self, order_id: &str, now_ms: u64) {
        if let Some(order) = self.orders.get_mut(order_id) {
            order.fok_checked = true;
            order.last_update_ms = now_ms;
        }
    }

    /// Mark an order as cancelled.
    pub fn mark_cancelled(&mut self, order_id: &str, now_ms: u64) {
        if let Some(order) = self.orders.get_mut(order_id) {
            order.status = OrderStatus::Cancelled;
            order.last_update_ms = now_ms;
        }
    }

    /// Mark an order as rejected.
    pub fn mark_rejected(&mut self, order_id: &str, now_ms: u64) {
        if let Some(order) = self.orders.get_mut(order_id) {
            order.status = OrderStatus::Rejected;
            order.last_update_ms = now_ms;
        }
    }

    /// Return pending actions (cancels/checks) based on policy.
    pub fn sweep_actions(&mut self, now_ms: u64) -> Vec<OrderAction> {
        let mut actions = Vec::new();
        for order in self.orders.values_mut() {
            if order.status != OrderStatus::Active {
                continue;
            }

            if should_cancel_ladder(order, now_ms, self.config.ladder_cancel_grace_ms) {
                order.status = OrderStatus::PendingCancel;
                actions.push(OrderAction::Cancel(CancelRequest {
                    order_id: order.order_id.clone(),
                    reason: CancelReason::MarketDegraded,
                }));
                continue;
            }

            if let Some(check_at_ms) = order.fok_check_at_ms {
                if !order.fok_checked && now_ms >= check_at_ms {
                    actions.push(OrderAction::CheckFill(FokCheck {
                        order_id: order.order_id.clone(),
                    }));
                    continue;
                }
                if order.fok_checked && order.filled_qty < order.requested_qty {
                    let reason = match order.order_type {
                        OrderType::Fok => CancelReason::UnfilledFok,
                        OrderType::Fak | OrderType::Ioc => CancelReason::UnfilledImmediate,
                        OrderType::Gtc | OrderType::Gtd { .. } => continue,
                    };
                    order.status = OrderStatus::PendingCancel;
                    actions.push(OrderAction::Cancel(CancelRequest {
                        order_id: order.order_id.clone(),
                        reason,
                    }));
                }
            }

            if let OrderType::Gtd { expires_at_ms } = order.order_type {
                if now_ms >= expires_at_ms {
                    order.status = OrderStatus::Expired;
                    actions.push(OrderAction::Cancel(CancelRequest {
                        order_id: order.order_id.clone(),
                        reason: CancelReason::Expired,
                    }));
                }
            }
        }
        actions
    }

    /// Remove completed orders from memory.
    pub fn purge_completed(&mut self) {
        self.orders.retain(|_, order| {
            matches!(
                order.status,
                OrderStatus::Active | OrderStatus::PendingCancel | OrderStatus::Expired
            )
        });
    }

    /// Get a snapshot of all tracked orders.
    pub fn snapshot(&self) -> Vec<TrackedOrder> {
        self.orders.values().cloned().collect()
    }
}

fn should_cancel_ladder(order: &TrackedOrder, now_ms: u64, grace_ms: u64) -> bool {
    if order.mode != TradeMode::Ladder {
        return false;
    }
    if let Some(degraded_at_ms) = order.degraded_at_ms {
        return now_ms.saturating_sub(degraded_at_ms) >= grace_ms;
    }
    false
}
