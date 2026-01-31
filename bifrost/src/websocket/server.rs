use crate::connection::ConnectionManager;
use crate::message::MessageHandler;
use crate::mesh::{FloodRouter, transport};
use crate::routing::MessageRouter;
use crate::websocket::HeartbeatManager;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tracing::{error, info};
use uuid::Uuid;

/// Heartbeat send interval (server sends Ping).
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
/// Connection timeout when no Pong received.
const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(45);
/// Tick interval for checking heartbeat (1s).
const HEARTBEAT_TICK: Duration = Duration::from_secs(1);

#[derive(Clone)]
pub struct WebSocketServer {
    port: u16,
    connection_manager: Arc<ConnectionManager>,
    router: Arc<MessageRouter>,
}

impl WebSocketServer {
    /// Exposes the connection manager (e.g. for tests).
    pub fn connection_manager(&self) -> &Arc<ConnectionManager> {
        &self.connection_manager
    }

    pub fn new(port: u16) -> Self {
        let connection_manager = Arc::new(ConnectionManager::new());
        let router = Arc::new(MessageRouter::new(connection_manager.clone()));
        Self { 
            port,
            connection_manager,
            router,
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;
        info!("Bifrost WebSocket server listening on {}", addr);
        self.run_listener(listener).await
    }

    /// Runs the accept loop on an existing listener (for tests with ephemeral port).
    pub async fn run_listener(
        &self,
        listener: TcpListener,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let server = self.clone();
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream, addr).await {
                            error!("Connection error: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }

    fn extract_device_id_from_handshake(&self, _stream: &TcpStream) -> Option<String> {
        // In a real implementation, would extract from WebSocket headers or query params
        // For now, return None (will use "unknown" as fallback)
        None
    }

    fn extract_user_id_from_handshake(&self, _stream: &TcpStream) -> Option<String> {
        // In a real implementation, would extract from WebSocket headers or query params
        // For now, return None (will use "unknown" as fallback)
        None
    }

    async fn handle_connection(
        &self,
        stream: TcpStream,
        addr: std::net::SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Extract device_id and user_id from handshake before stream is consumed by accept_async
        let device_id = self.extract_device_id_from_handshake(&stream).unwrap_or_else(|| "unknown".to_string());
        let user_id = self.extract_user_id_from_handshake(&stream).unwrap_or_else(|| "unknown".to_string());

        let ws_stream = accept_async(stream).await?;
        info!("New WebSocket connection from {}", addr);

        let connection_id = Uuid::new_v4().to_string();

        let (write, mut read) = ws_stream.split();
        self.connection_manager
            .register(connection_id.clone(), device_id, user_id, write)
            .await;

        let mut heartbeat = HeartbeatManager::new(HEARTBEAT_INTERVAL, HEARTBEAT_TIMEOUT);

        let result = loop {
            tokio::select! {
                msg = read.next() => {
                    let Some(msg) = msg else { break Ok(()) };
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Ok(packet) = transport::decode_mesh_packet(&text) {
                                let flood = FloodRouter::new();
                                const MY_NODE_ID: u32 = 0;
                                if flood.should_forward(&packet, MY_NODE_ID) && packet.hop_limit > 0 {
                                    let mut fwd = packet.clone();
                                    fwd.hop_limit = packet.hop_limit.saturating_sub(1);
                                    if let Ok(encoded) = transport::encode_mesh_packet(&fwd) {
                                        let ids = self.connection_manager.list_connection_ids().await;
                                        for id in ids {
                                            if id != connection_id {
                                                if let Some(conn) = self.connection_manager.get(&id).await {
                                                    let mut c = conn.write().await;
                                                    let _ = c.stream.send(Message::Text(encoded.clone())).await;
                                                }
                                            }
                                        }
                                    }
                                }
                            } else if let Ok(message) = MessageHandler::parse_message(&text) {
                                if let Err(e) = self.router.route_message(message).await {
                                    error!("Failed to route message: {}", e);
                                }
                            }
                        }
                        Ok(Message::Close(_)) => {
                            info!("Connection closed");
                            break Ok(());
                        }
                        Ok(Message::Ping(data)) => {
                            if let Some(conn) = self.connection_manager.get(&connection_id).await {
                                let mut c = conn.write().await;
                                let _ = c.stream.send(Message::Pong(data)).await;
                            }
                            heartbeat.record_received();
                        }
                        Ok(Message::Pong(_)) => {
                            heartbeat.record_received();
                        }
                        Err(e) => {
                            error!("WebSocket error: {}", e);
                            break Err(e.into());
                        }
                        _ => {}
                    }
                }
                _ = sleep(HEARTBEAT_TICK) => {
                    if heartbeat.should_timeout() {
                        info!("Connection heartbeat timeout from {}", addr);
                        break Ok(());
                    }
                    if heartbeat.should_send_heartbeat() {
                        if let Some(conn) = self.connection_manager.get(&connection_id).await {
                            let mut c = conn.write().await;
                            if c.stream.send(Message::Ping(vec![])).await.is_err() {
                                break Ok(());
                            }
                        }
                        heartbeat.record_sent();
                    }
                }
            }
        };

        self.connection_manager.remove(&connection_id).await;
        result
    }
}
