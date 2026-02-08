// UpdateVerifier tests (Phase 7.2.1, TDD).

use jotunheim_esp32::ota::UpdateVerifier;

#[test]
fn verify_checksum_matches_sha256() {
    let data = b"hello";
    let hash = sha2::Sha256::digest(data);
    let hex_str = hex::encode(hash);
    assert!(UpdateVerifier::verify_checksum(data, &hex_str).unwrap());
}

#[test]
fn verify_checksum_rejects_wrong_hash() {
    let data = b"hello";
    assert!(!UpdateVerifier::verify_checksum(data, "wrong").unwrap());
}

#[test]
fn verify_checksum_rejects_invalid_hex() {
    let data = b"x";
    assert!(UpdateVerifier::verify_checksum(data, "zz").is_err());
}
