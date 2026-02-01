use std::time::Duration;
use tokio::time::sleep;

/// Wait for a service to be ready
pub async fn wait_for_service(url: &str, max_retries: u32) -> bool {
    for _ in 0..max_retries {
        if tokio::net::TcpStream::connect(url).await.is_ok() {
            return true;
        }
        sleep(Duration::from_millis(500)).await;
    }
    false
}

/// Get service URL from environment or use default
pub fn get_service_url(service_name: &str, default_port: u16) -> String {
    let env_var = format!("{}_URL", service_name.to_uppercase());
    std::env::var(&env_var).unwrap_or_else(|_| {
        format!("http://localhost:{}", default_port)
    })
}

/// WebSocket test client helper (Phase 1.2.2). Connects to `url` for use in integration tests.
/// Uses Bifrost's WebSocketClient; returns the stream and response for send/receive tests.
pub async fn connect_websocket_test_client(
    url: &str,
) -> Result<
    (
        tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::handshake::client::Response,
    ),
    Box<dyn std::error::Error + Send + Sync>,
> {
    let client = bifrost::websocket::WebSocketClient::new();
    client.connect(url).await
}
