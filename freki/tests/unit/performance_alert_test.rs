//! Unit tests for PerformanceAlertManager (Phase 15.2.2).

#[cfg(test)]
mod tests {
    use freki::utils::{MetricsCollector, PerformanceAlert, PerformanceAlertManager};
    use std::time::Duration;

    #[test]
    fn test_no_alerts_when_under_threshold() {
        let metrics = MetricsCollector::new();
        metrics.record_indexing_time(Duration::from_millis(50));
        metrics.record_search_time(Duration::from_millis(20));
        let manager = PerformanceAlertManager::new(1000.0, 500.0);
        let alerts = manager.check_alerts(&metrics);
        assert!(alerts.is_empty(), "no alerts when under threshold");
    }

    #[test]
    fn test_alert_when_indexing_above_threshold() {
        let metrics = MetricsCollector::new();
        metrics.record_indexing_time(Duration::from_millis(2000));
        let manager = PerformanceAlertManager::new(1000.0, 500.0);
        let alerts = manager.check_alerts(&metrics);
        assert_eq!(alerts.len(), 1);
        assert!(matches!(alerts[0], PerformanceAlert::IndexingSlow { current_avg_ms: 2000.0 }));
    }

    #[test]
    fn test_alert_when_search_above_threshold() {
        let metrics = MetricsCollector::new();
        metrics.record_search_time(Duration::from_millis(600));
        let manager = PerformanceAlertManager::new(1000.0, 500.0);
        let alerts = manager.check_alerts(&metrics);
        assert_eq!(alerts.len(), 1);
        assert!(matches!(alerts[0], PerformanceAlert::SearchSlow { current_avg_ms: 600.0 }));
    }

    #[test]
    fn test_both_alerts_when_both_above_threshold() {
        let metrics = MetricsCollector::new();
        metrics.record_indexing_time(Duration::from_millis(1500));
        metrics.record_search_time(Duration::from_millis(600));
        let manager = PerformanceAlertManager::new(1000.0, 500.0);
        let alerts = manager.check_alerts(&metrics);
        assert_eq!(alerts.len(), 2);
        let has_indexing = alerts.iter().any(|a| matches!(a, PerformanceAlert::IndexingSlow { .. }));
        let has_search = alerts.iter().any(|a| matches!(a, PerformanceAlert::SearchSlow { .. }));
        assert!(has_indexing && has_search);
    }

    #[test]
    fn test_no_alerts_when_no_metrics_recorded() {
        let metrics = MetricsCollector::new();
        let manager = PerformanceAlertManager::new(100.0, 100.0);
        let alerts = manager.check_alerts(&metrics);
        assert!(alerts.is_empty());
    }
}
