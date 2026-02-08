//! User-Identity Verification (Phase 5.3.1). Same user vs different user, Edda network membership, confirmation status.

use std::sync::Arc;

/// Outcome of verifying a user for connection (membership + confirmation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserVerificationOutcome {
    /// User is network member and confirmed.
    Allowed,
    /// User is not an Edda network member.
    NotMember,
    /// User is not confirmed.
    NotConfirmed,
}

/// Provider for Edda network membership and confirmation status (e.g. Yggdrasil/Heimdall).
pub trait UserIdentityProvider: Send + Sync {
    /// Returns whether the user is a member of the Edda network.
    fn is_network_member(&self, user_id: &str) -> bool;
    /// Returns whether the user/device is confirmed (e.g. verified).
    fn is_confirmed(&self, user_id: &str) -> bool;
}

/// Verifies user identity: same-user comparison, network membership, confirmation status.
pub struct UserIdentityVerifier {
    provider: Option<Arc<dyn UserIdentityProvider>>,
}

impl UserIdentityVerifier {
    pub fn new(provider: Option<Arc<dyn UserIdentityProvider>>) -> Self {
        Self { provider }
    }

    /// Returns true if both user IDs refer to the same user.
    pub fn same_user(&self, user_id_a: &str, user_id_b: &str) -> bool {
        user_id_a == user_id_b
    }

    /// Returns whether the user is an Edda network member. Without provider, defaults to true.
    pub fn is_member(&self, user_id: &str) -> bool {
        self.provider
            .as_ref()
            .map(|p| p.is_network_member(user_id))
            .unwrap_or(true)
    }

    /// Returns whether the user is confirmed. Without provider, defaults to true.
    pub fn is_confirmed(&self, user_id: &str) -> bool {
        self.provider
            .as_ref()
            .map(|p| p.is_confirmed(user_id))
            .unwrap_or(true)
    }

    /// Verifies user for connection: must be member and confirmed. Returns outcome.
    pub fn verify_user(&self, user_id: &str) -> UserVerificationOutcome {
        if !self.is_member(user_id) {
            return UserVerificationOutcome::NotMember;
        }
        if !self.is_confirmed(user_id) {
            return UserVerificationOutcome::NotConfirmed;
        }
        UserVerificationOutcome::Allowed
    }
}
