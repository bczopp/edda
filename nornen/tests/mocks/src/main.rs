use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use tracing::info;
use serde_json::json;
use base64::prelude::{Engine as _, BASE64_STANDARD};

pub mod mimir {
    tonic::include_proto!("mimir");
}

use mimir::mimir_service_server::{MimirService, MimirServiceServer};
use mimir::{
    StoreDataRequest, StoreDataResponse,
    RetrieveDataRequest, RetrieveDataResponse,
    DeleteDataRequest, DeleteDataResponse,
    ExportUserDataRequest, ExportUserDataResponse,
    DeleteUserDataRequest, DeleteUserDataResponse,
    RectifyUserDataRequest, RectifyUserDataResponse,
};

// In-memory storage for mock Mimir service
#[derive(Clone)]
struct MockMimirStorage {
    // data_id -> (user_id, data)
    data: Arc<RwLock<HashMap<String, (String, Vec<u8>)>>>,
    next_id: Arc<RwLock<u64>>,
}

impl MockMimirStorage {
    fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }

    async fn generate_data_id(&self) -> String {
        let mut next_id = self.next_id.write().await;
        let id = format!("data_{}", *next_id);
        *next_id += 1;
        id
    }
}

struct MimirServiceImpl {
    storage: MockMimirStorage,
}

impl MimirServiceImpl {
    fn new() -> Self {
        Self {
            storage: MockMimirStorage::new(),
        }
    }
}

#[tonic::async_trait]
impl MimirService for MimirServiceImpl {
    async fn store_data(
        &self,
        request: Request<StoreDataRequest>,
    ) -> Result<Response<StoreDataResponse>, Status> {
        let req = request.into_inner();
        let data_id = self.storage.generate_data_id().await;
        
        let mut storage = self.storage.data.write().await;
        storage.insert(data_id.clone(), (req.user_id.clone(), req.data));
        
        info!("Stored data for user {} with data_id {}", req.user_id, data_id);
        
        Ok(Response::new(StoreDataResponse { data_id }))
    }

    async fn retrieve_data(
        &self,
        request: Request<RetrieveDataRequest>,
    ) -> Result<Response<RetrieveDataResponse>, Status> {
        let req = request.into_inner();
        
        let storage = self.storage.data.read().await;
        if let Some((user_id, data)) = storage.get(&req.data_id) {
            if user_id == &req.user_id {
                Ok(Response::new(RetrieveDataResponse { data: data.clone() }))
            } else {
                Err(Status::permission_denied("User ID mismatch"))
            }
        } else {
            Err(Status::not_found("Data not found"))
        }
    }

    async fn delete_data(
        &self,
        request: Request<DeleteDataRequest>,
    ) -> Result<Response<DeleteDataResponse>, Status> {
        let req = request.into_inner();
        
        let mut storage = self.storage.data.write().await;
        if let Some((user_id, _)) = storage.get(&req.data_id) {
            if user_id == &req.user_id {
                storage.remove(&req.data_id);
                info!("Deleted data {} for user {}", req.data_id, req.user_id);
                Ok(Response::new(DeleteDataResponse { success: true }))
            } else {
                Err(Status::permission_denied("User ID mismatch"))
            }
        } else {
            Err(Status::not_found("Data not found"))
        }
    }

    async fn export_user_data(
        &self,
        request: Request<ExportUserDataRequest>,
    ) -> Result<Response<ExportUserDataResponse>, Status> {
        let req = request.into_inner();
        
        let storage = self.storage.data.read().await;
        let mut data_entries = Vec::new();
        
        for (data_id, (user_id, data)) in storage.iter() {
            if user_id == &req.user_id {
                let data_base64 = BASE64_STANDARD.encode(data);
                data_entries.push(json!({
                    "data_id": data_id,
                    "data": data_base64
                }));
            }
        }
        
        let export_json = json!({
            "user_id": req.user_id,
            "export_timestamp": chrono::Utc::now().to_rfc3339(),
            "data_entries": data_entries
        });
        
        let export_data = serde_json::to_vec(&export_json)
            .map_err(|e| Status::internal(format!("Failed to serialize export: {}", e)))?;
        
        info!("Exported {} entries for user {}", data_entries.len(), req.user_id);
        
        Ok(Response::new(ExportUserDataResponse { data: export_data }))
    }

    async fn delete_user_data(
        &self,
        request: Request<DeleteUserDataRequest>,
    ) -> Result<Response<DeleteUserDataResponse>, Status> {
        let req = request.into_inner();
        
        let mut storage = self.storage.data.write().await;
        let mut count = 0;
        
        storage.retain(|_, (user_id, _)| {
            if user_id == &req.user_id {
                count += 1;
                false
            } else {
                true
            }
        });
        
        info!("Deleted {} entries for user {}", count, req.user_id);
        
        Ok(Response::new(DeleteUserDataResponse { success: true }))
    }

    async fn rectify_user_data(
        &self,
        request: Request<RectifyUserDataRequest>,
    ) -> Result<Response<RectifyUserDataResponse>, Status> {
        let req = request.into_inner();
        
        let mut storage = self.storage.data.write().await;
        if let Some((user_id, _)) = storage.get(&req.data_id) {
            if user_id == &req.user_id {
                storage.insert(req.data_id.clone(), (req.user_id.clone(), req.new_data));
                info!("Rectified data {} for user {}", req.data_id, req.user_id);
                Ok(Response::new(RectifyUserDataResponse { 
                    success: true,
                    data_id: req.data_id.clone(),
                }))
            } else {
                Err(Status::permission_denied("User ID mismatch"))
            }
        } else {
            Err(Status::not_found("Data not found"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let port: u16 = std::env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "50051".to_string())
        .parse()
        .unwrap_or(50051);
    
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
    info!("Mock Mimir service listening on {}", addr);

    let mimir_service = MimirServiceImpl::new();

    tonic::transport::Server::builder()
        .add_service(MimirServiceServer::new(mimir_service))
        .serve(addr)
        .await?;
    
    Ok(())
}
