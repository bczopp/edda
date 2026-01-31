use std::sync::Arc;
use tokio::sync::Mutex;
use crate::clients::{NornenClient, HeidrunClient, MimirClient};
use crate::utils::config::ServiceEndpoints;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientManagerError {
    #[error("Nornen client error: {0}")]
    NornenError(#[from] crate::clients::nornen::NornenClientError),
    #[error("Heidrun client error: {0}")]
    HeidrunError(#[from] crate::clients::heidrun::HeidrunClientError),
    #[error("Mimir client error: {0}")]
    MimirError(#[from] crate::clients::mimir::MimirClientError),
}

pub struct ClientManager {
    nornen_client: Arc<Mutex<NornenClient>>,
    heidrun_client: Arc<Mutex<HeidrunClient>>,
    mimir_client: Arc<Mutex<MimirClient>>,
}

impl ClientManager {
    pub async fn new(endpoints: ServiceEndpoints) -> Result<Self, ClientManagerError> {
        let nornen_client = Arc::new(Mutex::new(
            NornenClient::new(endpoints.nornen).await?
        ));
        let heidrun_client = Arc::new(Mutex::new(
            HeidrunClient::new(endpoints.heidrun).await?
        ));
        let mimir_client = Arc::new(Mutex::new(
            MimirClient::new(endpoints.mimir).await?
        ));
        
        Ok(Self {
            nornen_client,
            heidrun_client,
            mimir_client,
        })
    }

    pub fn nornen(&self) -> Arc<Mutex<NornenClient>> {
        self.nornen_client.clone()
    }

    pub fn heidrun(&self) -> Arc<Mutex<HeidrunClient>> {
        self.heidrun_client.clone()
    }

    pub fn mimir(&self) -> Arc<Mutex<MimirClient>> {
        self.mimir_client.clone()
    }
}
