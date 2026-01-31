use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod laeradr {
    tonic::include_proto!("laeradr");
}

use laeradr::laeradr_service_server::{LaeradrService, LaeradrServiceServer};

pub struct LaeradrServiceImpl {
    coordinator: Arc<crate::coordinator::ServiceCoordinator>,
}

impl LaeradrServiceImpl {
    pub fn new(coordinator: Arc<crate::coordinator::ServiceCoordinator>) -> Self {
        Self { coordinator }
    }
}

#[tonic::async_trait]
impl LaeradrService for LaeradrServiceImpl {
    async fn index_data(
        &self,
        request: Request<laeradr::IndexDataRequest>,
    ) -> Result<Response<laeradr::IndexDataResponse>, Status> {
        let req = request.into_inner();
        
        let metadata: std::collections::HashMap<String, String> = req.metadata;
        let index_id = self.coordinator.index_data(&req.data_id, &req.data, &metadata).await
            .map_err(|e| Status::internal(format!("Indexing failed: {}", e)))?;

        Ok(Response::new(laeradr::IndexDataResponse {
            success: true,
            index_id,
        }))
    }

    async fn validate_data(
        &self,
        request: Request<laeradr::ValidateDataRequest>,
    ) -> Result<Response<laeradr::ValidateDataResponse>, Status> {
        let req = request.into_inner();
        
        let valid = self.coordinator.validate_data(&req.schema_id, &req.data).await
            .map_err(|e| Status::internal(format!("Validation failed: {}", e)))?;

        Ok(Response::new(laeradr::ValidateDataResponse {
            valid,
            errors: vec![],
        }))
    }

    async fn aggregate_data(
        &self,
        request: Request<laeradr::AggregateDataRequest>,
    ) -> Result<Response<laeradr::AggregateDataResponse>, Status> {
        let req = request.into_inner();
        
        let aggregated_data = self.coordinator.aggregate_data(&req.aggregation_type, &req.data_ids).await
            .map_err(|e| Status::internal(format!("Aggregation failed: {}", e)))?;

        Ok(Response::new(laeradr::AggregateDataResponse {
            success: true,
            aggregated_data,
        }))
    }

    async fn archive_data(
        &self,
        request: Request<laeradr::ArchiveDataRequest>,
    ) -> Result<Response<laeradr::ArchiveDataResponse>, Status> {
        let req = request.into_inner();
        
        let archive_id = self.coordinator.archive_data(&req.data_id, &req.archive_location).await
            .map_err(|e| Status::internal(format!("Archiving failed: {}", e)))?;

        Ok(Response::new(laeradr::ArchiveDataResponse {
            success: true,
            archive_id,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub coordinator: Arc<crate::coordinator::ServiceCoordinator>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Læraðr gRPC server on {}", addr);

    let laeradr_service = LaeradrServiceImpl::new(deps.coordinator);

    Server::builder()
        .add_service(LaeradrServiceServer::new(laeradr_service))
        .serve(addr)
        .await?;

    Ok(())
}
