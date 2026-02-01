use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;
use crate::security::access_control::{AccessControl, Permission};

pub mod nornen {
    tonic::include_proto!("nornen");
}

use nornen::{
    nornen_service_server::{NornenService, NornenServiceServer},
    urd_service_server::{UrdService, UrdServiceServer},
    verdandi_service_server::{VerdandiService, VerdandiServiceServer},
};

// Nornen Service Implementation
pub struct NornenServiceImpl {
    coordinator: Arc<crate::coordinator::NornenCoordinator>,
    access_control: Arc<AccessControl>,
}

impl NornenServiceImpl {
    pub fn new(coordinator: Arc<crate::coordinator::NornenCoordinator>, access_control: Arc<AccessControl>) -> Self {
        Self { coordinator, access_control }
    }
}

#[tonic::async_trait]
impl NornenService for NornenServiceImpl {
    async fn coordinate_request(
        &self,
        request: Request<nornen::CoordinateRequest>,
    ) -> Result<Response<nornen::CoordinateResponse>, Status> {
        // Check access control
        let _user_id = self.access_control.check_access(&request, &Permission::CoordinateRequest)
            .map_err(|e| Status::permission_denied(e.to_string()))?;
        
        let req = request.into_inner();
        
        let context: HashMap<String, String> = req.context.into_iter().collect();
        
        let result = self.coordinator
            .coordinate_request(&req.request_id, &req.request_type, &context)
            .await
            .map_err(|e| Status::internal(format!("Coordination failed: {}", e)))?;

        Ok(Response::new(nornen::CoordinateResponse {
            decision: result.decision,
            provider_id: result.provider_id,
            confidence: result.confidence,
            reasoning: result.reasoning,
        }))
    }
}

// Urd Service Implementation
pub struct UrdServiceImpl {
    registry: Arc<crate::urd::registry::ProviderRegistry>,
}

impl UrdServiceImpl {
    pub fn new(registry: Arc<crate::urd::registry::ProviderRegistry>) -> Self {
        Self { registry }
    }
}

#[tonic::async_trait]
impl UrdService for UrdServiceImpl {
    async fn register_provider(
        &self,
        request: Request<nornen::RegisterProviderRequest>,
    ) -> Result<Response<nornen::RegisterProviderResponse>, Status> {
        // Check access control
        let _user_id = self.access_control.check_access(&request, &Permission::RegisterProvider)
            .map_err(|e| Status::permission_denied(e.to_string()))?;
        
        let req = request.into_inner();
        
        let capabilities: Vec<String> = req.capabilities.into_iter().collect();
        let metadata = serde_json::json!(req.metadata);
        
        self.registry
            .register_provider(
                &req.provider_id,
                &req.name,
                &capabilities,
                &req.endpoint,
                &metadata,
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to register provider: {}", e)))?;

        Ok(Response::new(nornen::RegisterProviderResponse {
            success: true,
            message: format!("Provider {} registered successfully", req.provider_id),
        }))
    }

    async fn update_provider(
        &self,
        request: Request<nornen::UpdateProviderRequest>,
    ) -> Result<Response<nornen::UpdateProviderResponse>, Status> {
        // Check access control
        let _user_id = self.access_control.check_access(&request, &Permission::UpdateProvider)
            .map_err(|e| Status::permission_denied(e.to_string()))?;
        
        let req = request.into_inner();
        
        let capabilities: Option<Vec<String>> = if req.capabilities.is_empty() {
            None
        } else {
            Some(req.capabilities.into_iter().collect())
        };
        
        let metadata = if req.metadata.is_empty() {
            None
        } else {
            Some(serde_json::json!(req.metadata))
        };
        
        self.registry
            .update_provider(
                &req.provider_id,
                None, // name not provided in UpdateProviderRequest
                capabilities.as_deref(),
                None, // endpoint not provided in UpdateProviderRequest
                metadata.as_ref(),
            )
            .await
            .map_err(|e| Status::internal(format!("Failed to update provider: {}", e)))?;

        // Update status if provided
        if !req.status.is_empty() {
            self.registry
                .update_provider_status(&req.provider_id, &req.status)
                .await
                .map_err(|e| Status::internal(format!("Failed to update provider status: {}", e)))?;
        }

        Ok(Response::new(nornen::UpdateProviderResponse {
            success: true,
        }))
    }

    async fn query_providers(
        &self,
        request: Request<nornen::QueryProvidersRequest>,
    ) -> Result<Response<nornen::QueryProvidersResponse>, Status> {
        // Check access control
        let _user_id = self.access_control.check_access(&request, &Permission::QueryProviders)
            .map_err(|e| Status::permission_denied(e.to_string()))?;
        
        let req = request.into_inner();
        
        let capabilities: Vec<String> = req.capabilities.into_iter().collect();
        let status = if req.status.is_empty() {
            None
        } else {
            Some(req.status.as_str())
        };
        
        let providers = self.registry
            .query_providers(&capabilities, status)
            .await
            .map_err(|e| Status::internal(format!("Failed to query providers: {}", e)))?;

        let provider_infos: Vec<nornen::ProviderInfo> = providers
            .into_iter()
            .map(|p| {
                let metadata_map: HashMap<String, String> = if let Some(obj) = p.metadata.as_object() {
                    obj.iter()
                        .filter_map(|(k, v)| {
                            v.as_str().map(|s| (k.clone(), s.to_string()))
                        })
                        .collect()
                } else {
                    HashMap::new()
                };
                
                nornen::ProviderInfo {
                    provider_id: p.provider_id,
                    name: p.name,
                    capabilities: p.capabilities,
                    endpoint: p.endpoint,
                    status: p.status,
                    metadata: metadata_map,
                }
            })
            .collect();

        Ok(Response::new(nornen::QueryProvidersResponse {
            providers: provider_infos,
        }))
    }

    async fn list_providers(
        &self,
        request: Request<nornen::ListProvidersRequest>,
    ) -> Result<Response<nornen::ListProvidersResponse>, Status> {
        // Check access control
        let _user_id = self.access_control.check_access(&request, &Permission::ListProviders)
            .map_err(|e| Status::permission_denied(e.to_string()))?;
        
        let req = request.into_inner();
        
        let limit = if req.limit > 0 { req.limit } else { 100 };
        let offset = if req.offset >= 0 { req.offset } else { 0 };
        
        let result = self.registry
            .list_providers(limit, offset)
            .await
            .map_err(|e| Status::internal(format!("Failed to list providers: {}", e)))?;

        let provider_infos: Vec<nornen::ProviderInfo> = result
            .providers
            .into_iter()
            .map(|p| {
                let metadata_map: HashMap<String, String> = if let Some(obj) = p.metadata.as_object() {
                    obj.iter()
                        .filter_map(|(k, v)| {
                            v.as_str().map(|s| (k.clone(), s.to_string()))
                        })
                        .collect()
                } else {
                    HashMap::new()
                };
                
                nornen::ProviderInfo {
                    provider_id: p.provider_id,
                    name: p.name,
                    capabilities: p.capabilities,
                    endpoint: p.endpoint,
                    status: p.status,
                    metadata: metadata_map,
                }
            })
            .collect();

        Ok(Response::new(nornen::ListProvidersResponse {
            providers: provider_infos,
            total: result.total,
        }))
    }
}

// Verdandi Service Implementation
pub struct VerdandiServiceImpl {
    router: Arc<crate::verdandi::router::RequestRouter>,
    access_control: Arc<AccessControl>,
}

impl VerdandiServiceImpl {
    pub fn new(router: Arc<crate::verdandi::router::RequestRouter>, access_control: Arc<AccessControl>) -> Self {
        Self { router, access_control }
    }
}

#[tonic::async_trait]
impl VerdandiService for VerdandiServiceImpl {
    async fn route_request(
        &self,
        request: Request<nornen::RouteRequestRequest>,
    ) -> Result<Response<nornen::RouteRequestResponse>, Status> {
        let req = request.into_inner();
        
        let capabilities: Vec<String> = req.required_capabilities.into_iter().collect();
        let preferences: HashMap<String, String> = req.context.into_iter().collect();
        
        // Use select_provider to get provider with score for confidence calculation
        let (provider_id, endpoint, score) = self.router
            .select_provider(&capabilities, &preferences)
            .await
            .map_err(|e| Status::internal(format!("Failed to route request: {}", e)))?;

        // Calculate confidence from score (score is already normalized to 0.0-1.0)
        let confidence = score.min(1.0).max(0.0);
        
        Ok(Response::new(nornen::RouteRequestResponse {
            provider_id,
            endpoint,
            confidence,
        }))
    }

    async fn select_provider(
        &self,
        request: Request<nornen::SelectProviderRequest>,
    ) -> Result<Response<nornen::SelectProviderResponse>, Status> {
        let req = request.into_inner();
        
        let capabilities: Vec<String> = req.required_capabilities.into_iter().collect();
        let preferences: HashMap<String, String> = req.preferences.into_iter().collect();
        
        let (provider_id, endpoint, score) = self.router
            .select_provider(&capabilities, &preferences)
            .await
            .map_err(|e| Status::internal(format!("Failed to select provider: {}", e)))?;

        Ok(Response::new(nornen::SelectProviderResponse {
            provider_id,
            endpoint,
            score,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub coordinator: Arc<crate::coordinator::NornenCoordinator>,
    pub registry: Arc<crate::urd::registry::ProviderRegistry>,
    pub router: Arc<crate::verdandi::router::RequestRouter>,
    pub access_control: Arc<AccessControl>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Nornen gRPC server on {}", addr);

    let nornen_service = NornenServiceImpl::new(deps.coordinator.clone(), deps.access_control.clone());
    let urd_service = UrdServiceImpl::new(deps.registry.clone(), deps.access_control.clone());
    let verdandi_service = VerdandiServiceImpl::new(deps.router.clone(), deps.access_control.clone());

    Server::builder()
        .add_service(NornenServiceServer::new(nornen_service))
        .add_service(UrdServiceServer::new(urd_service))
        .add_service(VerdandiServiceServer::new(verdandi_service))
        .serve(addr)
        .await?;

    Ok(())
}
