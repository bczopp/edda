use tonic::{transport::Server, Request, Response, Status};
use tracing::info;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod geri {
    tonic::include_proto!("geri");
}

use geri::geri_service_server::{GeriService, GeriServiceServer};

pub struct GeriServiceImpl {
    model_registry: Arc<crate::model::ModelRegistry>,
    llm_provider: Arc<dyn crate::llm::LLMProvider>,
    vision_processor: Arc<crate::vision::VisionProcessor>,
}

impl GeriServiceImpl {
    pub fn new(
        model_registry: Arc<crate::model::ModelRegistry>,
        llm_provider: Arc<dyn crate::llm::LLMProvider>,
        vision_processor: Arc<crate::vision::VisionProcessor>,
    ) -> Self {
        Self {
            model_registry,
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
        
        let system_prompt = if req.system_prompt.is_empty() {
            "You are a helpful assistant in the Edda platform.".to_string()
        } else {
            req.system_prompt
        };

        // Mandatory XML Protocol Injection
        let injected_system = crate::prompt::inject_xml_protocol(&system_prompt);

        let prompt_request = crate::llm::PromptRequest {
            prompt: req.prompt,
            system_prompt: Some(injected_system),
            context: if req.context.is_empty() { None } else { Some(req.context) },
            max_tokens: if req.max_tokens == 0 { None } else { Some(req.max_tokens) },
        };

        let response = self.llm_provider.process_prompt(prompt_request).await
            .map_err(|e| Status::internal(format!("LLM processing failed: {}", e)))?;

        Ok(Response::new(geri::ProcessPromptResponse {
            text: response.text,
            tokens_used: response.tokens_used,
            model_used: self.llm_provider.model_name().to_string(),
        }))
    }

    async fn process_vision(
        &self,
        request: Request<geri::ProcessVisionRequest>,
    ) -> Result<Response<geri::ProcessVisionResponse>, Status> {
        let req = request.into_inner();
        let vision_req = crate::vision::VisionRequest {
            image_data: req.image_data,
            prompt: if req.prompt.is_empty() { None } else { Some(req.prompt) },
        };
        let result = self.vision_processor.process(vision_req).await
            .map_err(|e| Status::internal(format!("Vision processing failed: {}", e)))?;
        let analysis_data = serde_json::to_vec(&result.analysis)
            .map_err(|e| Status::internal(format!("Vision analysis serialization failed: {}", e)))?;
        Ok(Response::new(geri::ProcessVisionResponse {
            description: result.description,
            analysis_data,
            model_used: self.vision_processor.model_name().to_string(),
        }))
    }

    async fn list_models(
        &self,
        request: Request<geri::ListModelsRequest>,
    ) -> Result<Response<geri::ListModelsResponse>, Status> {
        let req = request.into_inner();
        
        // Get all models from registry
        let all_models = self.model_registry.list_all();
        
        // Apply filters if specified
        let filtered_models = if !req.model_type.is_empty() || !req.provider.is_empty() {
            all_models.into_iter().filter(|model| {
                let type_match = req.model_type.is_empty() || 
                    model.model_type.to_string().to_lowercase() == req.model_type.to_lowercase();
                let provider_match = req.provider.is_empty() || 
                    model.provider.to_lowercase() == req.provider.to_lowercase();
                type_match && provider_match
            }).collect()
        } else {
            all_models
        };
        
        // Convert to proto format
        let proto_models = filtered_models.into_iter().map(|model| {
            geri::ModelInfo {
                id: model.id.clone(),
                name: model.name.clone(),
                provider: model.provider.clone(),
                model_type: model.model_type.to_string(),
                parameter_count: model.parameter_count.unwrap_or(0),
                hardware_requirements: model.hardware_requirements.clone().unwrap_or_default(),
                context_window: model.context_window.unwrap_or(0),
            }
        }).collect();
        
        Ok(Response::new(geri::ListModelsResponse {
            models: proto_models,
        }))
    }

    async fn get_model_info(
        &self,
        request: Request<geri::GetModelInfoRequest>,
    ) -> Result<Response<geri::GetModelInfoResponse>, Status> {
        let req = request.into_inner();
        
        let model = self.model_registry
            .get_by_id(&req.model_id)
            .ok_or_else(|| Status::not_found(format!("Model '{}' not found", req.model_id)))?;
        
        let proto_model = geri::ModelInfo {
            id: model.id.clone(),
            name: model.name.clone(),
            provider: model.provider.clone(),
            model_type: model.model_type.to_string(),
            parameter_count: model.parameter_count.unwrap_or(0),
            hardware_requirements: model.hardware_requirements.clone().unwrap_or_default(),
            context_window: model.context_window.unwrap_or(0),
        };
        
        Ok(Response::new(geri::GetModelInfoResponse {
            model: Some(proto_model),
        }))
    }
}

pub struct GrpcServerDependencies {
    pub model_registry: Arc<crate::model::ModelRegistry>,
    pub llm_provider: Arc<dyn crate::llm::LLMProvider>,
    pub vision_processor: Arc<crate::vision::VisionProcessor>,
}

pub async fn start_grpc_server(
    addr: SocketAddr,
    deps: GrpcServerDependencies,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting Geri gRPC server on {}", addr);

    let geri_service = GeriServiceImpl::new(
        deps.model_registry,
        deps.llm_provider,
        deps.vision_processor,
    );

    Server::builder()
        .add_service(GeriServiceServer::new(geri_service))
        .serve(addr)
        .await?;

    Ok(())
}
