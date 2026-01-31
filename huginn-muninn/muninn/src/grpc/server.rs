use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod muninn {
    tonic::include_proto!("muninn");
}

use muninn::muninn_service_server::{MuninnService, MuninnServiceServer};

pub struct MuninnServiceImpl {
    tts_engine: Arc<crate::tts::TTSEngine>,
}

impl MuninnServiceImpl {
    pub fn new(tts_engine: Arc<crate::tts::TTSEngine>) -> Self {
        Self { tts_engine }
    }
}

#[tonic::async_trait]
impl MuninnService for MuninnServiceImpl {
    async fn synthesize_speech(
        &self,
        request: Request<muninn::SynthesizeSpeechRequest>,
    ) -> Result<Response<muninn::SynthesizeSpeechResponse>, Status> {
        let req = request.into_inner();
        
        let audio_data = self.tts_engine.synthesize(&req.text).await
            .map_err(|e| Status::internal(format!("TTS failed: {}", e)))?;

        Ok(Response::new(muninn::SynthesizeSpeechResponse {
            audio_data,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub tts_engine: Arc<crate::tts::TTSEngine>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Muninn gRPC server on {}", addr);

    let muninn_service = MuninnServiceImpl::new(deps.tts_engine);

    Server::builder()
        .add_service(MuninnServiceServer::new(muninn_service))
        .serve(addr)
        .await?;

    Ok(())
}
