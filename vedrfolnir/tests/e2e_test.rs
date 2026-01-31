use vedrfolnir::connection::{ConnectionBuilder, ConnectionManager};
use vedrfolnir::auth::AuthManager;
use vedrfolnir::ratelimiter::RateLimiter;
use vedrfolnir::audit::AuditLogger;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_e2e_connection_establishment() {
    // This test would require a mock Yggdrasil server
    // For now, we test the components individually
    
    let auth_manager = Arc::new(AuthManager::new());
    let device_identity = "test-device-identity".to_string();
    
    // Authenticate
    let auth_token = auth_manager.authenticate(&device_identity).await
        .expect("Authentication should succeed");
    
    assert!(!auth_token.is_empty(), "Auth token should not be empty");
}

#[tokio::test]
async fn test_rate_limiter() {
    let limiter = Arc::new(RateLimiter::new(5, 60)); // 5 per minute
    
    // Make requests up to limit
    for _ in 0..5 {
        let result = limiter.check_rate_limit("test-key").await;
        assert!(result.is_ok(), "Should allow requests within limit");
    }
    
    // Next request should be blocked
    let result = limiter.check_rate_limit("test-key").await;
    assert!(result.is_err(), "Should block requests over limit");
}

#[tokio::test]
async fn test_audit_logger() {
    let logger = Arc::new(AuditLogger::new(1000));
    
    logger.log(
        vedrfolnir::audit::AuditEventType::ConnectionEstablished,
        Some("test-device".to_string()),
        Some("test-user".to_string()),
        "Connection established".to_string(),
        true,
    ).await;
    
    let logs = logger.get_recent_logs(10).await;
    assert!(!logs.is_empty(), "Should record audit logs");
    assert_eq!(logs[0].event_type, vedrfolnir::audit::AuditEventType::ConnectionEstablished);
}

#[tokio::test]
async fn test_retry_mechanism() {
    use vedrfolnir::retry::exponential_backoff::ExponentialBackoff;
    
    let backoff = ExponentialBackoff::new(100, 1000, 2.0, 3);
    
    let mut attempt_count = 0;
    let result = backoff.execute(|| {
        attempt_count += 1;
        Box::pin(async move {
            if attempt_count < 3 {
                Err("Temporary failure".to_string())
            } else {
                Ok("Success".to_string())
            }
        })
    }).await;
    
    assert!(result.is_ok(), "Should succeed after retries");
    assert_eq!(attempt_count, 3, "Should retry 3 times");
}
