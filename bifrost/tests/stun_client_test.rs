//! Integration tests for STUN Client (Phase 13.1 – NAT Traversal).
//!
//! - Public-IP ermitteln
//! - NAT-Type ermitteln
//! Uses STUNClientStub for container-friendly tests.

use bifrost::nat::{NatType, STUNClient, STUNClientStub};
use std::net::IpAddr;
use std::sync::Arc;

#[tokio::test]
async fn test_stun_client_stub_creation() {
    let stub = STUNClientStub::new(None, NatType::Unknown);
    let client = STUNClient::new(Arc::new(stub));
    let result = client.get_public_ip_and_nat_type().await;
    assert!(result.is_ok());
    let (ip, nat_type) = result.unwrap();
    assert!(ip.is_none());
    assert_eq!(nat_type, NatType::Unknown);
}

#[tokio::test]
async fn test_stun_client_stub_returns_configured_ip() {
    let stub = STUNClientStub::new(
        Some("203.0.113.1".parse::<IpAddr>().unwrap()),
        NatType::FullCone,
    );
    let client = STUNClient::new(Arc::new(stub));
    let (ip, nat_type) = client.get_public_ip_and_nat_type().await.unwrap();
    assert_eq!(ip, Some("203.0.113.1".parse().unwrap()));
    assert_eq!(nat_type, NatType::FullCone);
}

#[tokio::test]
async fn test_stun_client_stub_nat_types() {
    for nat_type in [
        NatType::Unknown,
        NatType::FullCone,
        NatType::RestrictedCone,
        NatType::PortRestrictedCone,
        NatType::Symmetric,
    ] {
        let stub = STUNClientStub::new(None, nat_type);
        let client = STUNClient::new(Arc::new(stub));
        let (_, t) = client.get_public_ip_and_nat_type().await.unwrap();
        assert_eq!(t, nat_type);
    }
}
