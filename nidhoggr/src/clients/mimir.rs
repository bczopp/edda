use tonic::transport::Channel;
use thiserror::Error;

pub mod mimir {
    tonic::include_proto!("mimir");
}

use mimir::{
    mimir_service_client::MimirServiceClient,
    StoreDataRequest, RetrieveDataRequest, DeleteDataRequest,
    ExportUserDataRequest, DeleteUserDataRequest,
};

#[derive(Debug, Error)]
pub enum MimirClientError {
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct MimirClient {
    client: MimirServiceClient<Channel>,
}

impl MimirClient {
    pub async fn new(endpoint: String) -> Result<Self, MimirClientError> {
        let channel = Channel::from_shared(endpoint)?
            .connect()
            .await?;
        
        let client = MimirServiceClient::new(channel);
        
        Ok(Self { client })
    }

    pub async fn store_data(
        &mut self,
        user_id: String,
        data: Vec<u8>,
    ) -> Result<String, MimirClientError> {
        let request = tonic::Request::new(StoreDataRequest {
            user_id,
            data,
        });
        
        let response = self.client.store_data(request).await?;
        Ok(response.into_inner().data_id)
    }

    pub async fn retrieve_data(
        &mut self,
        data_id: String,
        user_id: String,
    ) -> Result<Vec<u8>, MimirClientError> {
        let request = tonic::Request::new(RetrieveDataRequest {
            data_id,
            user_id,
        });
        
        let response = self.client.retrieve_data(request).await?;
        Ok(response.into_inner().data)
    }

    pub async fn delete_data(
        &mut self,
        data_id: String,
        user_id: String,
    ) -> Result<bool, MimirClientError> {
        let request = tonic::Request::new(DeleteDataRequest {
            data_id,
            user_id,
        });
        
        let response = self.client.delete_data(request).await?;
        Ok(response.into_inner().success)
    }

    pub async fn export_user_data(
        &mut self,
        user_id: String,
    ) -> Result<Vec<u8>, MimirClientError> {
        let request = tonic::Request::new(ExportUserDataRequest {
            user_id,
        });
        
        let response = self.client.export_user_data(request).await?;
        Ok(response.into_inner().data)
    }

    pub async fn delete_user_data(
        &mut self,
        user_id: String,
    ) -> Result<bool, MimirClientError> {
        let request = tonic::Request::new(DeleteUserDataRequest {
            user_id,
        });
        
        let response = self.client.delete_user_data(request).await?;
        Ok(response.into_inner().success)
    }
}
