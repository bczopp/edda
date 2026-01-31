use crate::connection::ConnectionManager;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EndpointError {
    #[error("Endpoint handling failed: {0}")]
    HandlingFailed(String),
}

pub struct EndpointHandler {
    connection_manager: Arc<ConnectionManager>,
}

impl EndpointHandler {
    pub fn new(connection_manager: Arc<ConnectionManager>) -> Self {
        Self { connection_manager }
    }

    pub async fn handle_connection(&self, device_id: &str, endpoint_type: &str) -> Result<String, EndpointError> {
        use chrono::{Utc, Duration as ChronoDuration};
        let expires_at = Utc::now() + ChronoDuration::hours(24);
        let (connection_id, _) = self.connection_manager.register_connection(
            device_id,
            "system", // user_id for gRPC connections
            expires_at,
        ).await;
        Ok(connection_id)
    }
}
