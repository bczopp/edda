//! Integration tests for Geri gRPC server (Phase 2.2).
//! Tests handler logic without binding to a port (container-friendly).

use geri::grpc::geri::geri_service_server::GeriService;
use geri::grpc::geri::{ProcessPromptRequest, ProcessVisionRequest};
use geri::grpc::GeriServiceImpl;
use geri::llm::LocalLLMProvider;
use geri::vision::VisionProcessor;
use geri::model::ModelRegistry;
use std::sync::Arc;
use tonic::Request;

#[tokio::test]
async fn test_process_prompt_returns_response_with_text_and_model() {
    let registry = Arc::new(ModelRegistry::new());
    let llm = Arc::new(LocalLLMProvider::new("test-model".to_string()));
    let vision = Arc::new(VisionProcessor::new("test-vision".to_string()));
    let svc = GeriServiceImpl::new(registry, llm.clone(), vision);

    let req = Request::new(ProcessPromptRequest {
        prompt: "Hello".to_string(),
        context: String::new(),
        model_name: String::new(),
        max_tokens: 0,
    });

    let res = GeriService::process_prompt(&svc, req).await.expect("process_prompt");
    let out = res.into_inner();
    assert!(!out.text.is_empty());
    assert_eq!(out.model_used, "test-model");
}

#[tokio::test]
async fn test_process_vision_returns_description_and_model() {
    let registry = Arc::new(ModelRegistry::new());
    let llm = Arc::new(LocalLLMProvider::new("test-llm".to_string()));
    let vision = Arc::new(VisionProcessor::new("test-vision".to_string()));
    let svc = GeriServiceImpl::new(registry, llm, vision);

    let req = Request::new(ProcessVisionRequest {
        image_data: vec![0u8; 10],
        prompt: String::new(),
        model_name: String::new(),
    });

    let res = GeriService::process_vision(&svc, req).await.expect("process_vision");
    let out = res.into_inner();
    assert!(!out.description.is_empty());
    assert_eq!(out.model_used, "test-vision");
}
