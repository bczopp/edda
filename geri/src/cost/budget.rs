use std::sync::Arc;
use tokio::sync::RwLock;

/// Alert bei Budget-Überschreitung oder nahe am Limit.
#[derive(Debug, Clone, PartialEq)]
pub enum BudgetAlert {
    /// Budget überschritten (aktueller Usage).
    OverLimit(f64),
    /// Nahe am Limit (Usage, Limit); z. B. ≥ 80 %.
    NearLimit(f64, f64),
}

/// Verwaltet Budget-Limit und -Usage, erkennt Limits und erzeugt Alerts.
#[derive(Debug, Clone)]
pub struct BudgetManager {
    limit: f64,
    usage: f64,
}

const NEAR_LIMIT_RATIO: f64 = 0.8;

impl BudgetManager {
    pub fn new(limit: f64) -> Self {
        Self { limit, usage: 0.0 }
    }

    pub fn get_usage(&self) -> f64 { self.usage }
    pub fn get_limit(&self) -> f64 { self.limit }

    pub fn add_usage(&mut self, amount: f64) {
        self.usage += amount;
    }

    pub fn remaining(&self) -> f64 {
        (self.limit - self.usage).max(0.0)
    }

    pub fn is_over_limit(&self) -> bool {
        self.usage > self.limit
    }

    pub fn check_alerts(&self) -> Vec<BudgetAlert> {
        let mut out = Vec::new();
        if self.usage > self.limit {
            out.push(BudgetAlert::OverLimit(self.usage));
        }
        if self.limit > 0.0 && self.usage >= self.limit * NEAR_LIMIT_RATIO && self.usage <= self.limit {
            out.push(BudgetAlert::NearLimit(self.usage, self.limit));
        }
        out
    }
}

pub struct BudgetTracker {
    manager: RwLock<BudgetManager>,
}

impl BudgetTracker {
    pub fn new(limit: f64) -> Self {
        Self {
            manager: RwLock::new(BudgetManager::new(limit)),
        }
    }

    pub async fn add_usage(&self, amount: f64) {
        let mut mgr = self.manager.write().await;
        mgr.add_usage(amount);
    }

    pub async fn is_over_limit(&self) -> bool {
        let mgr = self.manager.read().await;
        mgr.is_over_limit()
    }

    pub async fn get_usage_info(&self) -> (f64, f64, bool) {
        let mgr = self.manager.read().await;
        (mgr.get_usage(), mgr.get_limit(), mgr.is_over_limit())
    }
}
