//! WebSocket Client Core (Phase 7.1.1). Initiate connection; optional TLS; WebSocket upgrade.

use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::handshake::client::Response,
    WebSocketStream,
};
use tokio_tungstenite::MaybeTlsStream;

/// WebSocket client: initiates connection and performs WebSocket upgrade. TLS (wss://) when enabled.
///
/// # Example
///
/// ```no_run
/// use bifrost::websocket::WebSocketClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let client = WebSocketClient::new();
///     let (mut stream, response) = client.connect("ws://127.0.0.1:8080").await?;
///     assert!(response.status().is_success());
///     // use stream for send/receive
///     Ok(())
/// }
/// ```
pub struct WebSocketClient;

impl WebSocketClient {
    pub fn new() -> Self {
        Self
    }

    /// Connects to the given URL (ws:// or wss://) and performs WebSocket upgrade. TLS handshake for wss:// when TLS is enabled.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bifrost::websocket::WebSocketClient;
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let client = WebSocketClient::new();
    /// let (stream, _) = client.connect("ws://localhost:8080").await?;
    /// # Ok(())
    /// # }
    /// ```
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
