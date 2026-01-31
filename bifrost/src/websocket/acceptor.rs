//! Connection Acceptor (Phase 6.1.2). Accept incoming TCP; optional TLS; WebSocket upgrade.

use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::WebSocketStream;

/// Accepts incoming TCP connection and performs WebSocket upgrade. TLS handshake is optional (not yet implemented).
pub struct ConnectionAcceptor;

impl ConnectionAcceptor {
    pub fn new() -> Self {
        Self
    }

    /// Performs WebSocket upgrade on the TCP stream. TLS handshake would be done before this when TLS is enabled.
    pub async fn accept(
        &self,
        stream: TcpStream,
    ) -> Result<WebSocketStream<TcpStream>, Box<dyn std::error::Error + Send + Sync>> {
        let ws_stream = accept_async(stream).await?;
        Ok(ws_stream)
    }
}

impl Default for ConnectionAcceptor {
    fn default() -> Self {
        Self::new()
    }
}
