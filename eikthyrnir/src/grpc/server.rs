use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod eikthyrnir {
    tonic::include_proto!("eikthyrnir");
}

use eikthyrnir::eikthyrnir_service_server::{EikthyrnirService, EikthyrnirServiceServer};

pub struct EikthyrnirServiceImpl {
    quality_assessor: Arc<crate::assessment::QualityAssessor>,
    quality_aggregator: Arc<crate::aggregation::QualityAggregator>,
    metrics_tracker: Arc<crate::metrics::MetricsTracker>,
}

impl EikthyrnirServiceImpl {
    pub fn new(
        quality_assessor: Arc<crate::assessment::QualityAssessor>,
        quality_aggregator: Arc<crate::aggregation::QualityAggregator>,
        metrics_tracker: Arc<crate::metrics::MetricsTracker>,
    ) -> Self {
        Self {
            quality_assessor,
            quality_aggregator,
            metrics_tracker,
        }
    }
}

#[tonic::async_trait]
impl EikthyrnirService for EikthyrnirServiceImpl {
    async fn assess_quality(
        &self,
        request: Request<eikthyrnir::AssessQualityRequest>,
    ) -> Result<Response<eikthyrnir::AssessQualityResponse>, Status> {
        let req = request.into_inner();
        
        let quality_data_proto = req.data.ok_or_else(|| Status::invalid_argument("QualityData missing"))?;
        
        let mut custom_metrics = std::collections::HashMap::new();
        for (k, v) in quality_data_proto.custom_metrics {
            custom_metrics.insert(k, v);
        }
        
        let quality_data = crate::assessment::QualityData {
            response_time: quality_data_proto.response_time,
            accuracy: quality_data_proto.accuracy,
            availability: quality_data_proto.availability,
            custom_metrics,
        };
        
        let assessment = self.quality_assessor
            .assess_quality(&req.provider_id, &req.service_id, quality_data)
            .await
            .map_err(|e| Status::internal(format!("Quality assessment failed: {}", e)))?;

        Ok(Response::new(eikthyrnir::AssessQualityResponse {
            quality_score: assessment.quality_score,
            metrics: assessment.metrics,
        }))
    }

    async fn aggregate_quality(
        &self,
        request: Request<eikthyrnir::AggregateQualityRequest>,
    ) -> Result<Response<eikthyrnir::AggregateQualityResponse>, Status> {
        let req = request.into_inner();
        
        let period_start = chrono::DateTime::parse_from_rfc3339(&req.period_start)
            .map_err(|e| Status::invalid_argument(format!("Invalid period_start: {}", e)))?
            .with_timezone(&chrono::Utc);
        
        let period_end = chrono::DateTime::parse_from_rfc3339(&req.period_end)
            .map_err(|e| Status::invalid_argument(format!("Invalid period_end: {}", e)))?
            .with_timezone(&chrono::Utc);
        
        let aggregation = self.quality_aggregator
            .aggregate(&req.provider_id, period_start, period_end)
            .await
            .map_err(|e| Status::internal(format!("Aggregation failed: {}", e)))?;

        Ok(Response::new(eikthyrnir::AggregateQualityResponse {
            aggregated_score: aggregation.aggregated_score,
            aggregated_metrics: aggregation.aggregated_metrics,
        }))
    }

    async fn get_quality_metrics(
        &self,
        request: Request<eikthyrnir::GetQualityMetricsRequest>,
    ) -> Result<Response<eikthyrnir::GetQualityMetricsResponse>, Status> {
        let req = request.into_inner();
        
        let limit = if req.limit > 0 { req.limit } else { 100 };
        
        let metrics = self.metrics_tracker
            .get_metrics(&req.provider_id, limit)
            .await
            .map_err(|e| Status::internal(format!("Failed to get metrics: {}", e)))?;

        let proto_metrics: Vec<eikthyrnir::QualityMetric> = metrics.into_iter().map(|m| {
            eikthyrnir::QualityMetric {
                metric_id: m.metric_id,
                provider_id: m.provider_id,
                value: m.value,
                timestamp: m.timestamp.to_rfc3339(),
            }
        }).collect();

        Ok(Response::new(eikthyrnir::GetQualityMetricsResponse {
            metrics: proto_metrics,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub quality_assessor: Arc<crate::assessment::QualityAssessor>,
    pub quality_aggregator: Arc<crate::aggregation::QualityAggregator>,
    pub metrics_tracker: Arc<crate::metrics::MetricsTracker>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Eikthyrnir gRPC server on {}", addr);

    let eikthyrnir_service = EikthyrnirServiceImpl::new(
        deps.quality_assessor,
        deps.quality_aggregator,
        deps.metrics_tracker,
    );

    Server::builder()
        .add_service(EikthyrnirServiceServer::new(eikthyrnir_service))
        .serve(addr)
        .await?;

    Ok(())
}
