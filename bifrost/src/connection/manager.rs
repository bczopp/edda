use futures_util::stream::SplitSink;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

/// Write-half of a WebSocket connection (for routing messages to the connection).
pub type WsWrite = SplitSink<WebSocketStream<TcpStream>, Message>;

pub struct Connection {
    pub device_id: String,
    pub user_id: String,
    pub stream: WsWrite,
}

pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<String, Arc<RwLock<Connection>>>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(
        &self,
        connection_id: String,
        device_id: String,
        user_id: String,
        stream: WsWrite,
    ) {
        let mut connections = self.connections.write().await;
        connections.insert(
            connection_id.clone(),
            Arc::new(RwLock::new(Connection {
                device_id,
                user_id,
                stream,
            })),
        );
    }

    pub async fn get(&self, connection_id: &str) -> Option<Arc<RwLock<Connection>>> {
        let connections = self.connections.read().await;
        connections.get(connection_id).cloned()
    }

    pub async fn remove(&self, connection_id: &str) {
        let mut connections = self.connections.write().await;
        connections.remove(connection_id);
    }

    pub async fn list_by_device(&self, device_id: &str) -> Vec<String> {
        let connections = self.connections.read().await;
        let mut result = Vec::new();
        for (id, conn) in connections.iter() {
            let conn_guard = conn.read().await;
            if conn_guard.device_id == device_id {
                result.push(id.clone());
            }
        }
        result
    }

    /// Returns all connection IDs (for mesh broadcast / IP transport).
    pub async fn list_connection_ids(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }

    /// Returns unique device IDs of all connected devices (for broadcast).
    pub async fn list_device_ids(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        let mut ids = HashSet::new();
        for conn in connections.values() {
            let guard = conn.read().await;
            ids.insert(guard.device_id.clone());
        }
        ids.into_iter().collect()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
