//! Tests für Retry-Manager (Phase 16.2.1).

#[cfg(test)]
mod tests {
    use geri::error_handling::RetryManager;
    use std::time::Duration;

    #[test]
    fn delay_for_attempt_zero_returns_base_delay() {
        let mgr = RetryManager::new(3, 100);
        let d = mgr.delay_for_attempt(0);
        assert_eq!(d, Duration::from_millis(100));
    }

    #[test]
    fn delay_for_attempt_increases_exponentially() {
        let mgr = RetryManager::new(5, 50);
        assert_eq!(mgr.delay_for_attempt(0), Duration::from_millis(50));
        assert_eq!(mgr.delay_for_attempt(1), Duration::from_millis(100));
        assert_eq!(mgr.delay_for_attempt(2), Duration::from_millis(200));
    }

    #[test]
    fn should_retry_true_before_max() {
        let mgr = RetryManager::new(3, 100);
        assert!(mgr.should_retry(0));
        assert!(mgr.should_retry(1));
        assert!(mgr.should_retry(2));
    }

    #[test]
    fn should_retry_false_at_or_after_max() {
        let mgr = RetryManager::new(3, 100);
        assert!(!mgr.should_retry(3));
        assert!(!mgr.should_retry(4));
    }

    #[test]
    fn delay_capped_at_max() {
        let mgr = RetryManager::new(10, 1000);
        let d = mgr.delay_for_attempt(20);
        assert!(d <= Duration::from_secs(60));
    }
}
