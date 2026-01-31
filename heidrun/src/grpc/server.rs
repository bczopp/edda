use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod heidrun {
    tonic::include_proto!("heidrun");
}

use heidrun::heidrun_service_server::{HeidrunService, HeidrunServiceServer};

pub struct HeidrunServiceImpl {
    token_counter: Arc<crate::token::TokenCounter>,
    pricing_calculator: Arc<crate::pricing::PricingCalculator>,
    settlement_processor: Arc<crate::settlement::SettlementProcessor>,
    preauth_manager: Arc<crate::preauth::PreAuthManager>,
}

impl HeidrunServiceImpl {
    pub fn new(
        token_counter: Arc<crate::token::TokenCounter>,
        pricing_calculator: Arc<crate::pricing::PricingCalculator>,
        settlement_processor: Arc<crate::settlement::SettlementProcessor>,
        preauth_manager: Arc<crate::preauth::PreAuthManager>,
    ) -> Self {
        Self {
            token_counter,
            pricing_calculator,
            settlement_processor,
            preauth_manager,
        }
    }
}

#[tonic::async_trait]
impl HeidrunService for HeidrunServiceImpl {
    async fn count_tokens(
        &self,
        request: Request<heidrun::CountTokensRequest>,
    ) -> Result<Response<heidrun::CountTokensResponse>, Status> {
        let req = request.into_inner();
        
        let token_count = self.token_counter.count_tokens(&req.text).await
            .map_err(|e| Status::internal(format!("Token counting failed: {}", e)))?;

        Ok(Response::new(heidrun::CountTokensResponse {
            token_count: token_count as i64,
        }))
    }

    async fn calculate_price(
        &self,
        request: Request<heidrun::CalculatePriceRequest>,
    ) -> Result<Response<heidrun::CalculatePriceResponse>, Status> {
        let req = request.into_inner();
        
        let result = self.pricing_calculator
            .calculate_price(req.token_count, &req.model, &req.provider_id)
            .await
            .map_err(|e| Status::internal(format!("Price calculation failed: {}", e)))?;

        Ok(Response::new(heidrun::CalculatePriceResponse {
            price: result.price,
            commission: result.commission,
            net_price: result.net_price,
        }))
    }

    async fn process_settlement(
        &self,
        request: Request<heidrun::ProcessSettlementRequest>,
    ) -> Result<Response<heidrun::ProcessSettlementResponse>, Status> {
        let req = request.into_inner();
        
        let period_start = chrono::DateTime::parse_from_rfc3339(&req.period_start)
            .map_err(|e| Status::invalid_argument(format!("Invalid period_start: {}", e)))?
            .with_timezone(&chrono::Utc);
        
        let period_end = chrono::DateTime::parse_from_rfc3339(&req.period_end)
            .map_err(|e| Status::invalid_argument(format!("Invalid period_end: {}", e)))?
            .with_timezone(&chrono::Utc);
        
        let settlement = self.settlement_processor
            .process_settlement(&req.provider_id, period_start, period_end)
            .await
            .map_err(|e| Status::internal(format!("Settlement processing failed: {}", e)))?;

        Ok(Response::new(heidrun::ProcessSettlementResponse {
            settlement_id: settlement.settlement_id,
            amount: settlement.amount,
            status: settlement.status,
        }))
    }

    async fn pre_authorize(
        &self,
        request: Request<heidrun::PreAuthorizeRequest>,
    ) -> Result<Response<heidrun::PreAuthorizeResponse>, Status> {
        let req = request.into_inner();
        
        let currency = if req.currency.is_empty() { "USD" } else { &req.currency };
        
        let preauth = self.preauth_manager
            .pre_authorize(&req.user_id, req.amount, currency)
            .await
            .map_err(|e| Status::internal(format!("Pre-authorization failed: {}", e)))?;

        Ok(Response::new(heidrun::PreAuthorizeResponse {
            authorization_id: preauth.authorization_id,
            approved: preauth.status == "approved",
        }))
    }
}

pub struct GrpcServerDependencies {
    pub token_counter: Arc<crate::token::TokenCounter>,
    pub pricing_calculator: Arc<crate::pricing::PricingCalculator>,
    pub settlement_processor: Arc<crate::settlement::SettlementProcessor>,
    pub preauth_manager: Arc<crate::preauth::PreAuthManager>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Heidrun gRPC server on {}", addr);

    let heidrun_service = HeidrunServiceImpl::new(
        deps.token_counter,
        deps.pricing_calculator,
        deps.settlement_processor,
        deps.preauth_manager,
    );

    Server::builder()
        .add_service(HeidrunServiceServer::new(heidrun_service))
        .serve(addr)
        .await?;

    Ok(())
}
