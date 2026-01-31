use crate::ratatoskr::RatatoskrConnection;
use crate::ratatoskr::RatatoskrError;
use crate::connection::builder::ConnectionBuilder;
use crate::retry::exponential_backoff::{ExponentialBackoff, RetryError};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{info, error, warn};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionManagerError {
    #[error("Connection error: {0}")]
    ConnectionError(#[from] RatatoskrError),
    #[error("Connection not established")]
    NotConnected,
}

pub struct ConnectionManager {
    connection: Arc<RwLock<Option<RatatoskrConnection>>>,
    heartbeat_interval: Duration,
    connection_builder: Option<Arc<ConnectionBuilder>>,
    device_id: Option<String>,
    user_id: Option<String>,
    device_identity: Option<String>,
    auth_token: Option<String>,
    auto_reconnect: bool,
    retry_backoff: ExponentialBackoff,
}

impl ConnectionManager {
    pub fn new(heartbeat_interval_secs: u64) -> Self {
        Self {
            connection: Arc::new(RwLock::new(None)),
            heartbeat_interval: Duration::from_secs(heartbeat_interval_secs),
            connection_builder: None,
            device_id: None,
            user_id: None,
            device_identity: None,
            auth_token: None,
            auto_reconnect: false,
            retry_backoff: ExponentialBackoff::new(1000, 30000, 2.0, 5), // 1s initial, 30s max, 5 retries
        }
    }

    pub fn with_auto_reconnect(
        heartbeat_interval_secs: u64,
        connection_builder: Arc<ConnectionBuilder>,
        device_id: String,
        user_id: String,
        device_identity: String,
        auth_token: String,
    ) -> Self {
        Self {
            connection: Arc::new(RwLock::new(None)),
            heartbeat_interval: Duration::from_secs(heartbeat_interval_secs),
            connection_builder: Some(connection_builder),
            device_id: Some(device_id),
            user_id: Some(user_id),
            device_identity: Some(device_identity),
            auth_token: Some(auth_token),
            auto_reconnect: true,
            retry_backoff: ExponentialBackoff::new(1000, 30000, 2.0, 5),
        }
    }

    pub async fn set_connection(&self, connection: RatatoskrConnection) {
        *self.connection.write().await = Some(connection);
    }

    pub async fn get_connection(&self) -> Option<Arc<RwLock<Option<RatatoskrConnection>>>> {
        Some(self.connection.clone())
    }

    pub async fn is_connected(&self) -> bool {
        self.connection.read().await.is_some()
    }

    pub async fn start_heartbeat(&self) {
        let connection = self.connection.clone();
        let interval_duration = self.heartbeat_interval;
        let auto_reconnect = self.auto_reconnect;
        let connection_builder = self.connection_builder.clone();
        let device_id = self.device_id.clone();
        let user_id = self.user_id.clone();
        let device_identity = self.device_identity.clone();
        let auth_token = self.auth_token.clone();
        let retry_backoff = self.retry_backoff.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(interval_duration);
            loop {
                interval.tick().await;
                
                let conn_guard = connection.read().await;
                if let Some(ref conn) = *conn_guard {
                    match conn.send_heartbeat().await {
                        Ok(_) => {
                            info!("Heartbeat sent successfully");
                        }
                        Err(e) => {
                            error!("Heartbeat failed: {}", e);
                            // Connection might be dead, clear it
                            drop(conn_guard);
                            *connection.write().await = None;
                            
                            // Auto-reconnect if enabled
                            if auto_reconnect {
                                if let (Some(builder), Some(device_id), Some(user_id), 
                                        Some(device_identity), Some(auth_token)) = 
                                    (connection_builder, device_id.clone(), user_id.clone(),
                                     device_identity.clone(), auth_token.clone()) {
                                    
                                    info!("Attempting to reconnect...");
                                    let reconnect_result = retry_backoff.execute(|| {
                                        Box::pin(async {
                                            builder.build_connection(
                                                device_id.clone(),
                                                user_id.clone(),
                                                device_identity.clone(),
                                                auth_token.clone(),
                                            ).await
                                            .map_err(|e| format!("Reconnection failed: {}", e))
                                        })
                                    }).await;
                                    
                                    match reconnect_result {
                                        Ok(new_conn) => {
                                            info!("Reconnection successful");
                                            *connection.write().await = Some(new_conn);
                                        }
                                        Err(_) => {
                                            error!("Reconnection failed after max retries");
                                            break;
                                        }
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                } else {
                    if auto_reconnect {
                        // Try to reconnect
                        if let (Some(builder), Some(device_id), Some(user_id), 
                                Some(device_identity), Some(auth_token)) = 
                            (connection_builder.clone(), device_id.clone(), user_id.clone(),
                             device_identity.clone(), auth_token.clone()) {
                            
                            info!("No connection, attempting to reconnect...");
                            let backoff = retry_backoff.clone();
                            let reconnect_result = backoff.execute(|| {
                                Box::pin(async {
                                    builder.build_connection(
                                        device_id.clone(),
                                        user_id.clone(),
                                        device_identity.clone(),
                                        auth_token.clone(),
                                    ).await
                                    .map_err(|e| format!("Reconnection failed: {}", e))
                                })
                            }).await;
                            
                            match reconnect_result {
                                Ok(new_conn) => {
                                    info!("Reconnection successful");
                                    *connection.write().await = Some(new_conn);
                                }
                                Err(_) => {
                                    warn!("Reconnection failed, will retry on next heartbeat");
                                }
                            }
                        }
                    } else {
                        warn!("No connection available for heartbeat");
                        break;
                    }
                }
            }
        });
    }

    pub async fn send_business_request(
        &self,
        payload: Vec<u8>,
    ) -> Result<ratatoskr::messages::RatatoskrResponse, ConnectionManagerError> {
        let conn_guard = self.connection.read().await;
        let conn = conn_guard.as_ref()
            .ok_or(ConnectionManagerError::NotConnected)?;
        
        conn.send_business_request(payload).await
            .map_err(ConnectionManagerError::ConnectionError)
    }

    pub async fn disconnect(&self) -> Result<(), ConnectionManagerError> {
        let mut conn_guard = self.connection.write().await;
        if let Some(conn) = conn_guard.take() {
            conn.disconnect().await
                .map_err(ConnectionManagerError::ConnectionError)?;
        }
        Ok(())
    }
}
