//! STUN Client for NAT Discovery (Phase 13.1).
//!
//! Provides public IP and NAT type. Uses a provider trait so tests can use
//! a stub; production can use a real STUN library (e.g. webrtc-rs or stun-rs).

use anyhow::Result;
use async_trait::async_trait;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::info;

/// NAT type as discovered by STUN (RFC 3489-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatType {
    Unknown,
    FullCone,
    RestrictedCone,
    PortRestrictedCone,
    Symmetric,
}

/// Provider trait for STUN client (enables stub in tests / real impl in production).
#[async_trait]
pub trait STUNClientProvider: Send + Sync {
    /// Get public IP (if discoverable) and NAT type.
    async fn get_public_ip_and_nat_type(&self) -> Result<(Option<IpAddr>, NatType)>;
}

/// STUN client wrapper.
pub struct STUNClient {
    provider: Arc<dyn STUNClientProvider>,
}

impl STUNClient {
    pub fn new(provider: Arc<dyn STUNClientProvider>) -> Self {
        info!("STUNClient created");
        Self { provider }
    }

    pub async fn get_public_ip_and_nat_type(&self) -> Result<(Option<IpAddr>, NatType)> {
        self.provider.get_public_ip_and_nat_type().await
    }
}

/// Stub STUN provider for tests and environments without STUN server.
pub struct STUNClientStub {
    public_ip: Option<IpAddr>,
    nat_type: NatType,
}

impl STUNClientStub {
    pub fn new(public_ip: Option<IpAddr>, nat_type: NatType) -> Self {
        info!(?public_ip, ?nat_type, "STUNClientStub created");
        Self { public_ip, nat_type }
    }
}

#[async_trait]
impl STUNClientProvider for STUNClientStub {
    async fn get_public_ip_and_nat_type(&self) -> Result<(Option<IpAddr>, NatType)> {
        Ok((self.public_ip, self.nat_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stub_unknown() {
        let stub = STUNClientStub::new(None, NatType::Unknown);
        let (ip, nt) = stub.get_public_ip_and_nat_type().await.unwrap();
        assert!(ip.is_none());
        assert_eq!(nt, NatType::Unknown);
    }

    #[tokio::test]
    async fn test_stub_with_ip() {
        let addr: IpAddr = "203.0.113.1".parse().unwrap();
        let stub = STUNClientStub::new(Some(addr), NatType::Symmetric);
        let (ip, nt) = stub.get_public_ip_and_nat_type().await.unwrap();
        assert_eq!(ip, Some(addr));
        assert_eq!(nt, NatType::Symmetric);
    }

    #[tokio::test]
    async fn test_client_wraps_stub() {
        let stub = Arc::new(STUNClientStub::new(None, NatType::FullCone));
        let client = STUNClient::new(stub);
        let (ip, nt) = client.get_public_ip_and_nat_type().await.unwrap();
        assert!(ip.is_none());
        assert_eq!(nt, NatType::FullCone);
    }
}
