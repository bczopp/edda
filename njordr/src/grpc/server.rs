use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod njordr {
    tonic::include_proto!("njordr");
}

use njordr::njordr_service_server::{NjordrService, NjordrServiceServer};

pub struct NjordrServiceImpl {
    earnings_manager: Arc<crate::earnings::EarningsManager>,
    settlement_processor: Arc<crate::settlements::SettlementProcessor>,
    trade_manager: Arc<crate::trade::TradeManager>,
    payment_gateway: Arc<crate::payment::PaymentGateway>,
}

impl NjordrServiceImpl {
    pub fn new(
        earnings_manager: Arc<crate::earnings::EarningsManager>,
        settlement_processor: Arc<crate::settlements::SettlementProcessor>,
        trade_manager: Arc<crate::trade::TradeManager>,
        payment_gateway: Arc<crate::payment::PaymentGateway>,
    ) -> Self {
        Self {
            earnings_manager,
            settlement_processor,
            trade_manager,
            payment_gateway,
        }
    }
}

#[tonic::async_trait]
impl NjordrService for NjordrServiceImpl {
    async fn calculate_earnings(
        &self,
        request: Request<njordr::CalculateEarningsRequest>,
    ) -> Result<Response<njordr::CalculateEarningsResponse>, Status> {
        let req = request.into_inner();
        
        let period_start = chrono::DateTime::parse_from_rfc3339(&req.period_start)
            .map_err(|e| Status::invalid_argument(format!("Invalid period_start: {}", e)))?
            .with_timezone(&chrono::Utc);
        
        let period_end = chrono::DateTime::parse_from_rfc3339(&req.period_end)
            .map_err(|e| Status::invalid_argument(format!("Invalid period_end: {}", e)))?
            .with_timezone(&chrono::Utc);
        
        let earnings = self.earnings_manager
            .calculate_earnings(&req.provider_id, period_start, period_end)
            .await
            .map_err(|e| Status::internal(format!("Earnings calculation failed: {}", e)))?;

        Ok(Response::new(njordr::CalculateEarningsResponse {
            total_earnings: earnings.total_earnings,
            commission: earnings.commission,
            net_earnings: earnings.net_earnings,
        }))
    }

    async fn generate_settlement(
        &self,
        request: Request<njordr::GenerateSettlementRequest>,
    ) -> Result<Response<njordr::GenerateSettlementResponse>, Status> {
        let req = request.into_inner();
        
        let period_start = chrono::DateTime::parse_from_rfc3339(&req.period_start)
            .map_err(|e| Status::invalid_argument(format!("Invalid period_start: {}", e)))?
            .with_timezone(&chrono::Utc);
        
        let period_end = chrono::DateTime::parse_from_rfc3339(&req.period_end)
            .map_err(|e| Status::invalid_argument(format!("Invalid period_end: {}", e)))?
            .with_timezone(&chrono::Utc);
        
        let settlement = self.settlement_processor
            .generate_settlement(&req.provider_id, period_start, period_end)
            .await
            .map_err(|e| Status::internal(format!("Settlement generation failed: {}", e)))?;

        Ok(Response::new(njordr::GenerateSettlementResponse {
            settlement_id: settlement.settlement_id,
            amount: settlement.amount,
            status: settlement.status,
        }))
    }

    async fn execute_settlement(
        &self,
        request: Request<njordr::ExecuteSettlementRequest>,
    ) -> Result<Response<njordr::ExecuteSettlementResponse>, Status> {
        let req = request.into_inner();
        
        let transaction_id = self.settlement_processor
            .execute_settlement(&req.settlement_id, &req.payment_method)
            .await
            .map_err(|e| Status::internal(format!("Settlement execution failed: {}", e)))?;

        Ok(Response::new(njordr::ExecuteSettlementResponse {
            transaction_id,
            status: "completed".to_string(),
        }))
    }

    async fn track_trade(
        &self,
        request: Request<njordr::TrackTradeRequest>,
    ) -> Result<Response<njordr::TrackTradeResponse>, Status> {
        let req = request.into_inner();
        
        let metadata = if !req.timestamp.is_empty() {
            Some(serde_json::json!({"timestamp": req.timestamp}))
        } else {
            None
        };

        self.trade_manager.track_trade(&req.trade_id, &req.provider_id, req.amount, metadata).await
            .map_err(|e| Status::internal(format!("Trade tracking failed: {}", e)))?;

        Ok(Response::new(njordr::TrackTradeResponse {
            success: true,
        }))
    }

    async fn get_trade_history(
        &self,
        request: Request<njordr::GetTradeHistoryRequest>,
    ) -> Result<Response<njordr::GetTradeHistoryResponse>, Status> {
        let req = request.into_inner();
        
        let limit = if req.limit > 0 { req.limit } else { 100 };
        let offset = if req.offset >= 0 { req.offset } else { 0 };
        
        let result = self.trade_manager
            .get_trade_history(&req.provider_id, limit, offset)
            .await
            .map_err(|e| Status::internal(format!("Failed to get trade history: {}", e)))?;

        let proto_trades: Vec<njordr::Trade> = result.trades.into_iter().map(|t| njordr::Trade {
            trade_id: t.trade_id,
            amount: t.amount,
            timestamp: t.timestamp.to_rfc3339(),
            status: t.status,
        }).collect();

        Ok(Response::new(njordr::GetTradeHistoryResponse {
            trades: proto_trades,
            total: result.total,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub earnings_manager: Arc<crate::earnings::EarningsManager>,
    pub settlement_processor: Arc<crate::settlements::SettlementProcessor>,
    pub trade_manager: Arc<crate::trade::TradeManager>,
    pub payment_gateway: Arc<crate::payment::PaymentGateway>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Njörðr gRPC server on {}", addr);

    let njordr_service = NjordrServiceImpl::new(
        deps.earnings_manager,
        deps.settlement_processor,
        deps.trade_manager,
        deps.payment_gateway,
    );

    Server::builder()
        .add_service(NjordrServiceServer::new(njordr_service))
        .serve(addr)
        .await?;

    Ok(())
}
