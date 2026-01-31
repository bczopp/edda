use crate::connection::ConnectionManager;
use crate::message::{BifrostMessage, MessageHandler};
use crate::routing::RetryManager;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, error};
use tokio_tungstenite::tungstenite::Message;
use futures_util::SinkExt;

/// Retry configuration for routing. When set, transient failures (target not connected, send failure) are retried with exponential backoff.
#[derive(Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 5,
            base_delay: Duration::from_secs(1),
        }
    }
}

pub struct MessageRouter {
    connection_manager: Arc<ConnectionManager>,
    retry_config: Option<RetryConfig>,
}

impl MessageRouter {
    pub fn new(connection_manager: Arc<ConnectionManager>) -> Self {
        Self {
            connection_manager,
            retry_config: None,
        }
    }

    /// Enable retry with exponential backoff for routing failures.
    pub fn with_retry(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }

    pub async fn route_message(&self, message: BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let (max_retries, base_delay) = match &self.retry_config {
            Some(c) => (c.max_retries, c.base_delay),
            None => (0, Duration::ZERO),
        };
        let mut retry = RetryManager::new(max_retries, base_delay);

        loop {
            match self.try_route_one(&message).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    let delay = retry.next_delay();
                    retry.record_attempt();
                    if !retry.should_retry() {
                        return Err(e);
                    }
                    if delay > Duration::ZERO {
                        tokio::time::sleep(delay).await;
                    }
                }
            }
        }
    }

    /// Single attempt: resolve target connections and send. Returns Err on target not connected or when all sends fail.
    async fn try_route_one(&self, message: &BifrostMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "Routing message {} from {} to {}",
            message.message_id, message.source_device_id, message.target_device_id
        );

        let connection_ids = self.connection_manager.list_by_device(&message.target_device_id).await;
        if connection_ids.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Target device {} not connected", message.target_device_id),
            )
            .into());
        }

        let message_json = MessageHandler::serialize_message(message)?;
        let ws_message = Message::Text(message_json);

        let mut any_sent = false;
        for connection_id in &connection_ids {
            if let Some(connection) = self.connection_manager.get(connection_id).await {
                let mut conn = connection.write().await;
                match conn.stream.send(ws_message.clone()).await {
                    Ok(()) => any_sent = true,
                    Err(e) => error!("Failed to send message to connection {}: {}", connection_id, e),
                }
            }
        }

        if !any_sent {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionRefused,
                format!("Failed to send to any connection for device {}", message.target_device_id),
            )
            .into());
        }

        info!(
            "Message routed to device {} ({} connections)",
            message.target_device_id,
            connection_ids.len()
        );
        Ok(())
    }
}
