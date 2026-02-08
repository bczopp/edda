//! Performance monitor – script execution times and metrics (Phase 12.2.1).

use super::limits::ResourceLimits;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Per-script metrics.
#[derive(Debug, Clone, Default)]
pub struct ScriptMetrics {
    pub count: u64,
    pub total_duration_ms: u64,
}

impl ScriptMetrics {
    pub fn avg_duration_ms(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total_duration_ms as f64 / self.count as f64
        }
    }
}

/// Aggregated performance metrics.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_executions: u64,
    pub script_stats: HashMap<String, ScriptMetrics>,
}

/// Tracks script execution times and collects performance metrics.
pub struct PerformanceMonitor {
    limits: Arc<RwLock<ResourceLimits>>,
    stats: Arc<RwLock<HashMap<String, ScriptMetrics>>>,
    total_executions: Arc<RwLock<u64>>,
}

impl PerformanceMonitor {
    pub fn new(limits: Arc<RwLock<ResourceLimits>>) -> Self {
        Self {
            limits,
            stats: Arc::new(RwLock::new(HashMap::new())),
            total_executions: Arc::new(RwLock::new(0)),
        }
    }

    /// Record a script execution (duration in milliseconds).
    pub async fn record_execution(&self, script_name: &str, duration_ms: u64) {
        *self.total_executions.write().await += 1;
        let mut stats = self.stats.write().await;
        let entry = stats.entry(script_name.to_string()).or_default();
        entry.count += 1;
        entry.total_duration_ms += duration_ms;
    }

    /// Get current performance metrics snapshot.
    pub async fn get_metrics(&self) -> PerformanceMetrics {
        let total = *self.total_executions.read().await;
        let stats = self.stats.read().await.clone();
        PerformanceMetrics {
            total_executions: total,
            script_stats: stats,
        }
    }

    /// Check if execution time exceeds configured limit.
    pub async fn exceeds_execution_limit(&self, duration_ms: u64) -> bool {
        let limits = self.limits.read().await;
        duration_ms > limits.max_execution_time_ms as u64
    }
}
