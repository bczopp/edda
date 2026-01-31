use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;

pub mod loki {
    tonic::include_proto!("loki");
}

use loki::loki_service_server::{LokiService, LokiServiceServer};

pub struct LokiServiceImpl {
    coordinator: Arc<crate::coordination::ServiceCoordinator>,
}

impl LokiServiceImpl {
    pub fn new(coordinator: Arc<crate::coordination::ServiceCoordinator>) -> Self {
        Self { coordinator }
    }
}

#[tonic::async_trait]
impl LokiService for LokiServiceImpl {
    async fn execute_script(
        &self,
        request: Request<loki::ExecuteScriptRequest>,
    ) -> Result<Response<loki::ExecuteScriptResponse>, Status> {
        let req = request.into_inner();
        
        // Execute script
        let result = self.coordinator.execute_script(&req.script_content).await
            .map_err(|e| Status::internal(format!("Script execution failed: {}", e)))?;

        Ok(Response::new(loki::ExecuteScriptResponse {
            success: true,
            output: result,
            error: String::new(),
        }))
    }
}

pub struct GrpcServerDependencies {
    pub coordinator: Arc<crate::coordination::ServiceCoordinator>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Loki gRPC server on {}", addr);

    let loki_service = LokiServiceImpl::new(deps.coordinator);

    Server::builder()
        .add_service(LokiServiceServer::new(loki_service))
        .serve(addr)
        .await?;

    Ok(())
}
