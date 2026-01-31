use tonic::transport::Channel;
use thiserror::Error;

pub mod nornen {
    tonic::include_proto!("nornen");
}

use nornen::{
    nornen_service_client::NornenServiceClient,
    verdandi_service_client::VerdandiServiceClient,
    CoordinateRequest, RouteRequestRequest,
};

#[derive(Debug, Error)]
pub enum NornenClientError {
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct NornenClient {
    nornen_client: NornenServiceClient<Channel>,
    verdandi_client: VerdandiServiceClient<Channel>,
}

impl NornenClient {
    pub async fn new(endpoint: String) -> Result<Self, NornenClientError> {
        let channel = Channel::from_shared(endpoint)?
            .connect()
            .await?;
        
        let nornen_client = NornenServiceClient::new(channel.clone());
        let verdandi_client = VerdandiServiceClient::new(channel);
        
        Ok(Self {
            nornen_client,
            verdandi_client,
        })
    }

    pub async fn coordinate_request(
        &mut self,
        request_id: String,
        request_type: String,
        context: std::collections::HashMap<String, String>,
    ) -> Result<nornen::CoordinateResponse, NornenClientError> {
        let request = tonic::Request::new(CoordinateRequest {
            request_id,
            request_type,
            context,
        });
        
        let response = self.nornen_client.coordinate_request(request).await?;
        Ok(response.into_inner())
    }

    pub async fn route_request(
        &mut self,
        request_id: String,
        required_capabilities: Vec<String>,
        context: std::collections::HashMap<String, String>,
    ) -> Result<nornen::RouteRequestResponse, NornenClientError> {
        let request = tonic::Request::new(RouteRequestRequest {
            request_id,
            required_capabilities,
            context,
        });
        
        let response = self.verdandi_client.route_request(request).await?;
        Ok(response.into_inner())
    }
}
