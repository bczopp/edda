//! Unit tests for PermissionChecker (Phase 8).
//! PermissionChecker calls Heimdall gRPC; without a running Heimdall, connection fails.

use thor::permissions::{PermissionChecker, PermissionError};
use std::sync::Arc;

#[tokio::test]
async fn test_permission_checker_new() {
    let checker = PermissionChecker::new("http://localhost:50051".to_string());
    let _ = checker; // construction succeeds
}

#[tokio::test]
async fn test_permission_checker_returns_error_when_heimdall_unreachable() {
    // No Heimdall server on this port in unit test env → connection fails
    let checker = Arc::new(PermissionChecker::new("http://127.0.0.1:17999".to_string()));
    let result = checker
        .check_permission("device-1", "user-1", "file", "read")
        .await;
    assert!(result.is_err(), "Without Heimdall, check_permission should fail");
    assert!(matches!(result.unwrap_err(), PermissionError::ConnectionError(_)));
}

#[tokio::test]
async fn test_permission_checker_allow_on_connection_error_for_tests() {
    // With fallback, when Heimdall is unreachable we get Ok(true) so tests can run without Heimdall
    let checker = PermissionChecker::new_allow_on_connection_error("http://127.0.0.1:17998".to_string());
    let ok = checker
        .check_permission("dev", "usr", "resource_type", "action")
        .await
        .unwrap();
    assert!(ok);
}
