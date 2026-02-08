//! Tests for PerformanceMonitor (TDD – Phase 12.2.1).

use loki::resources::performance_monitor::{PerformanceMonitor, ScriptMetrics};
use loki::resources::ResourceLimits;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn performance_monitor_records_execution_time() {
    let monitor = PerformanceMonitor::new(Arc::new(RwLock::new(Default::default())));
    monitor.record_execution("script_a", 10).await;
    monitor.record_execution("script_a", 20).await;
    monitor.record_execution("script_b", 5).await;

    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.total_executions, 3);
    assert_eq!(metrics.script_stats.get("script_a").unwrap().count, 2);
    assert_eq!(metrics.script_stats.get("script_a").unwrap().total_duration_ms, 30);
    assert_eq!(metrics.script_stats.get("script_b").unwrap().count, 1);
    assert_eq!(metrics.script_stats.get("script_b").unwrap().total_duration_ms, 5);
}

#[tokio::test]
async fn performance_monitor_avg_duration() {
    let monitor = PerformanceMonitor::new(Arc::new(RwLock::new(ResourceLimits::default())));
    monitor.record_execution("x", 10).await;
    monitor.record_execution("x", 20).await;

    let metrics = monitor.get_metrics().await;
    let x = metrics.script_stats.get("x").unwrap();
    assert_eq!(x.count, 2);
    assert_eq!(x.avg_duration_ms(), 15.0);
}

#[tokio::test]
async fn performance_monitor_empty_metrics() {
    let monitor = PerformanceMonitor::new(Arc::new(RwLock::new(ResourceLimits::default())));
    let metrics = monitor.get_metrics().await;
    assert_eq!(metrics.total_executions, 0);
    assert!(metrics.script_stats.is_empty());
}

#[tokio::test]
async fn performance_monitor_exceeds_execution_limit() {
    let mut limits = ResourceLimits::default();
    limits.max_execution_time_ms = 100;
    let monitor = PerformanceMonitor::new(Arc::new(RwLock::new(limits)));
    assert!(!monitor.exceeds_execution_limit(50).await);
    assert!(monitor.exceeds_execution_limit(150).await);
}
