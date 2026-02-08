#[cfg(test)]
mod tests {
    use freki::utils::MetricsCollector;
    use std::time::Duration;

    #[test]
    fn test_initial_counts_zero() {
        let m = MetricsCollector::new();
        assert_eq!(m.get_indexing_count(), 0);
        assert_eq!(m.get_query_count(), 0);
        assert_eq!(m.get_avg_indexing_time_ms(), 0.0);
        assert_eq!(m.get_avg_search_time_ms(), 0.0);
    }

    #[test]
    fn test_increment_counts() {
        let m = MetricsCollector::new();
        m.increment_indexing_count();
        m.increment_indexing_count();
        m.increment_query_count();
        assert_eq!(m.get_indexing_count(), 2);
        assert_eq!(m.get_query_count(), 1);
    }

    #[test]
    fn test_record_indexing_time() {
        let m = MetricsCollector::new();
        m.record_indexing_time(Duration::from_millis(100));
        m.record_indexing_time(Duration::from_millis(200));
        assert_eq!(m.get_avg_indexing_time_ms(), 150.0);
    }

    #[test]
    fn test_record_search_time() {
        let m = MetricsCollector::new();
        m.record_search_time(Duration::from_millis(50));
        assert_eq!(m.get_avg_search_time_ms(), 50.0);
    }

    #[test]
    fn test_avg_zero_when_no_records() {
        let m = MetricsCollector::new();
        m.increment_query_count();
        assert_eq!(m.get_avg_search_time_ms(), 0.0);
    }
}
