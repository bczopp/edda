use nidhoggr::security::SecurityMonitor;
use nidhoggr::security::{SecurityEventType, SecuritySeverity};
use nidhoggr::ratelimiter::RateLimiter;
use std::sync::Arc;

#[tokio::test]
async fn test_security_monitor_records_events() {
    let monitor = Arc::new(SecurityMonitor::new(1000));
    
    monitor.record_event(
        SecurityEventType::RateLimitExceeded,
        "test-device".to_string(),
        "test-user".to_string(),
        "Rate limit exceeded".to_string(),
        SecuritySeverity::High,
    ).await;
    
    let events = monitor.get_recent_events(10).await;
    assert!(!events.is_empty(), "Should record security events");
    assert_eq!(events[0].event_type, SecurityEventType::RateLimitExceeded);
    assert_eq!(events[0].severity, SecuritySeverity::High);
}

#[tokio::test]
async fn test_security_monitor_filters_by_severity() {
    let monitor = Arc::new(SecurityMonitor::new(1000));
    
    monitor.record_event(
        SecurityEventType::RateLimitExceeded,
        "test-device".to_string(),
        "test-user".to_string(),
        "Rate limit exceeded".to_string(),
        SecuritySeverity::High,
    ).await;
    
    monitor.record_event(
        SecurityEventType::ConnectionAttempt,
        "test-device".to_string(),
        "test-user".to_string(),
        "Connection attempt".to_string(),
        SecuritySeverity::Low,
    ).await;
    
    let high_severity_events = monitor.get_events_by_severity(SecuritySeverity::High).await;
    assert!(!high_severity_events.is_empty(), "Should filter by severity");
    assert!(high_severity_events.iter().all(|e| e.severity >= SecuritySeverity::High));
}

#[tokio::test]
async fn test_security_monitor_detects_suspicious_activity() {
    let monitor = Arc::new(SecurityMonitor::new(1000));
    
    // Record multiple events from same device
    for _ in 0..20 {
        monitor.record_event(
            SecurityEventType::RateLimitExceeded,
            "suspicious-device".to_string(),
            "test-user".to_string(),
            "Rate limit exceeded".to_string(),
            SecuritySeverity::High,
        ).await;
    }
    
    let is_suspicious = monitor.check_suspicious_activity("suspicious-device", 10).await;
    assert!(is_suspicious, "Should detect suspicious activity");
    
    let is_not_suspicious = monitor.check_suspicious_activity("normal-device", 10).await;
    assert!(!is_not_suspicious, "Should not flag normal devices");
}

#[tokio::test]
async fn test_rate_limiter_security_integration() {
    let limiter = Arc::new(RateLimiter::new(5, 100)); // 5 per minute
    let monitor = Arc::new(SecurityMonitor::new(1000));
    
    // Make requests up to limit
    for _ in 0..5 {
        let _ = limiter.check_rate_limit("test-device", "test-user").await;
    }
    
    // Next request should be blocked
    let result = limiter.check_rate_limit("test-device", "test-user").await;
    assert!(result.is_err(), "Should block requests over limit");
    
    // Security monitor should record this
    monitor.record_event(
        SecurityEventType::RateLimitExceeded,
        "test-device".to_string(),
        "test-user".to_string(),
        "Rate limit exceeded".to_string(),
        SecuritySeverity::High,
    ).await;
    
    let events = monitor.get_recent_events(1).await;
    assert!(!events.is_empty(), "Should record rate limit events");
}
