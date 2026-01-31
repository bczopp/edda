use tonic::transport::Channel;
use thiserror::Error;

pub mod heidrun {
    tonic::include_proto!("heidrun");
}

use heidrun::{
    heidrun_service_client::HeidrunServiceClient,
    CountTokensRequest, CalculatePriceRequest, ProcessSettlementRequest, PreAuthorizeRequest,
};

#[derive(Debug, Error)]
pub enum HeidrunClientError {
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct HeidrunClient {
    client: HeidrunServiceClient<Channel>,
}

impl HeidrunClient {
    pub async fn new(endpoint: String) -> Result<Self, HeidrunClientError> {
        let channel = Channel::from_shared(endpoint)?
            .connect()
            .await?;
        
        let client = HeidrunServiceClient::new(channel);
        
        Ok(Self { client })
    }

    pub async fn count_tokens(
        &mut self,
        text: String,
        model: String,
    ) -> Result<i64, HeidrunClientError> {
        let request = tonic::Request::new(CountTokensRequest {
            text,
            model,
        });
        
        let response = self.client.count_tokens(request).await?;
        Ok(response.into_inner().token_count)
    }

    pub async fn calculate_price(
        &mut self,
        token_count: i64,
        model: String,
        provider_id: String,
    ) -> Result<heidrun::CalculatePriceResponse, HeidrunClientError> {
        let request = tonic::Request::new(CalculatePriceRequest {
            token_count,
            model,
            provider_id,
        });
        
        let response = self.client.calculate_price(request).await?;
        Ok(response.into_inner())
    }

    pub async fn process_settlement(
        &mut self,
        provider_id: String,
        period_start: String,
        period_end: String,
    ) -> Result<heidrun::ProcessSettlementResponse, HeidrunClientError> {
        let request = tonic::Request::new(ProcessSettlementRequest {
            provider_id,
            period_start,
            period_end,
        });
        
        let response = self.client.process_settlement(request).await?;
        Ok(response.into_inner())
    }

    pub async fn pre_authorize(
        &mut self,
        user_id: String,
        amount: f64,
        currency: String,
    ) -> Result<heidrun::PreAuthorizeResponse, HeidrunClientError> {
        let request = tonic::Request::new(PreAuthorizeRequest {
            user_id,
            amount,
            currency,
        });
        
        let response = self.client.pre_authorize(request).await?;
        Ok(response.into_inner())
    }
}
