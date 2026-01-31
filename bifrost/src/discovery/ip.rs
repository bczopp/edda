//! IP-based Connection Manager (Phase 8.2.1). Connect to IP:port with WebSocket, timeout, retry.

use crate::websocket::{ReconnectionConfig, ReconnectionManager, WebSocketClient};
use std::time::Duration;
use tokio::time::timeout;

/// Builds ws:// URL from host and port.
pub fn build_ws_url(host: &str, port: u16) -> String {
    format!("ws://{}:{}", host, port)
}

/// Manages WebSocket connections to an IP address: connect with timeout and optional retry.
pub struct IpConnectionManager {
    connect_timeout: Duration,
    reconnection_config: Option<ReconnectionConfig>,
    client: WebSocketClient,
}

impl IpConnectionManager {
    pub fn new(
        connect_timeout: Duration,
        reconnection_config: Option<ReconnectionConfig>,
    ) -> Self {
        Self {
            connect_timeout,
            reconnection_config,
            client: WebSocketClient::new(),
        }
    }

    /// Connects to host:port via WebSocket. Fails after connect_timeout.
    pub async fn connect(
        &self,
        host: &str,
        port: u16,
    ) -> Result<
        (
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            tokio_tungstenite::tungstenite::handshake::client::Response,
        ),
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let url = build_ws_url(host, port);
        let result = timeout(self.connect_timeout, self.client.connect(&url)).await;
        match result {
            Ok(Ok(stream_and_response)) => Ok(stream_and_response),
            Ok(Err(e)) => Err(e),
            Err(_) => Err("connection timeout".into()),
        }
    }

    /// Connects with retry using exponential backoff. Stops after max_attempts failed attempts.
    pub async fn connect_with_retry(
        &self,
        host: &str,
        port: u16,
        max_attempts: u32,
    ) -> Result<
        (
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            tokio_tungstenite::tungstenite::handshake::client::Response,
        ),
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let mut reconnection = match &self.reconnection_config {
            Some(cfg) => ReconnectionManager::new(cfg.clone()),
            None => return self.connect(host, port).await,
        };
        let mut last_error: Box<dyn std::error::Error + Send + Sync> =
            "no attempts".into();
        for attempt in 0..max_attempts {
            if attempt > 0 {
                let delay = reconnection.next_delay();
                tokio::time::sleep(delay).await;
            }
            match self.connect(host, port).await {
                Ok(stream_and_response) => return Ok(stream_and_response),
                Err(e) => {
                    last_error = e;
                    reconnection.record_attempt();
                }
            }
        }
        Err(last_error)
    }
}
