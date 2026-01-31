// Security-specific test helpers and test data generators.

use uuid::Uuid;

/// Generate a new random UUID for owner/device IDs in tests.
pub fn random_owner_id() -> Uuid {
    Uuid::new_v4()
}

/// Generate a random device ID string for tests.
pub fn random_device_id(prefix: &str) -> String {
    format!("{}-{}", prefix, Uuid::new_v4())
}

/// Generate a base64-like public key string for mesh/device tests (not cryptographically valid).
pub fn random_public_key_base64() -> String {
    use base64::Engine;
    let bytes: [u8; 32] = rand_bytes();
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

fn rand_bytes() -> [u8; 32] {
    use std::sync::atomic::{AtomicU64, Ordering};
    static S: AtomicU64 = AtomicU64::new(0);
    let mut out = [0u8; 32];
    for chunk in out.chunks_exact_mut(8) {
        let v = S.fetch_add(1, Ordering::Relaxed).to_le_bytes();
        chunk.copy_from_slice(&v);
    }
    out
}

/// Fixed test token string for tests that need a predictable token.
pub const TEST_TOKEN_STR: &str = "test-token-constant";

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;

    #[test]
    fn random_device_id_has_prefix() {
        let id = random_device_id("mesh");
        assert!(id.starts_with("mesh-"));
    }

    #[test]
    fn random_public_key_base64_is_base64() {
        let s = random_public_key_base64();
        let decoded = base64::engine::general_purpose::STANDARD.decode(&s);
        assert!(decoded.is_ok());
    }
}
