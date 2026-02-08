//! Cross-User Connection Blocking (Phase 5.3.2). Block direct connections between different users, enforce Yggdrasil relay.

use crate::heimdall::UserIdentityVerifier;

/// Blocks direct connections between different users; enforces Yggdrasil relay for cross-user traffic.
pub struct CrossUserConnectionBlocker {
    verifier: UserIdentityVerifier,
}

impl CrossUserConnectionBlocker {
    pub fn new(verifier: UserIdentityVerifier) -> Self {
        Self { verifier }
    }

    /// Returns true if a direct connection between the two users is allowed (same user only).
    pub fn allow_direct_connection(&self, user_id_a: &str, user_id_b: &str) -> bool {
        self.verifier.same_user(user_id_a, user_id_b)
    }

    /// Returns true if traffic between these users must go via Yggdrasil relay (different users).
    pub fn requires_relay(&self, user_id_a: &str, user_id_b: &str) -> bool {
        !self.allow_direct_connection(user_id_a, user_id_b)
    }
}
