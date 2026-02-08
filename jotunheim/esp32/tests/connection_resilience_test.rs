// ConnectionResilienceManager tests (Phase 8.1.1, TDD).

use jotunheim_esp32::resilience::ConnectionResilienceManager;
use std::time::Duration;

#[test]
fn new_starts_with_zero_failures() {
    let m = ConnectionResilienceManager::new(3, 100);
    assert!(m.should_retry());
    assert_eq!(m.retry_count(), 0);
}

#[test]
fn record_failure_increments_count() {
    let m = ConnectionResilienceManager::new(3, 100);
    m.record_failure();
    assert_eq!(m.retry_count(), 1);
    m.record_failure();
    assert_eq!(m.retry_count(), 2);
}

#[test]
fn should_retry_false_after_max_retries() {
    let m = ConnectionResilienceManager::new(2, 100);
    m.record_failure();
    m.record_failure();
    assert!(!m.should_retry());
}

#[test]
fn backoff_delay_increases_with_attempts() {
    let m = ConnectionResilienceManager::new(5, 100);
    let d0 = m.next_backoff_delay();
    m.record_failure();
    let d1 = m.next_backoff_delay();
    m.record_failure();
    let d2 = m.next_backoff_delay();
    assert!(d1 >= d0);
    assert!(d2 >= d1);
}

#[test]
fn record_success_resets_count() {
    let m = ConnectionResilienceManager::new(3, 100);
    m.record_failure();
    m.record_failure();
    m.record_success();
    assert_eq!(m.retry_count(), 0);
}
