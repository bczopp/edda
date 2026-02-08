//! Integration tests for TURN Client (Phase 13.2 – NAT Traversal).
//!
//! - Relay allocation (connection to TURN server, relay address/port)
//! Uses TURNClientStub for container-friendly tests.

use bifrost::nat::{RelayAllocation, TURNClient, TURNClientStub};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

#[tokio::test]
async fn test_turn_client_stub_creation() {
    let stub = TURNClientStub::new(None);
    let client = TURNClient::new(Arc::new(stub));
    let result = client.allocate_relay("turn:example.com:3478").await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[tokio::test]
async fn test_turn_client_stub_returns_allocation() {
    let allocation = RelayAllocation {
        relay_address: IpAddr::V4(Ipv4Addr::new(203, 0, 113, 10)),
        relay_port: 49152,
    };
    let stub = TURNClientStub::new(Some(allocation.clone()));
    let client = TURNClient::new(Arc::new(stub));
    let result = client.allocate_relay("turn:asgard.local:3478").await.unwrap();
    let got = result.as_ref().unwrap();
    assert_eq!(got, &allocation);
    assert_eq!(got.relay_port, 49152);
}

#[tokio::test]
async fn test_turn_client_stub_allocation_values() {
    let allocation = RelayAllocation {
        relay_address: "192.0.2.1".parse().unwrap(),
        relay_port: 50000,
    };
    let stub = TURNClientStub::new(Some(allocation));
    let client = TURNClient::new(Arc::new(stub));
    let got = client.allocate_relay("turn:yggdrasil.example:3478").await.unwrap();
    let got = got.unwrap();
    assert_eq!(got.relay_address, "192.0.2.1".parse::<IpAddr>().unwrap());
    assert_eq!(got.relay_port, 50000);
}
