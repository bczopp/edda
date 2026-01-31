use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod hirtir {
    tonic::include_proto!("hirtir");
}

use hirtir::hirtir_service_server::{HirtirService, HirtirServiceServer};

pub struct HirtirServiceImpl {
    coordinator: Arc<crate::coordinator::ServiceCoordinator>,
}

impl HirtirServiceImpl {
    pub fn new(coordinator: Arc<crate::coordinator::ServiceCoordinator>) -> Self {
        Self { coordinator }
    }
}

#[tonic::async_trait]
impl HirtirService for HirtirServiceImpl {
    async fn index_data(
        &self,
        request: Request<hirtir::IndexDataRequest>,
    ) -> Result<Response<hirtir::IndexDataResponse>, Status> {
        let req = request.into_inner();
        
        let metadata: std::collections::HashMap<String, String> = req.metadata;
        let index_id = self.coordinator.index_data(&req.data_id, &req.data, &metadata).await
            .map_err(|e| Status::internal(format!("Indexing failed: {}", e)))?;

        Ok(Response::new(hirtir::IndexDataResponse {
            success: true,
            index_id,
        }))
    }

    async fn validate_data(
        &self,
        request: Request<hirtir::ValidateDataRequest>,
    ) -> Result<Response<hirtir::ValidateDataResponse>, Status> {
        let req = request.into_inner();
        
        let valid = self.coordinator.validate_data(&req.schema_id, &req.data).await
            .map_err(|e| Status::internal(format!("Validation failed: {}", e)))?;

        Ok(Response::new(hirtir::ValidateDataResponse {
            valid,
            errors: vec![],
        }))
    }

    async fn aggregate_data(
        &self,
        request: Request<hirtir::AggregateDataRequest>,
    ) -> Result<Response<hirtir::AggregateDataResponse>, Status> {
        let req = request.into_inner();
        
        let aggregated_data = self.coordinator.aggregate_data(&req.aggregation_type, &req.data_ids).await
            .map_err(|e| Status::internal(format!("Aggregation failed: {}", e)))?;

        Ok(Response::new(hirtir::AggregateDataResponse {
            success: true,
            aggregated_data,
        }))
    }

    async fn archive_data(
        &self,
        request: Request<hirtir::ArchiveDataRequest>,
    ) -> Result<Response<hirtir::ArchiveDataResponse>, Status> {
        let req = request.into_inner();
        
        let archive_id = self.coordinator.archive_data(&req.data_id, &req.archive_location).await
            .map_err(|e| Status::internal(format!("Archiving failed: {}", e)))?;

        Ok(Response::new(hirtir::ArchiveDataResponse {
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
    info!("Starting Hirtir gRPC server on {}", addr);

    let hirtir_service = HirtirServiceImpl::new(deps.coordinator);

    Server::builder()
        .add_service(HirtirServiceServer::new(hirtir_service))
        .serve(addr)
        .await?;

    Ok(())
}
