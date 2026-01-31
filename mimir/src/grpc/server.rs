use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod mimir {
    tonic::include_proto!("mimir");
}

use mimir::mimir_service_server::{MimirService, MimirServiceServer};

pub struct MimirServiceImpl {
    database: Arc<crate::storage::EncryptedDatabase>,
    gdpr: Arc<crate::gdpr::GDPRCompliance>,
}

impl MimirServiceImpl {
    pub fn new(
        database: Arc<crate::storage::EncryptedDatabase>,
        gdpr: Arc<crate::gdpr::GDPRCompliance>,
    ) -> Self {
        Self { database, gdpr }
    }
}

#[tonic::async_trait]
impl MimirService for MimirServiceImpl {
    async fn store_data(
        &self,
        request: Request<mimir::StoreDataRequest>,
    ) -> Result<Response<mimir::StoreDataResponse>, Status> {
        let req = request.into_inner();
        
        let data_id = self.database.store_data(&req.user_id, &req.data).await
            .map_err(|e| Status::internal(format!("Storage failed: {}", e)))?;

        Ok(Response::new(mimir::StoreDataResponse {
            data_id,
        }))
    }

    async fn retrieve_data(
        &self,
        request: Request<mimir::RetrieveDataRequest>,
    ) -> Result<Response<mimir::RetrieveDataResponse>, Status> {
        let req = request.into_inner();
        
        let data = self.database.retrieve_data(&req.data_id, &req.user_id).await
            .map_err(|e| Status::internal(format!("Retrieval failed: {}", e)))?;

        Ok(Response::new(mimir::RetrieveDataResponse {
            data,
        }))
    }

    async fn delete_data(
        &self,
        request: Request<mimir::DeleteDataRequest>,
    ) -> Result<Response<mimir::DeleteDataResponse>, Status> {
        let req = request.into_inner();
        
        self.database.delete_data(&req.data_id, &req.user_id).await
            .map_err(|e| Status::internal(format!("Deletion failed: {}", e)))?;

        Ok(Response::new(mimir::DeleteDataResponse {
            success: true,
        }))
    }

    async fn export_user_data(
        &self,
        request: Request<mimir::ExportUserDataRequest>,
    ) -> Result<Response<mimir::ExportUserDataResponse>, Status> {
        let req = request.into_inner();
        
        let data = self.gdpr.export_user_data(&req.user_id).await
            .map_err(|e| Status::internal(format!("Export failed: {}", e)))?;

        Ok(Response::new(mimir::ExportUserDataResponse {
            data,
        }))
    }

    async fn delete_user_data(
        &self,
        request: Request<mimir::DeleteUserDataRequest>,
    ) -> Result<Response<mimir::DeleteUserDataResponse>, Status> {
        let req = request.into_inner();
        
        self.gdpr.delete_user_data(&req.user_id).await
            .map_err(|e| Status::internal(format!("Deletion failed: {}", e)))?;

        Ok(Response::new(mimir::DeleteUserDataResponse {
            success: true,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub database: Arc<crate::storage::EncryptedDatabase>,
    pub gdpr: Arc<crate::gdpr::GDPRCompliance>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Mimir gRPC server on {}", addr);

    let mimir_service = MimirServiceImpl::new(deps.database, deps.gdpr);

    Server::builder()
        .add_service(MimirServiceServer::new(mimir_service))
        .serve(addr)
        .await?;

    Ok(())
}
