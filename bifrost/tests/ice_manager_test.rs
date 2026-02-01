//! Integration tests for ICE Manager (Phase 13.3 – NAT Traversal).
//!
//! - Candidate gathering (Host, Server-Reflexive, Relayed)
//! - Best-path selection
//! Uses ICEManagerStub for container-friendly tests.

use bifrost::nat::{
    IceCandidate, IceCandidateKind, ICEManager, ICEManagerStub, SelectedPath,
};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

fn host_candidate(addr: &str, port: u16) -> IceCandidate {
    IceCandidate {
        kind: IceCandidateKind::Host,
        address: addr.parse().unwrap(),
        port,
    }
}

fn relayed_candidate(addr: &str, port: u16) -> IceCandidate {
    IceCandidate {
        kind: IceCandidateKind::Relayed,
        address: addr.parse().unwrap(),
        port,
    }
}

#[tokio::test]
async fn test_ice_manager_stub_gather_empty() {
    let stub = ICEManagerStub::new(vec![], None);
    let manager = ICEManager::new(Arc::new(stub));
    let candidates = manager.gather_candidates().await.unwrap();
    assert!(candidates.is_empty());
}

#[tokio::test]
async fn test_ice_manager_stub_gather_returns_configured() {
    let candidates = vec![
        host_candidate("192.168.1.10", 9000),
        relayed_candidate("203.0.113.1", 49152),
    ];
    let stub = ICEManagerStub::new(candidates.clone(), None);
    let manager = ICEManager::new(Arc::new(stub));
    let got = manager.gather_candidates().await.unwrap();
    assert_eq!(got.len(), 2);
    assert_eq!(got[0].kind, IceCandidateKind::Host);
    assert_eq!(got[0].port, 9000);
    assert_eq!(got[1].kind, IceCandidateKind::Relayed);
}

#[tokio::test]
async fn test_ice_manager_stub_select_best_path() {
    let candidates = vec![host_candidate("192.168.1.10", 9000)];
    let best = SelectedPath {
        address: "192.168.1.10".parse::<IpAddr>().unwrap(),
        port: 9000,
        is_relay: false,
    };
    let stub = ICEManagerStub::new(candidates, Some(best.clone()));
    let manager = ICEManager::new(Arc::new(stub));
    let gathered = manager.gather_candidates().await.unwrap();
    let selected = manager.select_best_path(&gathered).await.unwrap();
    assert!(selected.is_some());
    let s = selected.unwrap();
    assert_eq!(s.address, best.address);
    assert_eq!(s.port, 9000);
    assert!(!s.is_relay);
}

#[tokio::test]
async fn test_ice_manager_stub_select_relay_path() {
    let candidates = vec![
        host_candidate("192.168.1.10", 9000),
        relayed_candidate("203.0.113.1", 49152),
    ];
    let best = SelectedPath {
        address: IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)),
        port: 49152,
        is_relay: true,
    };
    let stub = ICEManagerStub::new(candidates, Some(best));
    let manager = ICEManager::new(Arc::new(stub));
    let gathered = manager.gather_candidates().await.unwrap();
    let selected = manager.select_best_path(&gathered).await.unwrap();
    assert!(selected.is_some());
    assert!(selected.unwrap().is_relay);
}
