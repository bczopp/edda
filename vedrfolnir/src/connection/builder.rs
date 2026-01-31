use crate::ratatoskr::RatatoskrClient;
use crate::ratatoskr::RatatoskrConnection;
use crate::ratatoskr::RatatoskrError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Connection building failed: {0}")]
    BuildingFailed(String),
    #[error("Ratatoskr error: {0}")]
    RatatoskrError(#[from] RatatoskrError),
}

pub struct ConnectionBuilder {
    yggdrasil_url: String,
}

impl ConnectionBuilder {
    pub fn new(yggdrasil_url: String) -> Self {
        Self {
            yggdrasil_url,
        }
    }

    pub async fn build_connection(
        &self,
        device_id: String,
        user_id: String,
        device_identity: String,
        authentication_token: String,
    ) -> Result<RatatoskrConnection, ConnectionError> {
        let mut client = RatatoskrClient::new(
            self.yggdrasil_url.clone(),
            device_id,
            user_id,
            device_identity,
            authentication_token,
        );
        
        client.connect().await
            .map_err(|e| ConnectionError::BuildingFailed(format!("Failed to connect: {}", e)))
    }
}
