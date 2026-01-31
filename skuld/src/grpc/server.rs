use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod skuld {
    tonic::include_proto!("skuld");
}

use skuld::skuld_service_server::{SkuldService, SkuldServiceServer};

pub struct SkuldServiceImpl {
    model_selector: Arc<crate::selection::ModelSelector>,
}

impl SkuldServiceImpl {
    pub fn new(model_selector: Arc<crate::selection::ModelSelector>) -> Self {
        Self { model_selector }
    }
}

#[tonic::async_trait]
impl SkuldService for SkuldServiceImpl {
    async fn select_model(
        &self,
        request: Request<skuld::SelectModelRequest>,
    ) -> Result<Response<skuld::SelectModelResponse>, Status> {
        let req = request.into_inner();
        
        let requirements = crate::selection::ModelRequirements {
            max_size: if req.max_size == 0 { None } else { Some(req.max_size) },
            min_reliability: if req.min_reliability == 0.0 { None } else { Some(req.min_reliability) },
            max_latency_ms: if req.max_latency_ms == 0 { None } else { Some(req.max_latency_ms) },
        };

        let model_name = self.model_selector.select_best_model(requirements).await
            .map_err(|e| Status::internal(format!("Model selection failed: {}", e)))?;

        // Get evaluation for selected model
        let evaluation = self.model_selector.evaluator().evaluate(&model_name).await
            .map_err(|e| Status::internal(format!("Model evaluation failed: {}", e)))?;

        let mut factor_scores = std::collections::HashMap::new();
        factor_scores.insert("performance".to_string(), evaluation.performance_score);
        factor_scores.insert("reliability".to_string(), evaluation.reliability_score);
        factor_scores.insert("efficiency".to_string(), evaluation.efficiency_score);

        Ok(Response::new(skuld::SelectModelResponse {
            model_name,
            score: evaluation.total_score,
            factor_scores,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub model_selector: Arc<crate::selection::ModelSelector>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Skuld gRPC server on {}", addr);

    let skuld_service = SkuldServiceImpl::new(deps.model_selector);

    Server::builder()
        .add_service(SkuldServiceServer::new(skuld_service))
        .serve(addr)
        .await?;

    Ok(())
}
