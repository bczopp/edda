//! Version negotiation tests (Phase 2.2.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::protocol::{ProtocolVersion, VersionNegotiator};

#[test]
fn select_highest_common_when_same_version_returns_it() {
    let client = vec![ProtocolVersion::new(1, 0, 0)];
    let server = vec![ProtocolVersion::new(1, 0, 0)];
    let selected = VersionNegotiator::select_highest_common(&client, &server);
    assert_eq!(selected, Some(ProtocolVersion::new(1, 0, 0)));
}

#[test]
fn select_highest_common_returns_highest_common_version() {
    let client = vec![
        ProtocolVersion::new(1, 0, 0),
        ProtocolVersion::new(1, 1, 0),
        ProtocolVersion::new(2, 0, 0),
    ];
    let server = vec![
        ProtocolVersion::new(1, 0, 0),
        ProtocolVersion::new(1, 1, 0),
    ];
    let selected = VersionNegotiator::select_highest_common(&client, &server);
    assert_eq!(selected, Some(ProtocolVersion::new(1, 1, 0)));
}

#[test]
fn select_highest_common_when_no_common_returns_none() {
    let client = vec![ProtocolVersion::new(2, 0, 0)];
    let server = vec![ProtocolVersion::new(1, 0, 0)];
    let selected = VersionNegotiator::select_highest_common(&client, &server);
    assert_eq!(selected, None);
}

#[test]
fn select_highest_common_when_client_empty_returns_none() {
    let client: Vec<ProtocolVersion> = vec![];
    let server = vec![ProtocolVersion::new(1, 0, 0)];
    let selected = VersionNegotiator::select_highest_common(&client, &server);
    assert_eq!(selected, None);
}

#[test]
fn select_highest_common_when_server_empty_returns_none() {
    let client = vec![ProtocolVersion::new(1, 0, 0)];
    let server: Vec<ProtocolVersion> = vec![];
    let selected = VersionNegotiator::select_highest_common(&client, &server);
    assert_eq!(selected, None);
}

#[test]
fn protocol_version_ordering() {
    let v100 = ProtocolVersion::new(1, 0, 0);
    let v110 = ProtocolVersion::new(1, 1, 0);
    let v200 = ProtocolVersion::new(2, 0, 0);
    assert!(v100 < v110);
    assert!(v110 < v200);
    assert!(v100 < v200);
}

#[test]
fn protocol_version_equality() {
    let a = ProtocolVersion::new(1, 2, 3);
    let b = ProtocolVersion::new(1, 2, 3);
    assert_eq!(a, b);
}

#[test]
fn supported_versions_returns_non_empty() {
    let supported = VersionNegotiator::supported_versions();
    assert!(!supported.is_empty());
    assert!(supported.contains(&ProtocolVersion::new(1, 0, 0)));
}
