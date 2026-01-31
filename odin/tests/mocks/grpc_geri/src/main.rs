//! Minimal gRPC mock for Geri: Einherjar GetCapabilities, Responsibility TakeResponsibility, Geri ProcessPrompt.
//! Used so Odin container E2E can get Ok(response) when routing to Geri.

use std::net::SocketAddr;
use tonic::{Request, Response, Status};
use tracing::info;

pub mod einherjar {
    tonic::include_proto!("einherjar");
}
pub mod responsibility {
    tonic::include_proto!("responsibility");
}
pub mod geri {
    tonic::include_proto!("geri");
}

use einherjar::einherjar_protocol_server::{EinherjarProtocol, EinherjarProtocolServer};
use einherjar::{CapabilityRequest, CapabilityResponse};
use responsibility::responsibility_service_server::{ResponsibilityService, ResponsibilityServiceServer};
use responsibility::{TakeResponsibilityRequest, TakeResponsibilityResponse};
use geri::geri_service_server::{GeriService, GeriServiceServer};
use geri::{ProcessPromptRequest, ProcessPromptResponse};

struct EinherjarImpl;
#[tonic::async_trait]
impl EinherjarProtocol for EinherjarImpl {
    async fn get_capabilities(
        &self,
        _req: Request<CapabilityRequest>,
    ) -> Result<Response<CapabilityResponse>, Status> {
        Ok(Response::new(CapabilityResponse {
            god_name: "geri".to_string(),
            purpose: "LLM Processing".to_string(),
            functions: vec![],
            responsibility_domains: vec!["text".to_string(), "question".to_string()],
            responsibility_keywords: vec!["answer".to_string(), "explain".to_string()],
        }))
    }
}

struct ResponsibilityImpl;
#[tonic::async_trait]
impl ResponsibilityService for ResponsibilityImpl {
    async fn take_responsibility(
        &self,
        _req: Request<TakeResponsibilityRequest>,
    ) -> Result<Response<TakeResponsibilityResponse>, Status> {
        Ok(Response::new(TakeResponsibilityResponse {
            accepted: true,
            message: "Accepted".to_string(),
        }))
    }
    async fn return_responsibility(
        &self,
        _req: Request<responsibility::ReturnResponsibilityRequest>,
    ) -> Result<Response<responsibility::ReturnResponsibilityResponse>, Status> {
        Ok(Response::new(responsibility::ReturnResponsibilityResponse {
            acknowledged: true,
            message: "OK".to_string(),
        }))
    }
    async fn reject_responsibility(
        &self,
        _req: Request<responsibility::RejectResponsibilityRequest>,
    ) -> Result<Response<responsibility::RejectResponsibilityResponse>, Status> {
        Ok(Response::new(responsibility::RejectResponsibilityResponse {
            acknowledged: true,
            message: "OK".to_string(),
        }))
    }
}

struct GeriImpl;
#[tonic::async_trait]
impl GeriService for GeriImpl {
    async fn process_prompt(
        &self,
        req: Request<ProcessPromptRequest>,
    ) -> Result<Response<ProcessPromptResponse>, Status> {
        let p = req.into_inner();
        Ok(Response::new(ProcessPromptResponse {
            text: format!("[mock-geri] {}", p.prompt),
            tokens_used: 0,
            model_used: "mock".to_string(),
        }))
    }
    async fn process_vision(
        &self,
        _req: Request<geri::ProcessVisionRequest>,
    ) -> Result<Response<geri::ProcessVisionResponse>, Status> {
        Ok(Response::new(geri::ProcessVisionResponse {
            description: "mock vision".to_string(),
            analysis_data: vec![],
            model_used: "mock".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let port: u16 = std::env::var("SERVICE_PORT").unwrap_or_else(|_| "50054".to_string()).parse().unwrap_or(50054);
    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    info!("grpc_geri mock listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(EinherjarProtocolServer::new(EinherjarImpl))
        .add_service(ResponsibilityServiceServer::new(ResponsibilityImpl))
        .add_service(GeriServiceServer::new(GeriImpl))
        .serve(addr)
        .await?;
    Ok(())
}
