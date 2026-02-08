//! Tests for ConnectionResilienceManager (TDD – Phase 10.2.1).

use loki::resilience::ConnectionResilienceManager;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

#[tokio::test]
async fn resilience_retry_succeeds_on_second_attempt() {
    let mgr = ConnectionResilienceManager::new(3, Duration::from_millis(1));
    let attempts = AtomicU32::new(0);
    let result = mgr
        .run_with_retry(|| {
            let n = attempts.fetch_add(1, Ordering::SeqCst);
            async move {
                if n < 1 {
                    Err::<(), _>(shared::LokiError::ServiceUnavailable("retry".into()))
                } else {
                    Ok(())
                }
            }
        })
        .await;
    assert!(result.is_ok());
    assert_eq!(attempts.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn resilience_retry_exhausted_returns_last_error() {
    let mgr = ConnectionResilienceManager::new(2, Duration::from_millis(1));
    let result = mgr
        .run_with_retry(|| async { Err::<(), _>(shared::LokiError::ServiceUnavailable("fail".into())) })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn resilience_retry_succeeds_immediately() {
    let mgr = ConnectionResilienceManager::new(3, Duration::from_millis(10));
    let result = mgr.run_with_retry(|| async { Ok::<_, shared::LokiError>(42u32) }).await;
    assert_eq!(result.unwrap(), 42);
}
