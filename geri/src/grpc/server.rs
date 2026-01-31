use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod geri {
    tonic::include_proto!("geri");
}

use geri::geri_service_server::{GeriService, GeriServiceServer};

pub struct GeriServiceImpl {
    llm_provider: Arc<dyn crate::llm::LLMProvider>,
    vision_processor: Arc<crate::vision::VisionProcessor>,
}

impl GeriServiceImpl {
    pub fn new(
        llm_provider: Arc<dyn crate::llm::LLMProvider>,
        vision_processor: Arc<crate::vision::VisionProcessor>,
    ) -> Self {
        Self {
            llm_provider,
            vision_processor,
        }
    }
}

#[tonic::async_trait]
impl GeriService for GeriServiceImpl {
    async fn process_prompt(
        &self,
        request: Request<geri::ProcessPromptRequest>,
    ) -> Result<Response<geri::ProcessPromptResponse>, Status> {
        let req = request.into_inner();
        
        let prompt_request = crate::llm::PromptRequest {
            prompt: req.prompt,
            context: if req.context.is_empty() { None } else { Some(req.context) },
            max_tokens: if req.max_tokens == 0 { None } else { Some(req.max_tokens) },
        };

        let response = self.llm_provider.process_prompt(prompt_request).await
            .map_err(|e| Status::internal(format!("LLM processing failed: {}", e)))?;

        Ok(Response::new(geri::ProcessPromptResponse {
            text: response.text,
            tokens_used: response.tokens_used,
            model_name: self.llm_provider.model_name().to_string(),
        }))
    }

    async fn process_vision(
        &self,
        request: Request<geri::ProcessVisionRequest>,
    ) -> Result<Response<geri::ProcessVisionResponse>, Status> {
        let req = request.into_inner();
        
        let result = self.vision_processor.process_image(&req.image_data, &req.prompt).await
            .map_err(|e| Status::internal(format!("Vision processing failed: {}", e)))?;

        Ok(Response::new(geri::ProcessVisionResponse {
            description: result.description,
            extracted_text: result.extracted_text,
            metadata: result.metadata,
        }))
    }
}

pub struct GrpcServerDependencies {
    pub llm_provider: Arc<dyn crate::llm::LLMProvider>,
    pub vision_processor: Arc<crate::vision::VisionProcessor>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Geri gRPC server on {}", addr);

    let geri_service = GeriServiceImpl::new(
        deps.llm_provider,
        deps.vision_processor,
    );

    Server::builder()
        .add_service(GeriServiceServer::new(geri_service))
        .serve(addr)
        .await?;

    Ok(())
}
