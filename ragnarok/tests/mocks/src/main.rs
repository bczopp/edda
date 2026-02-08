//! Mock Odin gRPC service for Ragnarok E2E tests.
//! Implements OdinService::Process with a fixed response.

use std::env;
use tonic::{Request, Response, Status};
use tracing::{info, error};

pub mod odin {
    tonic::include_proto!("odin");
}

use odin::odin_service_server::{OdinService, OdinServiceServer};

struct MockOdinService;

#[tonic::async_trait]
impl OdinService for MockOdinService {
    async fn process(
        &self,
        request: Request<odin::ProcessRequest>,
    ) -> Result<Response<odin::ProcessResponse>, Status> {
        let req = request.into_inner();
        info!("Mock Odin Process: request_id={} input={}", req.request_id, req.input);
        Ok(Response::new(odin::ProcessResponse {
            response: format!("mock response for: {}", req.input),
            actions_taken: vec!["mock_action".to_string()],
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "odin".to_string());
    let port: u16 = env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "50050".to_string())
        .parse()
        .unwrap_or(50050);

    info!("Starting mock {} gRPC server on port {}", service_name, port);

    let addr = ([0, 0, 0, 0], port).into();
    let svc = MockOdinService;

    tonic::transport::Server::builder()
        .add_service(OdinServiceServer::new(svc))
        .serve(addr)
        .await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;

    Ok(())
}
