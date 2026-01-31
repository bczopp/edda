//! Fallback Routing Manager (Phase 10.1.2): Direct → Asgard → Yggdrasil.

use crate::message::BifrostMessage;
use crate::routing::MessageRouter;
use std::io;
use tracing::debug;

/// Route in the fallback hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteKind {
    Direct,
    Asgard,
    Yggdrasil,
}

/// Stub for Asgard relay (Phase 9.2.2). Returns error until real client is implemented.
#[derive(Debug, Default)]
pub struct AsgardRelayStub;

impl AsgardRelayStub {
    pub async fn route_message(&self, _msg: &BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Asgard relay not configured",
        )
        .into())
    }
}

/// Stub for Yggdrasil relay (Phase 9.2.3). Returns error until real client is implemented.
#[derive(Debug, Default)]
pub struct YggdrasilRelayStub;

impl YggdrasilRelayStub {
    pub async fn route_message(&self, _msg: &BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Yggdrasil relay not configured",
        )
        .into())
    }
}

/// Tries routes in order: Direct → Asgard → Yggdrasil. On failure, tries next route.
pub struct FallbackRoutingManager {
    direct: MessageRouter,
    asgard: Option<AsgardRelayStub>,
    yggdrasil: Option<YggdrasilRelayStub>,
}

impl FallbackRoutingManager {
    pub fn new(
        direct: MessageRouter,
        asgard: Option<AsgardRelayStub>,
        yggdrasil: Option<YggdrasilRelayStub>,
    ) -> Self {
        Self {
            direct,
            asgard,
            yggdrasil,
        }
    }

    pub async fn route_message(&self, message: BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut last_error = match self.direct.route_message(message.clone()).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                debug!(route = ?RouteKind::Direct, "direct route failed: {}", e);
                e
            }
        };

        if let Some(ref a) = self.asgard {
            match a.route_message(&message).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    debug!(route = ?RouteKind::Asgard, "asgard route failed: {}", e);
                    last_error = e;
                }
            }
        }

        if let Some(ref y) = self.yggdrasil {
            match y.route_message(&message).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    debug!(route = ?RouteKind::Yggdrasil, "yggdrasil route failed: {}", e);
                    last_error = e;
                }
            }
        }

        Err(last_error)
    }
}
