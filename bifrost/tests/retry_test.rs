//! Retry Manager tests (Phase 10.1.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::routing::RetryManager;
use std::time::Duration;

#[test]
fn first_retry_is_immediate() {
    let mgr = RetryManager::new(5, Duration::from_secs(1));
    assert!(mgr.should_retry());
    assert_eq!(mgr.next_delay(), Duration::ZERO);
}

#[test]
fn next_delay_increases_exponentially() {
    let mut mgr = RetryManager::new(5, Duration::from_secs(1));
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(1));
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(2));
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(4));
}

#[test]
fn should_retry_false_after_max_attempts() {
    let mut mgr = RetryManager::new(3, Duration::from_secs(1));
    assert!(mgr.should_retry());
    mgr.record_attempt();
    assert!(mgr.should_retry());
    mgr.record_attempt();
    assert!(mgr.should_retry());
    mgr.record_attempt();
    assert!(!mgr.should_retry());
}

#[test]
fn reset_restarts_attempt_count() {
    let mut mgr = RetryManager::new(2, Duration::from_secs(1));
    mgr.record_attempt();
    mgr.record_attempt();
    assert!(!mgr.should_retry());
    mgr.reset();
    assert!(mgr.should_retry());
    assert_eq!(mgr.next_delay(), Duration::ZERO);
}

#[test]
fn default_max_retries_is_five() {
    let mgr = RetryManager::default();
    assert!(mgr.should_retry());
}
