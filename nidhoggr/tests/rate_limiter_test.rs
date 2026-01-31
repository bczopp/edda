use nidhoggr::ratelimiter::RateLimiter;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_rate_limiter_allows_requests_within_limit() {
    let limiter = RateLimiter::new(60, 1000); // 60 per minute, 1000 per hour
    
    for _ in 0..60 {
        let allowed = limiter.check_rate_limit("test-device", "test-user").await;
        assert!(allowed.is_ok());
    }
}

#[tokio::test]
async fn test_rate_limiter_blocks_requests_over_minute_limit() {
    let limiter = RateLimiter::new(60, 1000); // 60 per minute, 1000 per hour
    
    // Make 60 requests (at limit)
    for _ in 0..60 {
        let _ = limiter.check_rate_limit("test-device", "test-user").await;
    }
    
    // 61st request should be blocked
    let result = limiter.check_rate_limit("test-device", "test-user").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_rate_limiter_resets_after_time_window() {
    let limiter = RateLimiter::new(60, 1000);
    
    // Exhaust limit
    for _ in 0..60 {
        let _ = limiter.check_rate_limit("test-device", "test-user").await;
    }
    
    // Wait for time window to reset (in real implementation, this would be configurable)
    sleep(Duration::from_secs(61)).await;
    
    // Should be able to make requests again
    let result = limiter.check_rate_limit("test-device", "test-user").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_rate_limiter_tracks_per_device() {
    let limiter = RateLimiter::new(60, 1000);
    
    // Device 1 exhausts limit
    for _ in 0..60 {
        let _ = limiter.check_rate_limit("device-1", "user-1").await;
    }
    
    // Device 2 should still be able to make requests
    let result = limiter.check_rate_limit("device-2", "user-1").await;
    assert!(result.is_ok());
}
