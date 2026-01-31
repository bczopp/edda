use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod huginn {
    tonic::include_proto!("huginn");
}

use huginn::huginn_service_server::{HuginnService, HuginnServiceServer};

pub struct HuginnServiceImpl {
    stt_engine: Arc<crate::stt::STTEngine>,
    data_forwarder: Arc<crate::forwarding::DataForwarder>,
}

impl HuginnServiceImpl {
    pub fn new(
        stt_engine: Arc<crate::stt::STTEngine>,
        data_forwarder: Arc<crate::forwarding::DataForwarder>,
    ) -> Self {
        Self {
            stt_engine,
            data_forwarder,
        }
    }
}

#[tonic::async_trait]
impl HuginnService for HuginnServiceImpl {
    async fn transcribe_audio(
        &self,
        request: Request<huginn::TranscribeAudioRequest>,
    ) -> Result<Response<huginn::TranscribeAudioResponse>, Status> {
        let req = request.into_inner();
        
        let transcription = self.stt_engine.transcribe(&req.audio_data).await
            .map_err(|e| Status::internal(format!("STT failed: {}", e)))?;

        // Forward to Odin
        self.data_forwarder.forward_to_odin(transcription.as_bytes(), "text").await
            .map_err(|e| Status::internal(format!("Forwarding failed: {}", e)))?;

        Ok(Response::new(huginn::TranscribeAudioResponse {
            transcription,
        }))
    }

    async fn forward_data(
        &self,
        request: Request<huginn::ForwardDataRequest>,
    ) -> Result<Response<huginn::ForwardDataResponse>, Status> {
        let req = request.into_inner();
        
        self.data_forwarder.forward_to_odin(&req.data, &req.data_type).await
            .map_err(|e| Status::internal(format!("Forwarding failed: {}", e)))?;

        Ok(Response::new(huginn::ForwardDataResponse {
            success: true,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub stt_engine: Arc<crate::stt::STTEngine>,
    pub data_forwarder: Arc<crate::forwarding::DataForwarder>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Huginn gRPC server on {}", addr);

    let huginn_service = HuginnServiceImpl::new(
        deps.stt_engine,
        deps.data_forwarder,
    );

    Server::builder()
        .add_service(HuginnServiceServer::new(huginn_service))
        .serve(addr)
        .await?;

    Ok(())
}
