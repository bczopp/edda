//! UpdateVerifier (Phase 7.2.1, TDD).

use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Invalid hex: {0}")]
    InvalidHex(String),
}

/// Verifies update integrity (checksum, optional signature).
pub struct UpdateVerifier;

impl UpdateVerifier {
    pub fn verify_checksum(data: &[u8], expected_sha256_hex: &str) -> Result<bool, VerificationError> {
        let expected =
            hex::decode(expected_sha256_hex).map_err(|e| VerificationError::InvalidHex(e.to_string()))?;
        let mut hasher = Sha256::new();
        hasher.update(data);
        let got = hasher.finalize();
        Ok(got.as_slice() == expected.as_slice())
    }
}
