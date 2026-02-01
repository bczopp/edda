use tonic::{transport::Server, Request, Response, Status};
use tracing::{info, warn};
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

    pub fn new_with_deps(deps: GrpcServerDependencies) -> Self {
        Self {
            database: deps.database,
            gdpr: deps.gdpr,
        }
    }

    /// Extract user_id from request metadata or use provided user_id
    /// In production, this would extract from JWT token or similar
    fn get_user_id_from_request<T>(&self, request: &Request<T>, default_user_id: &str) -> String {
        // TODO: Extract from request metadata (JWT token, etc.)
        // For now, use the provided user_id from the request
        default_user_id.to_string()
    }
}

#[tonic::async_trait]
impl MimirService for MimirServiceImpl {
    async fn store_data(
        &self,
        request: Request<mimir::StoreDataRequest>,
    ) -> Result<Response<mimir::StoreDataResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.get_user_id_from_request(&request, &req.user_id);
        
        // Use access control-aware method if available
        let data_id = if self.database.has_access_control() {
            self.database.store_data_with_access_control(&user_id, &req.data, &req.user_id).await
                .map_err(|e| {
                    warn!("Access denied for user {} storing data for {}", user_id, req.user_id);
                    match e {
                        crate::storage::StorageError::AccessDenied => {
                            Status::permission_denied("Access denied")
                        }
                        crate::storage::StorageError::AccessControlError(_) => {
                            Status::permission_denied("Access denied")
                        }
                        _ => Status::internal(format!("Storage failed: {}", e))
                    }
                })?
        } else {
            // Fallback to basic method for backward compatibility
            self.database.store_data(&req.user_id, &req.data).await
                .map_err(|e| Status::internal(format!("Storage failed: {}", e)))?
        };

        Ok(Response::new(mimir::StoreDataResponse {
            data_id,
        }))
    }

    async fn retrieve_data(
        &self,
        request: Request<mimir::RetrieveDataRequest>,
    ) -> Result<Response<mimir::RetrieveDataResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.get_user_id_from_request(&request, &req.user_id);
        
        // Use access control-aware method if available
        let data = if self.database.has_access_control() {
            self.database.retrieve_data_with_access_control(&req.data_id, &user_id, &req.user_id).await
                .map_err(|e| {
                    warn!("Access denied for user {} retrieving data {} for {}", user_id, req.data_id, req.user_id);
                    match e {
                        crate::storage::StorageError::AccessDenied => {
                            Status::permission_denied("Access denied")
                        }
                        crate::storage::StorageError::AccessControlError(_) => {
                            Status::permission_denied("Access denied")
                        }
                        _ => Status::internal(format!("Retrieval failed: {}", e))
                    }
                })?
        } else {
            // Fallback to basic method for backward compatibility
            self.database.retrieve_data(&req.data_id, &req.user_id).await
                .map_err(|e| Status::internal(format!("Retrieval failed: {}", e)))?
        };

        Ok(Response::new(mimir::RetrieveDataResponse {
            data,
        }))
    }

    async fn delete_data(
        &self,
        request: Request<mimir::DeleteDataRequest>,
    ) -> Result<Response<mimir::DeleteDataResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.get_user_id_from_request(&request, &req.user_id);
        
        // Use access control-aware method if available
        if self.database.has_access_control() {
            self.database.delete_data_with_access_control(&req.data_id, &user_id, &req.user_id).await
                .map_err(|e| {
                    warn!("Access denied for user {} deleting data {} for {}", user_id, req.data_id, req.user_id);
                    match e {
                        crate::storage::StorageError::AccessDenied => {
                            Status::permission_denied("Access denied")
                        }
                        crate::storage::StorageError::AccessControlError(_) => {
                            Status::permission_denied("Access denied")
                        }
                        _ => Status::internal(format!("Deletion failed: {}", e))
                    }
                })?;
        } else {
            // Fallback to basic method for backward compatibility
            self.database.delete_data(&req.data_id, &req.user_id).await
                .map_err(|e| Status::internal(format!("Deletion failed: {}", e)))?;
        }

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

    async fn rectify_user_data(
        &self,
        request: Request<mimir::RectifyUserDataRequest>,
    ) -> Result<Response<mimir::RectifyUserDataResponse>, Status> {
        let req = request.into_inner();
        let user_id = self.get_user_id_from_request(&request, &req.user_id);
        
        // Use access control-aware method if available
        if self.database.has_access_control() {
            self.database.update_data_with_access_control(
                &req.data_id,
                &user_id,
                &req.user_id,
                &req.new_data,
            ).await
            .map_err(|e| {
                warn!("Access denied or error for user {} rectifying data {} for {}", user_id, req.data_id, req.user_id);
                match e {
                    crate::storage::StorageError::AccessDenied => {
                        Status::permission_denied("Access denied")
                    }
                    crate::storage::StorageError::AccessControlError(_) => {
                        Status::permission_denied("Access denied")
                    }
                    crate::storage::StorageError::NotFound => {
                        Status::not_found(format!("Data {} not found for user {}", req.data_id, req.user_id))
                    }
                    _ => Status::internal(format!("Rectification failed: {}", e))
                }
            })?;
        } else {
            // Use GDPR compliance method (which uses basic update_data)
            self.gdpr.rectify_user_data(&req.user_id, &req.data_id, &req.new_data).await
                .map_err(|e| {
                    match e {
                        crate::gdpr::GDPRError::DataNotFound(msg) => {
                            Status::not_found(msg)
                        }
                        _ => Status::internal(format!("Rectification failed: {}", e))
                    }
                })?;
        }
        
        Ok(Response::new(mimir::RectifyUserDataResponse {
            success: true,
            data_id: req.data_id, // Same ID is preserved with UPDATE
        }))
    }
}

pub struct GrpcServerDependencies {
    pub database: Arc<crate::storage::EncryptedDatabase>,
    pub gdpr: Arc<crate::gdpr::GDPRCompliance>,
    pub access_control: Option<Arc<crate::access_control::AccessControlManager>>,
    pub audit_logger: Option<Arc<crate::audit::AuditLogManager>>,
}

impl GrpcServerDependencies {
    pub fn new(
        database: Arc<crate::storage::EncryptedDatabase>,
        gdpr: Arc<crate::gdpr::GDPRCompliance>,
    ) -> Self {
        Self {
            database,
            gdpr,
            access_control: None,
            audit_logger: None,
        }
    }
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Mimir gRPC server on {}", addr);

    let mimir_service = MimirServiceImpl::new_with_deps(deps);

    Server::builder()
        .add_service(MimirServiceServer::new(mimir_service))
        .serve(addr)
        .await?;

    Ok(())
}
