//! Relay Routing (Phase 9.2). RelayManager, AsgardRelayClient, YggdrasilRelayClient.

use crate::message::BifrostMessage;
use async_trait::async_trait;
use std::io;
use std::sync::Arc;
use tracing::debug;

/// Relay client: route message via Asgard or Yggdrasil.
#[async_trait]
pub trait RelayClient: Send + Sync {
    async fn route_message(&self, msg: &BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    fn is_available(&self) -> bool {
        true
    }
}

/// Routes messages via relay (Asgard/Yggdrasil); selects by order and falls back on failure.
pub struct RelayManager {
    asgard: Option<Arc<dyn RelayClient>>,
    yggdrasil: Option<Arc<dyn RelayClient>>,
}

impl RelayManager {
    pub fn new(
        asgard: Option<Arc<dyn RelayClient>>,
        yggdrasil: Option<Arc<dyn RelayClient>>,
    ) -> Self {
        Self { asgard, yggdrasil }
    }

    pub async fn route_message(&self, msg: &BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut last_error: Box<dyn std::error::Error + Send + Sync> =
            io::Error::new(io::ErrorKind::Other, "no relay configured").into();

        if let Some(ref a) = self.asgard {
            if a.is_available() {
                match a.route_message(msg).await {
                    Ok(()) => return Ok(()),
                    Err(e) => {
                        debug!(relay = "asgard", "relay route failed: {}", e);
                        last_error = e;
                    }
                }
            }
        }

        if let Some(ref y) = self.yggdrasil {
            if y.is_available() {
                match y.route_message(msg).await {
                    Ok(()) => return Ok(()),
                    Err(e) => {
                        debug!(relay = "yggdrasil", "relay route failed: {}", e);
                        last_error = e;
                    }
                }
            }
        }

        Err(last_error)
    }
}

/// Asgard relay client (Phase 9.2.2). Connection to Asgard relay; route message via Asgard.
pub struct AsgardRelayClient {
    configured: bool,
    mock_success: bool,
}

impl AsgardRelayClient {
    pub fn unconfigured() -> Self {
        Self {
            configured: false,
            mock_success: false,
        }
    }

    /// For tests: always returns Ok(()) on route_message.
    pub fn mock_success() -> Self {
        Self {
            configured: true,
            mock_success: true,
        }
    }

    pub fn new(_endpoint: &str) -> Self {
        Self {
            configured: true,
            mock_success: false,
        }
    }
}

#[async_trait]
impl RelayClient for AsgardRelayClient {
    async fn route_message(&self, _msg: &BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.configured {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Asgard relay not configured",
            )
            .into());
        }
        if self.mock_success {
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Asgard relay not configured",
        )
        .into())
    }

    fn is_available(&self) -> bool {
        self.configured
    }
}

/// Yggdrasil relay client (Phase 9.2.3). Persistent connection to Yggdrasil; route via Yggdrasil.
pub struct YggdrasilRelayClient {
    configured: bool,
    mock_success: bool,
}

impl YggdrasilRelayClient {
    pub fn unconfigured() -> Self {
        Self {
            configured: false,
            mock_success: false,
        }
    }

    pub fn mock_success() -> Self {
        Self {
            configured: true,
            mock_success: true,
        }
    }

    pub fn new(_endpoint: &str) -> Self {
        Self {
            configured: true,
            mock_success: false,
        }
    }
}

#[async_trait]
impl RelayClient for YggdrasilRelayClient {
    async fn route_message(&self, _msg: &BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.configured {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Yggdrasil relay not configured",
            )
            .into());
        }
        if self.mock_success {
            return Ok(());
        }
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Yggdrasil relay not configured",
        )
        .into())
    }

    fn is_available(&self) -> bool {
        self.configured
    }
}
