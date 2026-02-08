//! Unit tests for gRPC Model Registry endpoints

use geri::grpc::geri::geri_service_server::GeriService;
use geri::grpc::GeriServiceImpl;
use geri::model::{ModelRegistry, ModelInfo, ModelType};
use tonic::Request;
use std::sync::Arc;

fn proto_list_models_request(model_type: &str, provider: &str) -> geri::grpc::geri::ListModelsRequest {
    geri::grpc::geri::ListModelsRequest {
        model_type: model_type.to_string(),
        provider: provider.to_string(),
    }
}

#[tokio::test]
async fn test_list_models_empty() {
    let registry = Arc::new(ModelRegistry::new());
    let llm_provider = Arc::new(geri::llm::LocalLLMProvider::new("test-model".to_string()));
    let vision_processor = Arc::new(geri::vision::VisionProcessor::new("test-vision".to_string()));
    
    let service = GeriServiceImpl::new(registry, llm_provider, vision_processor);
    
    let request = Request::new(proto_list_models_request("", ""));
    
    let response = GeriService::list_models(&service, request).await.expect("list_models");
    assert_eq!(response.get_ref().models.len(), 0);
}

#[tokio::test]
async fn test_list_models_returns_registered() {
    let model = ModelInfo {
        id: "gpt-4".to_string(),
        name: "GPT-4".to_string(),
        provider: "openai".to_string(),
        model_type: ModelType::Llm,
        parameter_count: Some(1_750_000_000_000),
        hardware_requirements: Some("GPU recommended".to_string()),
        context_window: Some(8192),
    };
    let registry = Arc::new(ModelRegistry::new().register(model));
    
    let llm_provider = Arc::new(geri::llm::LocalLLMProvider::new("test-model".to_string()));
    let vision_processor = Arc::new(geri::vision::VisionProcessor::new("test-vision".to_string()));
    
    let service = GeriServiceImpl::new(registry, llm_provider, vision_processor);
    
    let request = Request::new(proto_list_models_request("", ""));
    
    let response = GeriService::list_models(&service, request).await.expect("list_models");
    assert_eq!(response.get_ref().models.len(), 1);
    assert_eq!(response.get_ref().models[0].id, "gpt-4");
    assert_eq!(response.get_ref().models[0].provider, "openai");
}

#[tokio::test]
async fn test_list_models_filter_by_type() {
    let llm_model = ModelInfo {
        id: "gpt-4".to_string(),
        name: "GPT-4".to_string(),
        provider: "openai".to_string(),
        model_type: ModelType::Llm,
        parameter_count: Some(1_750_000_000_000),
        hardware_requirements: Some("GPU recommended".to_string()),
        context_window: Some(8192),
    };
    let vision_model = ModelInfo {
        id: "gpt-4v".to_string(),
        name: "GPT-4 Vision".to_string(),
        provider: "openai".to_string(),
        model_type: ModelType::Vision,
        parameter_count: Some(1_750_000_000_000),
        hardware_requirements: Some("GPU recommended".to_string()),
        context_window: Some(8192),
    };
    let registry = Arc::new(ModelRegistry::new().register(llm_model).register(vision_model));
    
    let llm_provider = Arc::new(geri::llm::LocalLLMProvider::new("test-model".to_string()));
    let vision_processor = Arc::new(geri::vision::VisionProcessor::new("test-vision".to_string()));
    
    let service = GeriServiceImpl::new(registry, llm_provider, vision_processor);
    
    let request = Request::new(proto_list_models_request("llm", ""));
    
    let response = GeriService::list_models(&service, request).await.expect("list_models");
    assert_eq!(response.get_ref().models.len(), 1);
    assert_eq!(response.get_ref().models[0].id, "gpt-4");
}

#[tokio::test]
async fn test_get_model_info_success() {
    let model = ModelInfo {
        id: "claude-3-opus".to_string(),
        name: "Claude 3 Opus".to_string(),
        provider: "anthropic".to_string(),
        model_type: ModelType::Llm,
        parameter_count: Some(0),
        hardware_requirements: Some("Cloud".to_string()),
        context_window: Some(200000),
    };
    let registry = Arc::new(ModelRegistry::new().register(model));
    
    let llm_provider = Arc::new(geri::llm::LocalLLMProvider::new("test-model".to_string()));
    let vision_processor = Arc::new(geri::vision::VisionProcessor::new("test-vision".to_string()));
    
    let service = GeriServiceImpl::new(registry, llm_provider, vision_processor);
    
    let request = Request::new(geri::grpc::geri::GetModelInfoRequest {
        model_id: "claude-3-opus".to_string(),
    });
    
    let response = GeriService::get_model_info(&service, request).await.expect("get_model_info");
    let model_info = response.get_ref().model.as_ref().expect("model");
    assert_eq!(model_info.id, "claude-3-opus");
    assert_eq!(model_info.context_window, 200000);
}

#[tokio::test]
async fn test_get_model_info_not_found() {
    let registry = Arc::new(ModelRegistry::new());
    let llm_provider = Arc::new(geri::llm::LocalLLMProvider::new("test-model".to_string()));
    let vision_processor = Arc::new(geri::vision::VisionProcessor::new("test-vision".to_string()));
    
    let service = GeriServiceImpl::new(registry, llm_provider, vision_processor);
    
    let request = Request::new(geri::grpc::geri::GetModelInfoRequest {
        model_id: "nonexistent".to_string(),
    });
    
    let result = GeriService::get_model_info(&service, request).await;
    assert!(result.is_err());
}
