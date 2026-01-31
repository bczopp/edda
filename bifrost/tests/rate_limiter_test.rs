//! Tests for Phase 4.3.1: Rate Limiter (token/sliding-window, exceeded handling).

use bifrost::security::rate_limiter::{RateLimitExceeded, RateLimiter};
use std::time::Duration;

#[test]
fn allows_requests_within_limit() {
    let limiter = RateLimiter::new(2, Duration::from_secs(1));
    assert!(limiter.check("key1").is_ok());
    assert!(limiter.check("key1").is_ok());
}

#[test]
fn denies_request_over_limit() {
    let limiter = RateLimiter::new(2, Duration::from_secs(1));
    let _ = limiter.check("key1");
    let _ = limiter.check("key1");
    let res = limiter.check("key1");
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), RateLimitExceeded { .. }));
}

#[test]
fn different_keys_are_limited_separately() {
    let limiter = RateLimiter::new(1, Duration::from_secs(1));
    assert!(limiter.check("key1").is_ok());
    assert!(limiter.check("key2").is_ok());
    assert!(limiter.check("key1").is_err());
    assert!(limiter.check("key2").is_err());
}

#[test]
fn exceeded_error_includes_retry_after() {
    let limiter = RateLimiter::new(1, Duration::from_secs(10));
    let _ = limiter.check("k");
    let err = limiter.check("k").unwrap_err();
    assert!(err.retry_after().as_secs() > 0);
}

#[test]
fn sliding_window_allows_after_window_advances() {
    let limiter = RateLimiter::new(1, Duration::from_millis(50));
    assert!(limiter.check("k").is_ok());
    assert!(limiter.check("k").is_err());
    std::thread::sleep(Duration::from_millis(60));
    assert!(limiter.check("k").is_ok());
}
