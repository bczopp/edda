//! Tests for Phase 5.3.2: CrossUserConnectionBlocker (block direct connections between different users, enforce Yggdrasil relay).

use bifrost::heimdall::{CrossUserConnectionBlocker, UserIdentityVerifier};

#[test]
fn allow_direct_connection_when_same_user() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(blocker.allow_direct_connection("user-1", "user-1"));
}

#[test]
fn block_direct_connection_when_different_users() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(!blocker.allow_direct_connection("user-1", "user-2"));
}

#[test]
fn requires_relay_when_different_users() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(blocker.requires_relay("user-1", "user-2"));
}

#[test]
fn does_not_require_relay_when_same_user() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(!blocker.requires_relay("user-1", "user-1"));
}

#[test]
fn block_direct_is_inverse_of_allow_direct() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(blocker.allow_direct_connection("a", "a"));
    assert!(!blocker.allow_direct_connection("a", "b"));
    assert!(blocker.requires_relay("a", "b"));
    assert!(!blocker.requires_relay("a", "a"));
}

#[test]
fn blocker_accepts_verifier() {
    let verifier = UserIdentityVerifier::new(None);
    let _blocker = CrossUserConnectionBlocker::new(verifier);
}
