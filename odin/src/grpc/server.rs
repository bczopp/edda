use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::grpc::odin;
use odin::odin_service_server::{OdinService, OdinServiceServer};

pub struct OdinServiceImpl {
    request_processor: Arc<crate::orchestration::RequestProcessor>,
    action_orchestrator: Arc<crate::orchestration::ActionOrchestrator>,
}

impl OdinServiceImpl {
    pub fn new(
        request_processor: Arc<crate::orchestration::RequestProcessor>,
        action_orchestrator: Arc<crate::orchestration::ActionOrchestrator>,
    ) -> Self {
        Self {
            request_processor,
            action_orchestrator,
        }
    }
}

#[tonic::async_trait]
impl OdinService for OdinServiceImpl {
    async fn process(
        &self,
        request: Request<odin::ProcessRequest>,
    ) -> Result<Response<odin::ProcessResponse>, Status> {
        let req = request.into_inner();
        
        let user_request = crate::orchestration::UserRequest {
            request_id: req.request_id.clone(),
            user_id: req.user_id,
            device_id: req.device_id,
            input: req.input,
            input_type: req.input_type,
        };

        // Process request
        let response = self.request_processor.process(user_request).await
            .map_err(|e| Status::internal(format!("Request processing failed: {}", e)))?;

        // Generate action plan
        let action_plan = self.action_orchestrator.plan_actions(&response).await
            .map_err(|e| Status::internal(format!("Action planning failed: {}", e)))?;

        // Execute actions
        let actions_taken = self.action_orchestrator.execute_actions(action_plan).await
            .map_err(|e| Status::internal(format!("Action execution failed: {}", e)))?;

        Ok(Response::new(odin::ProcessResponse {
            response,
            actions_taken,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub request_processor: Arc<crate::orchestration::RequestProcessor>,
    pub action_orchestrator: Arc<crate::orchestration::ActionOrchestrator>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting Odin gRPC server on {}", addr);

    let odin_service = OdinServiceImpl::new(
        deps.request_processor,
        deps.action_orchestrator,
    );

    Server::builder()
        .add_service(OdinServiceServer::new(odin_service))
        .serve(addr)
        .await?;

    Ok(())
}
