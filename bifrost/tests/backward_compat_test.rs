//! Backward-compatibility and version-mismatch tests (Phase 2.2.2, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::protocol::{ProtocolVersion, VersionMismatchError, VersionNegotiator};

#[test]
fn minor_update_old_client_with_new_server_selects_client_version() {
    // Old client supports only 1.0.0; server supports 1.0.0 and 1.1.0 → use 1.0.0 (backward compat).
    let client = vec![ProtocolVersion::new(1, 0, 0)];
    let server = vec![
        ProtocolVersion::new(1, 0, 0),
        ProtocolVersion::new(1, 1, 0),
    ];
    let result = VersionNegotiator::negotiate(&client, &server).unwrap();
    assert_eq!(result, ProtocolVersion::new(1, 0, 0));
}

#[test]
fn minor_update_new_client_with_old_server_selects_server_version() {
    // Client supports 1.0 and 1.1; server supports only 1.0 → use 1.0.
    let client = vec![
        ProtocolVersion::new(1, 0, 0),
        ProtocolVersion::new(1, 1, 0),
    ];
    let server = vec![ProtocolVersion::new(1, 0, 0)];
    let result = VersionNegotiator::negotiate(&client, &server).unwrap();
    assert_eq!(result, ProtocolVersion::new(1, 0, 0));
}

#[test]
fn major_mismatch_returns_error() {
    // Client only 2.0; server only 1.0 → no common version.
    let client = vec![ProtocolVersion::new(2, 0, 0)];
    let server = vec![ProtocolVersion::new(1, 0, 0)];
    let err = VersionNegotiator::negotiate(&client, &server).unwrap_err();
    assert!(matches!(err, VersionMismatchError::NoCommonVersion));
}

#[test]
fn negotiate_empty_client_returns_error() {
    let client: Vec<ProtocolVersion> = vec![];
    let server = vec![ProtocolVersion::new(1, 0, 0)];
    let err = VersionNegotiator::negotiate(&client, &server).unwrap_err();
    assert!(matches!(err, VersionMismatchError::NoCommonVersion));
}

#[test]
fn negotiate_success_returns_highest_common() {
    let client = vec![
        ProtocolVersion::new(1, 0, 0),
        ProtocolVersion::new(1, 1, 0),
    ];
    let server = vec![
        ProtocolVersion::new(1, 0, 0),
        ProtocolVersion::new(1, 1, 0),
    ];
    let result = VersionNegotiator::negotiate(&client, &server).unwrap();
    assert_eq!(result, ProtocolVersion::new(1, 1, 0));
}

#[test]
fn version_mismatch_error_is_display() {
    let err = VersionMismatchError::NoCommonVersion;
    let s = err.to_string();
    assert!(!s.is_empty());
    assert!(s.to_lowercase().contains("common") || s.to_lowercase().contains("version"));
}
