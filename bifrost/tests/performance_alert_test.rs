//! Tests for Phase 14.2.2: PerformanceAlertManager (threshold-based alerts).

use bifrost::utils::metrics::{MetricsCollector, PerformanceAlertManager};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn no_alert_when_below_threshold() {
    let metrics = Arc::new(MetricsCollector::new());
    metrics.record_response_time("route", Duration::from_millis(10));
    let alert_mgr = PerformanceAlertManager::new(Arc::clone(&metrics));
    alert_mgr.set_latency_threshold_ms("route", 100);
    let alerts = alert_mgr.check_alerts();
    assert!(alerts.is_empty());
}

#[test]
fn alert_when_latency_above_threshold() {
    let metrics = Arc::new(MetricsCollector::new());
    metrics.record_response_time("route", Duration::from_millis(200));
    let alert_mgr = PerformanceAlertManager::new(Arc::clone(&metrics));
    alert_mgr.set_latency_threshold_ms("route", 100);
    let alerts = alert_mgr.check_alerts();
    assert!(!alerts.is_empty());
    assert!(alerts[0].message.contains("latency") || alerts[0].message.contains("route"));
}

#[test]
fn alert_when_connections_above_threshold() {
    let metrics = Arc::new(MetricsCollector::new());
    metrics.record_connections_count(1000);
    let alert_mgr = PerformanceAlertManager::new(Arc::clone(&metrics));
    alert_mgr.set_connections_threshold(500);
    let alerts = alert_mgr.check_alerts();
    assert!(!alerts.is_empty());
}
