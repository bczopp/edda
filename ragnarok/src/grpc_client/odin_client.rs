use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OdinClientError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

#[derive(Clone)]
pub struct OdinClient {
    client: odin::odin_service_client::OdinServiceClient<Channel>,
}

impl OdinClient {
    pub async fn new(port: u16) -> Result<Self, OdinClientError> {
        let endpoint = format!("http://127.0.0.1:{}", port);
        let channel = Channel::from_shared(endpoint)
            .map_err(|e| OdinClientError::InvalidUri(e.to_string()))?
            .connect()
            .await?;
        
        let client = odin::odin_service_client::OdinServiceClient::new(channel);
        
        Ok(Self { client })
    }

    pub async fn process_request(&mut self, request: odin::ProcessRequest) -> Result<odin::ProcessResponse, OdinClientError> {
        let response = self.client
            .process(tonic::Request::new(request))
            .await?;
        Ok(response.into_inner())
    }
}

pub mod odin {
    tonic::include_proto!("odin");
}
