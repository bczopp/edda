//! Tests für Model-Performance-Tracker (Phase 15.2.1).

#[cfg(test)]
mod tests {
    use geri::metrics::{ModelMetrics, ModelPerformanceTracker};

    #[test]
    fn get_metrics_returns_none_for_unknown_model() {
        let tracker = ModelPerformanceTracker::new();
        assert!(tracker.get_model_metrics("unknown").is_none());
    }

    #[test]
    fn record_response_tracks_metrics() {
        let mut tracker = ModelPerformanceTracker::new();
        tracker.record_response("gpt-4", 1000, 50, true);
        let m = tracker.get_model_metrics("gpt-4").unwrap();
        assert_eq!(m.request_count(), 1);
        assert_eq!(m.success_count(), 1);
        assert_eq!(m.total_response_time_ms(), 1000);
        assert_eq!(m.total_tokens(), 50);
    }

    #[test]
    fn tokens_per_second() {
        let mut tracker = ModelPerformanceTracker::new();
        tracker.record_response("m", 1000, 100, true);
        let m = tracker.get_model_metrics("m").unwrap();
        let tps = m.average_tokens_per_second().unwrap();
        assert!((tps - 100.0).abs() < 0.01);
    }

    #[test]
    fn availability_tracked() {
        let mut tracker = ModelPerformanceTracker::new();
        tracker.record_response("m", 100, 10, true);
        tracker.record_response("m", 100, 10, false);
        let m = tracker.get_model_metrics("m").unwrap();
        assert_eq!(m.request_count(), 2);
        assert_eq!(m.success_count(), 1);
        assert!((m.availability() - 0.5).abs() < 0.01);
    }

    #[test]
    fn average_response_time_ms() {
        let mut tracker = ModelPerformanceTracker::new();
        tracker.record_response("m", 100, 0, true);
        tracker.record_response("m", 200, 0, true);
        let m = tracker.get_model_metrics("m").unwrap();
        assert_eq!(m.average_response_time_ms(), Some(150.0));
    }

    #[test]
    fn different_models_separate_metrics() {
        let mut tracker = ModelPerformanceTracker::new();
        tracker.record_response("a", 100, 10, true);
        tracker.record_response("b", 200, 20, true);
        let ma = tracker.get_model_metrics("a").unwrap();
        let mb = tracker.get_model_metrics("b").unwrap();
        assert_eq!(ma.total_response_time_ms(), 100);
        assert_eq!(mb.total_response_time_ms(), 200);
        assert_eq!(ma.total_tokens(), 10);
        assert_eq!(mb.total_tokens(), 20);
    }
}
