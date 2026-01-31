use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod nidhoggr {
    tonic::include_proto!("nidhoggr");
}

use nidhoggr::nidhoggr_service_server::{NidhoggrService, NidhoggrServiceServer};

pub struct NidhoggrServiceImpl {
    endpoint_handler: Arc<crate::endpoint::EndpointHandler>,
}

impl NidhoggrServiceImpl {
    pub fn new(endpoint_handler: Arc<crate::endpoint::EndpointHandler>) -> Self {
        Self { endpoint_handler }
    }
}

#[tonic::async_trait]
impl NidhoggrService for NidhoggrServiceImpl {
    async fn establish_connection(
        &self,
        request: Request<nidhoggr::EstablishConnectionRequest>,
    ) -> Result<Response<nidhoggr::EstablishConnectionResponse>, Status> {
        let req = request.into_inner();
        
        let connection_id = self.endpoint_handler.handle_connection(&req.device_id, &req.endpoint_type).await
            .map_err(|e| Status::internal(format!("Connection establishment failed: {}", e)))?;

        Ok(Response::new(nidhoggr::EstablishConnectionResponse {
            connection_id,
        }))
    }

    async fn close_connection(
        &self,
        request: Request<nidhoggr::CloseConnectionRequest>,
    ) -> Result<Response<nidhoggr::CloseConnectionResponse>, Status> {
        let req = request.into_inner();
        
        // TODO: Close connection
        Ok(Response::new(nidhoggr::CloseConnectionResponse {
            success: true,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub endpoint_handler: Arc<crate::endpoint::EndpointHandler>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Nidhöggr gRPC server on {}", addr);

    let nidhoggr_service = NidhoggrServiceImpl::new(deps.endpoint_handler);

    Server::builder()
        .add_service(NidhoggrServiceServer::new(nidhoggr_service))
        .serve(addr)
        .await?;

    Ok(())
}
