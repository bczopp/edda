use tonic::transport::Channel;
use tonic::Request;
use thiserror::Error;
use tracing::{info, error};
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod mimir {
    tonic::include_proto!("mimir");
}

use mimir::mimir_service_client::MimirServiceClient;
use mimir::{
    StoreDataRequest, StoreDataResponse,
    RetrieveDataRequest, RetrieveDataResponse,
    DeleteDataRequest, DeleteDataResponse,
    ExportUserDataRequest, ExportUserDataResponse,
    RectifyUserDataRequest, RectifyUserDataResponse,
};

#[derive(Debug, Error)]
pub enum MimirClientError {
    #[error("Mimir connection error: {0}")]
    ConnectionError(String),
    #[error("Mimir operation failed: {0}")]
    OperationFailed(String),
    #[error("Data not found")]
    NotFound,
}

pub struct MimirClient {
    client: Arc<RwLock<Option<MimirServiceClient<Channel>>>>,
    mimir_url: String,
}

impl MimirClient {
    pub fn new(mimir_url: String) -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
            mimir_url,
        }
    }

    pub async fn connect(&self) -> Result<(), MimirClientError> {
        let client = MimirServiceClient::connect(self.mimir_url.clone())
            .await
            .map_err(|e| MimirClientError::ConnectionError(e.to_string()))?;
        
        *self.client.write().await = Some(client);
        info!("Connected to Mimir at {}", self.mimir_url);
        Ok(())
    }

    pub async fn store_data(
        &self,
        user_id: &str,
        data: &[u8],
    ) -> Result<String, MimirClientError> {
        let client_guard = self.client.read().await;
        let client = client_guard.as_ref()
            .ok_or_else(|| MimirClientError::ConnectionError("Not connected".to_string()))?;
        
        let request = Request::new(StoreDataRequest {
            user_id: user_id.to_string(),
            data: data.to_vec(),
        });
        
        let mut client_clone = client.clone();
        let response = client_clone.store_data(request)
            .await
            .map_err(|e| MimirClientError::OperationFailed(e.to_string()))?
            .into_inner();
        
        Ok(response.data_id)
    }

    pub async fn retrieve_data(
        &self,
        data_id: &str,
        user_id: &str,
    ) -> Result<Vec<u8>, MimirClientError> {
        let client_guard = self.client.read().await;
        let client = client_guard.as_ref()
            .ok_or_else(|| MimirClientError::ConnectionError("Not connected".to_string()))?;
        
        let request = Request::new(RetrieveDataRequest {
            data_id: data_id.to_string(),
            user_id: user_id.to_string(),
        });
        
        let mut client_clone = client.clone();
        let response = client_clone.retrieve_data(request)
            .await
            .map_err(|e| {
                let status = e.code();
                if status == tonic::Code::NotFound {
                    MimirClientError::NotFound
                } else {
                    MimirClientError::OperationFailed(e.to_string())
                }
            })?
            .into_inner();
        
        Ok(response.data)
    }

    pub async fn delete_data(
        &self,
        data_id: &str,
        user_id: &str,
    ) -> Result<(), MimirClientError> {
        let client_guard = self.client.read().await;
        let client = client_guard.as_ref()
            .ok_or_else(|| MimirClientError::ConnectionError("Not connected".to_string()))?;
        
        let request = Request::new(DeleteDataRequest {
            data_id: data_id.to_string(),
            user_id: user_id.to_string(),
        });
        
        let mut client_clone = client.clone();
        client_clone.delete_data(request)
            .await
            .map_err(|e| MimirClientError::OperationFailed(e.to_string()))?;
        
        Ok(())
    }

    pub async fn export_user_data(
        &self,
        user_id: &str,
    ) -> Result<Vec<u8>, MimirClientError> {
        let client_guard = self.client.read().await;
        let client = client_guard.as_ref()
            .ok_or_else(|| MimirClientError::ConnectionError("Not connected".to_string()))?;
        
        let request = Request::new(ExportUserDataRequest {
            user_id: user_id.to_string(),
        });
        
        let mut client_clone = client.clone();
        let response = client_clone.export_user_data(request)
            .await
            .map_err(|e| MimirClientError::OperationFailed(e.to_string()))?
            .into_inner();
        
        Ok(response.data)
    }

    pub async fn rectify_user_data(
        &self,
        data_id: &str,
        user_id: &str,
        new_data: &[u8],
    ) -> Result<String, MimirClientError> {
        let client_guard = self.client.read().await;
        let client = client_guard.as_ref()
            .ok_or_else(|| MimirClientError::ConnectionError("Not connected".to_string()))?;
        
        let request = Request::new(RectifyUserDataRequest {
            user_id: user_id.to_string(),
            data_id: data_id.to_string(),
            new_data: new_data.to_vec(),
        });
        
        let mut client_clone = client.clone();
        let response = client_clone.rectify_user_data(request)
            .await
            .map_err(|e| MimirClientError::OperationFailed(e.to_string()))?
            .into_inner();
        
        Ok(response.data_id)
    }
}

impl Clone for MimirClient {
    fn clone(&self) -> Self {
        Self {
            client: Arc::clone(&self.client),
            mimir_url: self.mimir_url.clone(),
        }
    }
}
