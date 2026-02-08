//! ICE Manager for optimal path selection (Phase 13.3).
//!
//! Candidate gathering (Host, Server-Reflexive, Relayed) and best-path selection.
//! Uses a provider trait so tests can use a stub; production can use a real ICE implementation.

use anyhow::Result;
use async_trait::async_trait;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::info;

/// ICE candidate type (Host, Server-Reflexive from STUN, Relayed from TURN).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IceCandidateKind {
    Host,
    ServerReflexive,
    Relayed,
}

/// A single ICE candidate (address and port).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IceCandidate {
    pub kind: IceCandidateKind,
    pub address: IpAddr,
    pub port: u16,
}

/// The selected best path for connectivity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectedPath {
    pub address: IpAddr,
    pub port: u16,
    /// True if the path goes via a TURN relay.
    pub is_relay: bool,
}

/// Provider trait for ICE manager (enables stub in tests / real impl in production).
#[async_trait]
pub trait ICEManagerProvider: Send + Sync {
    /// Gather candidates (Host, Server-Reflexive, Relayed).
    async fn gather_candidates(&self) -> Result<Vec<IceCandidate>>;
    /// Select the best path from the given candidates (e.g. after connectivity checks).
    async fn select_best_path(&self, candidates: &[IceCandidate]) -> Result<Option<SelectedPath>>;
}

/// ICE manager wrapper.
pub struct ICEManager {
    provider: Arc<dyn ICEManagerProvider>,
}

impl ICEManager {
    pub fn new(provider: Arc<dyn ICEManagerProvider>) -> Self {
        info!("ICEManager created");
        Self { provider }
    }

    pub async fn gather_candidates(&self) -> Result<Vec<IceCandidate>> {
        self.provider.gather_candidates().await
    }

    pub async fn select_best_path(&self, candidates: &[IceCandidate]) -> Result<Option<SelectedPath>> {
        self.provider.select_best_path(candidates).await
    }
}

/// Stub ICE manager for tests and environments without STUN/TURN.
pub struct ICEManagerStub {
    candidates: Vec<IceCandidate>,
    best_path: Option<SelectedPath>,
}

impl ICEManagerStub {
    pub fn new(candidates: Vec<IceCandidate>, best_path: Option<SelectedPath>) -> Self {
        info!(count = candidates.len(), "ICEManagerStub created");
        Self {
            candidates,
            best_path,
        }
    }
}

#[async_trait]
impl ICEManagerProvider for ICEManagerStub {
    async fn gather_candidates(&self) -> Result<Vec<IceCandidate>> {
        Ok(self.candidates.clone())
    }

    async fn select_best_path(&self, _candidates: &[IceCandidate]) -> Result<Option<SelectedPath>> {
        Ok(self.best_path.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_stub_gather_empty() {
        let stub = ICEManagerStub::new(vec![], None);
        let c = stub.gather_candidates().await.unwrap();
        assert!(c.is_empty());
    }

    #[tokio::test]
    async fn test_stub_select_none() {
        let stub = ICEManagerStub::new(
            vec![IceCandidate {
                kind: IceCandidateKind::Host,
                address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
                port: 9000,
            }],
            None,
        );
        let candidates = stub.gather_candidates().await.unwrap();
        let best = stub.select_best_path(&candidates).await.unwrap();
        assert!(best.is_none());
    }

    #[tokio::test]
    async fn test_manager_wraps_stub() {
        let path = SelectedPath {
            address: "192.168.1.10".parse().unwrap(),
            port: 9000,
            is_relay: false,
        };
        let stub = Arc::new(ICEManagerStub::new(vec![], Some(path)));
        let manager = ICEManager::new(stub);
        let best = manager.select_best_path(&[]).await.unwrap();
        assert_eq!(best.unwrap().port, 9000);
    }
}
