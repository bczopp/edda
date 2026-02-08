// RetryManager tests (Phase 8.3.1, TDD).

use jotunheim_esp32::resilience::RetryManager;
use std::time::Duration;

#[test]
fn next_delay_exponential_backoff() {
    let m = RetryManager::new(5, 100);
    let d0 = m.next_delay(0);
    let d1 = m.next_delay(1);
    let d2 = m.next_delay(2);
    assert!(d0 <= d1);
    assert!(d1 <= d2);
    assert!(d0.as_millis() >= 50); // base or capped
}

#[test]
fn exhausted_after_max_attempts() {
    let m = RetryManager::new(3, 100);
    assert!(!m.exhausted(0));
    assert!(!m.exhausted(2));
    assert!(m.exhausted(3));
    assert!(m.exhausted(4));
}

#[test]
fn next_delay_capped_or_reasonable() {
    let m = RetryManager::new(10, 1000);
    let d = m.next_delay(20); // high attempt
    assert!(d.as_millis() <= 60_000); // e.g. cap at 60s
}
