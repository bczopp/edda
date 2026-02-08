//! OTA (Over-the-Air) updates (Phase 7).

pub mod client;
pub mod fetcher;
pub mod rollback;
pub mod verifier;

pub use client::{OtaUpdateClient, OtaUpdateError};
pub use fetcher::UpdateFetcher;
pub use rollback::{RollbackHandler, RollbackError};
pub use verifier::{UpdateVerifier, VerificationError};

/// Legacy alias.
pub struct OtaManager;

impl OtaManager {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OtaManager {
    fn default() -> Self {
        Self::new()
    }
}
