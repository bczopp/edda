//! Tests for Phase 12.2.3: Permission Token Manager (generate, validate, 24h expiry).

use bifrost::guest::permission::{PermissionTokenManager, TokenValidationError};
use std::sync::Arc;
use std::time::Duration;

fn default_expiry() -> Duration {
    Duration::from_secs(24 * 3600)
}

#[test]
fn generate_token_returns_non_empty_token() {
    let mgr = PermissionTokenManager::new(default_expiry());
    let token = mgr.generate("user-1", "device-guest", "main");
    assert!(!token.is_empty());
}

#[test]
fn generated_token_validates_successfully() {
    let mgr = Arc::new(PermissionTokenManager::new(default_expiry()));
    let token = mgr.generate("user-1", "device-guest", "main");
    let result = mgr.validate(&token);
    assert!(result.is_ok());
    let info = result.unwrap();
    assert_eq!(info.user_id, "user-1");
    assert_eq!(info.device_id, "device-guest");
    assert_eq!(info.mesh_id, "main");
}

#[test]
fn invalid_token_returns_error() {
    let mgr = PermissionTokenManager::new(default_expiry());
    let result = mgr.validate("invalid-token");
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TokenValidationError::InvalidOrExpired));
}

#[test]
fn expired_token_returns_error() {
    let mgr = PermissionTokenManager::new(Duration::from_secs(0)); // expire immediately
    let token = mgr.generate("user-1", "device-guest", "main");
    let result = mgr.validate(&token);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TokenValidationError::InvalidOrExpired));
}

#[test]
fn revoke_token_invalidates_future_validation() {
    let mgr = Arc::new(PermissionTokenManager::new(default_expiry()));
    let token = mgr.generate("user-1", "device-guest", "main");
    assert!(mgr.validate(&token).is_ok());
    mgr.revoke(&token);
    let result = mgr.validate(&token);
    assert!(result.is_err());
}

#[test]
fn different_tokens_for_different_requests() {
    let mgr = PermissionTokenManager::new(default_expiry());
    let t1 = mgr.generate("user-1", "device-a", "main");
    let t2 = mgr.generate("user-1", "device-b", "guest-xyz");
    assert_ne!(t1, t2);
}
