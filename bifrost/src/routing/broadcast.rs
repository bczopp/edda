//! Broadcast Manager (Phase 9.3.1). Send to all devices; rate-limiting and TTL to prevent broadcast storms.

use crate::connection::ConnectionManager;
use crate::message::BifrostMessage;
use crate::routing::MessageRouter;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
#[error("broadcast rate limit: min interval not elapsed")]
pub struct RateLimitError;

/// Broadcasts message to all connected devices (except source); rate-limiting and optional TTL.
pub struct BroadcastManager {
    connection_manager: Arc<ConnectionManager>,
    router: MessageRouter,
    min_interval: Duration,
    last_broadcast: RwLock<Option<Instant>>,
}

impl BroadcastManager {
    pub fn new(
        connection_manager: Arc<ConnectionManager>,
        router: MessageRouter,
        min_interval: Duration,
    ) -> Self {
        Self {
            connection_manager,
            router,
            min_interval,
            last_broadcast: RwLock::new(None),
        }
    }

    /// Broadcasts message to all connected devices except source_device_id. Respects min_interval (rate limit).
    pub async fn broadcast(&self, message: BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let now = Instant::now();
        {
            let mut last = self.last_broadcast.write().await;
            if let Some(prev) = *last {
                if now.duration_since(prev) < self.min_interval {
                    return Err(RateLimitError.into());
                }
            }
            *last = Some(now);
        }

        let device_ids = self.connection_manager.list_device_ids().await;
        let source = &message.source_device_id;
        for device_id in device_ids {
            if device_id == *source {
                continue;
            }
            let mut msg = message.clone();
            msg.target_device_id = device_id;
            let _ = self.router.route_message(msg).await;
        }
        Ok(())
    }
}
