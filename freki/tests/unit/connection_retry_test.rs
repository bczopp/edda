#[cfg(test)]
mod tests {
    use freki::vector_db::ConnectionRetryManager;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Duration;

    #[derive(Debug)]
    struct TestError(u32);
    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "attempt {}", self.0)
        }
    }
    impl std::error::Error for TestError {}

    #[tokio::test]
    async fn test_connect_with_retry_success_first_try() {
        let m = ConnectionRetryManager::new(3, Duration::from_millis(1), Duration::from_secs(1));
        let r = m
            .connect_with_retry(|| async { Ok::<i32, TestError>(42) })
            .await;
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_connect_with_retry_success_after_failures() {
        let m = ConnectionRetryManager::new(3, Duration::from_millis(1), Duration::from_secs(1));
        let count = std::sync::Arc::new(AtomicU32::new(0));
        let count_clone = std::sync::Arc::clone(&count);
        let r = m
            .connect_with_retry(move || {
                let c = std::sync::Arc::clone(&count_clone);
                async move {
                    let n = c.fetch_add(1, Ordering::SeqCst);
                    if n < 2 {
                        Err(TestError(n))
                    } else {
                        Ok(100)
                    }
                }
            })
            .await;
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 100);
        assert_eq!(count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_connect_with_retry_fail_after_max() {
        let m = ConnectionRetryManager::new(2, Duration::from_millis(1), Duration::from_secs(1));
        let r = m
            .connect_with_retry(|| async { Err::<i32, _>(TestError(0)) })
            .await;
        assert!(r.is_err());
    }

    #[test]
    fn test_default_config() {
        let m = ConnectionRetryManager::default();
        assert!(m.max_retries >= 1);
        assert!(!m.initial_delay.is_zero());
    }
}
