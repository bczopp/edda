//! Tests für MetricsCollector-Stub (16.2.1).

#[cfg(test)]
mod tests {
    use heimdall::utils::MetricsCollector;

    #[test]
    fn metrics_collector_starts_at_zero() {
        let m = MetricsCollector::new();
        assert_eq!(m.get_token_validations(), 0);
    }

    #[test]
    fn record_token_validation_increments_count() {
        let m = MetricsCollector::new();
        m.record_token_validation();
        assert_eq!(m.get_token_validations(), 1);
        m.record_token_validation();
        m.record_token_validation();
        assert_eq!(m.get_token_validations(), 3);
    }
}
