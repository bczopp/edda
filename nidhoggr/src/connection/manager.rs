use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct Connection {
    pub connection_id: String,
    pub session_id: String,
    pub device_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}

pub struct ConnectionManager {
    connections: Arc<RwLock<HashMap<String, Connection>>>,
    sessions: Arc<RwLock<HashMap<String, String>>>, // session_id -> connection_id
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_connection(
        &self,
        device_id: &str,
        user_id: &str,
        expires_at: DateTime<Utc>,
    ) -> (String, String) {
        let connection_id = Uuid::new_v4().to_string();
        let session_id = Uuid::new_v4().to_string();
        
        let connection = Connection {
            connection_id: connection_id.clone(),
            session_id: session_id.clone(),
            device_id: device_id.to_string(),
            user_id: user_id.to_string(),
            created_at: Utc::now(),
            expires_at,
            last_heartbeat: Utc::now(),
        };
        
        self.connections.write().await.insert(connection_id.clone(), connection);
        self.sessions.write().await.insert(session_id.clone(), connection_id.clone());
        
        (connection_id, session_id)
    }

    pub async fn get_connection(&self, connection_id: &str) -> Option<Connection> {
        self.connections.read().await.get(connection_id).cloned()
    }

    pub async fn get_connection_by_session(&self, session_id: &str) -> Option<Connection> {
        let sessions = self.sessions.read().await;
        if let Some(connection_id) = sessions.get(session_id) {
            self.connections.read().await.get(connection_id).cloned()
        } else {
            None
        }
    }

    pub async fn remove_connection(&self, connection_id: &str) {
        if let Some(connection) = self.connections.read().await.get(connection_id).cloned() {
            self.sessions.write().await.remove(&connection.session_id);
        }
        self.connections.write().await.remove(connection_id);
    }

    pub async fn remove_connection_by_session(&self, session_id: &str) {
        if let Some(connection_id) = self.sessions.read().await.get(session_id).cloned() {
            self.remove_connection(&connection_id).await;
        }
    }

    pub async fn update_heartbeat(&self, session_id: &str) -> bool {
        let sessions = self.sessions.read().await;
        if let Some(connection_id) = sessions.get(session_id) {
            let mut connections = self.connections.write().await;
            if let Some(connection) = connections.get_mut(connection_id) {
                connection.last_heartbeat = Utc::now();
                return true;
            }
        }
        false
    }

    pub async fn get_active_connections(&self) -> usize {
        self.connections.read().await.len()
    }

    pub async fn cleanup_expired(&self) {
        let now = Utc::now();
        let mut connections = self.connections.write().await;
        let mut sessions = self.sessions.write().await;
        
        let expired: Vec<String> = connections
            .iter()
            .filter(|(_, conn)| conn.expires_at < now)
            .map(|(id, _)| id.clone())
            .collect();
        
        for connection_id in expired {
            if let Some(connection) = connections.get(&connection_id) {
                sessions.remove(&connection.session_id);
            }
            connections.remove(&connection_id);
        }
    }

    pub async fn cleanup_stale_heartbeats(&self, max_idle_seconds: i64) {
        let now = Utc::now();
        let mut connections = self.connections.write().await;
        let mut sessions = self.sessions.write().await;
        
        let stale: Vec<String> = connections
            .iter()
            .filter(|(_, conn)| {
                let idle_seconds = (now - conn.last_heartbeat).num_seconds();
                idle_seconds > max_idle_seconds
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        for connection_id in stale {
            if let Some(connection) = connections.get(&connection_id) {
                sessions.remove(&connection.session_id);
            }
            connections.remove(&connection_id);
        }
    }
}
