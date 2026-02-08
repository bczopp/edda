//! TURN Client for Relay Allocation (Phase 13.2).
//!
//! Provides relay allocation for NAT traversal. Uses a provider trait so tests
//! can use a stub; production can use a real TURN library.

use anyhow::Result;
use async_trait::async_trait;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::info;

/// Result of a TURN relay allocation (relay address and port).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayAllocation {
    pub relay_address: IpAddr,
    pub relay_port: u16,
}

/// Provider trait for TURN client (enables stub in tests / real impl in production).
#[async_trait]
pub trait TURNClientProvider: Send + Sync {
    /// Allocate a relay on the given TURN server (e.g. "turn:asgard.local:3478").
    /// Returns None if allocation is not available (stub or server unreachable).
    async fn allocate_relay(&self, server_url: &str) -> Result<Option<RelayAllocation>>;
}

/// TURN client wrapper.
pub struct TURNClient {
    provider: Arc<dyn TURNClientProvider>,
}

impl TURNClient {
    pub fn new(provider: Arc<dyn TURNClientProvider>) -> Self {
        info!("TURNClient created");
        Self { provider }
    }

    pub async fn allocate_relay(&self, server_url: &str) -> Result<Option<RelayAllocation>> {
        self.provider.allocate_relay(server_url).await
    }
}

/// Stub TURN provider for tests and environments without TURN server.
pub struct TURNClientStub {
    allocation: Option<RelayAllocation>,
}

impl TURNClientStub {
    pub fn new(allocation: Option<RelayAllocation>) -> Self {
        info!(?allocation, "TURNClientStub created");
        Self { allocation }
    }
}

#[async_trait]
impl TURNClientProvider for TURNClientStub {
    async fn allocate_relay(&self, _server_url: &str) -> Result<Option<RelayAllocation>> {
        Ok(self.allocation.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_stub_no_allocation() {
        let stub = TURNClientStub::new(None);
        let r = stub.allocate_relay("turn:test:3478").await.unwrap();
        assert!(r.is_none());
    }

    #[tokio::test]
    async fn test_stub_with_allocation() {
        let alloc = RelayAllocation {
            relay_address: IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)),
            relay_port: 49152,
        };
        let stub = TURNClientStub::new(Some(alloc.clone()));
        let r = stub.allocate_relay("turn:test:3478").await.unwrap();
        assert_eq!(r, Some(alloc));
    }

    #[tokio::test]
    async fn test_client_wraps_stub() {
        let alloc = RelayAllocation {
            relay_address: "203.0.113.1".parse().unwrap(),
            relay_port: 50000,
        };
        let stub = Arc::new(TURNClientStub::new(Some(alloc)));
        let client = TURNClient::new(stub);
        let r = client.allocate_relay("turn:server:3478").await.unwrap();
        assert_eq!(r.unwrap().relay_port, 50000);
    }
}
