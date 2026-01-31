//! WebSocket Client Core (Phase 7.1.1). Initiate connection; optional TLS; WebSocket upgrade.

use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::handshake::client::Response,
    WebSocketStream,
};
use tokio_tungstenite::MaybeTlsStream;

/// WebSocket client: initiates connection and performs WebSocket upgrade. TLS (wss://) when enabled.
pub struct WebSocketClient;

impl WebSocketClient {
    pub fn new() -> Self {
        Self
    }

    /// Connects to the given URL (ws:// or wss://) and performs WebSocket upgrade. TLS handshake for wss:// when TLS is enabled.
    pub async fn connect(
        &self,
        url: &str,
    ) -> Result<
        (
            WebSocketStream<MaybeTlsStream<TcpStream>>,
            Response,
        ),
        Box<dyn std::error::Error + Send + Sync>,
    > {
        let (ws_stream, response) = connect_async(url).await?;
        Ok((ws_stream, response))
    }
}

impl Default for WebSocketClient {
    fn default() -> Self {
        Self::new()
    }
}
