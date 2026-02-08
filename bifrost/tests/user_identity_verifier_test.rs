//! Tests for Phase 5.3.1: UserIdentityVerifier (same user vs different user, network membership, confirmation status).

use bifrost::heimdall::{
    UserIdentityProvider, UserIdentityVerifier, UserVerificationOutcome,
};
use std::sync::Arc;

#[test]
fn same_user_returns_true_when_ids_equal() {
    let verifier = UserIdentityVerifier::new(None);
    assert!(verifier.same_user("user-1", "user-1"));
}

#[test]
fn same_user_returns_false_when_ids_differ() {
    let verifier = UserIdentityVerifier::new(None);
    assert!(!verifier.same_user("user-1", "user-2"));
}

#[test]
fn same_user_empty_strings() {
    let verifier = UserIdentityVerifier::new(None);
    assert!(verifier.same_user("", ""));
    assert!(!verifier.same_user("", "user-1"));
}

#[test]
fn is_member_without_provider_defaults_true() {
    let verifier = UserIdentityVerifier::new(None);
    assert!(verifier.is_member("user-1"));
}

#[test]
fn is_confirmed_without_provider_defaults_true() {
    let verifier = UserIdentityVerifier::new(None);
    assert!(verifier.is_confirmed("user-1"));
}

#[test]
fn is_member_delegates_to_provider() {
    let stub = Arc::new(StubProvider::member(true).confirmed(true));
    let verifier = UserIdentityVerifier::new(Some(stub.clone()));
    assert!(verifier.is_member("user-1"));

    let stub_deny = Arc::new(StubProvider::member(false).confirmed(true));
    let verifier_deny = UserIdentityVerifier::new(Some(stub_deny));
    assert!(!verifier_deny.is_member("user-1"));
}

#[test]
fn is_confirmed_delegates_to_provider() {
    let stub = Arc::new(StubProvider::member(true).confirmed(false));
    let verifier = UserIdentityVerifier::new(Some(stub));
    assert!(!verifier.is_confirmed("user-1"));
}

#[test]
fn verify_user_without_provider_returns_ok() {
    let verifier = UserIdentityVerifier::new(None);
    let outcome = verifier.verify_user("user-1");
    assert_eq!(outcome, UserVerificationOutcome::Allowed);
}

#[test]
fn verify_user_denied_when_not_member() {
    let stub = Arc::new(StubProvider::member(false).confirmed(true));
    let verifier = UserIdentityVerifier::new(Some(stub));
    let outcome = verifier.verify_user("user-1");
    assert_eq!(outcome, UserVerificationOutcome::NotMember);
}

#[test]
fn verify_user_denied_when_not_confirmed() {
    let stub = Arc::new(StubProvider::member(true).confirmed(false));
    let verifier = UserIdentityVerifier::new(Some(stub));
    let outcome = verifier.verify_user("user-1");
    assert_eq!(outcome, UserVerificationOutcome::NotConfirmed);
}

#[test]
fn verify_user_allowed_when_member_and_confirmed() {
    let stub = Arc::new(StubProvider::member(true).confirmed(true));
    let verifier = UserIdentityVerifier::new(Some(stub));
    let outcome = verifier.verify_user("user-1");
    assert_eq!(outcome, UserVerificationOutcome::Allowed);
}

/// Stub provider for tests.
struct StubProvider {
    member: bool,
    confirmed: bool,
}

impl StubProvider {
    fn member(member: bool) -> Self {
        Self {
            member,
            confirmed: true,
        }
    }
    fn confirmed(mut self, confirmed: bool) -> Self {
        self.confirmed = confirmed;
        self
    }
}

impl UserIdentityProvider for StubProvider {
    fn is_network_member(&self, _user_id: &str) -> bool {
        self.member
    }
    fn is_confirmed(&self, _user_id: &str) -> bool {
        self.confirmed
    }
}
