use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use crate::clients::ServiceClientConfig;

pub mod responsibility {
    tonic::include_proto!("responsibility");
}

use responsibility::responsibility_service_client::ResponsibilityServiceClient;
use responsibility::{
    TakeResponsibilityRequest, TakeResponsibilityResponse,
    ReturnResponsibilityRequest, ReturnResponsibilityResponse,
    RejectResponsibilityRequest, RejectResponsibilityResponse,
};

/// Client for Responsibility Service
/// Used to manage responsibility for requests
pub struct ResponsibilityClient {
    client: ResponsibilityServiceClient<Channel>,
}

impl ResponsibilityClient {
    /// Create a new Responsibility client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = ResponsibilityServiceClient::new(channel);
        
        Ok(Self { client })
    }

    /// Request a service to take responsibility for a request
    pub async fn take_responsibility(&mut self, request: TakeResponsibilityRequest) -> Result<TakeResponsibilityResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.take_responsibility(req).await?;
        Ok(response.into_inner())
    }

    /// Request a service to return responsibility
    pub async fn return_responsibility(&mut self, request: ReturnResponsibilityRequest) -> Result<ReturnResponsibilityResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.return_responsibility(req).await?;
        Ok(response.into_inner())
    }

    /// Request a service to reject responsibility
    pub async fn reject_responsibility(&mut self, request: RejectResponsibilityRequest) -> Result<RejectResponsibilityResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.reject_responsibility(req).await?;
        Ok(response.into_inner())
    }
}
