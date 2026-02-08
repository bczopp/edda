//! Tests für Performance-Metrics-Collector (Phase 15.1.1).

#[cfg(test)]
mod tests {
    use geri::metrics::{MetricsCollector, MetricsSnapshot};

    #[test]
    fn snapshot_empty_has_zero_count() {
        let collector = MetricsCollector::new();
        let snap = collector.snapshot();
        assert_eq!(snap.request_count(), 0);
    }

    #[test]
    fn record_response_increases_count() {
        let mut collector = MetricsCollector::new();
        collector.record_response(100);
        collector.record_response(200);
        let snap = collector.snapshot();
        assert_eq!(snap.request_count(), 2);
    }

    #[test]
    fn snapshot_tracks_response_times() {
        let mut collector = MetricsCollector::new();
        collector.record_response(10);
        collector.record_response(20);
        collector.record_response(30);
        let snap = collector.snapshot();
        assert_eq!(snap.total_response_time_ms(), 60);
        assert_eq!(snap.min_response_time_ms(), Some(10));
        assert_eq!(snap.max_response_time_ms(), Some(30));
    }

    #[test]
    fn snapshot_average_response_time() {
        let mut collector = MetricsCollector::new();
        collector.record_response(100);
        collector.record_response(200);
        let snap = collector.snapshot();
        assert_eq!(snap.average_response_time_ms(), Some(150.0));
    }

    #[test]
    fn snapshot_average_none_when_empty() {
        let collector = MetricsCollector::new();
        let snap = collector.snapshot();
        assert_eq!(snap.average_response_time_ms(), None);
    }

    #[test]
    fn min_max_none_when_empty() {
        let collector = MetricsCollector::new();
        let snap = collector.snapshot();
        assert_eq!(snap.min_response_time_ms(), None);
        assert_eq!(snap.max_response_time_ms(), None);
    }
}
