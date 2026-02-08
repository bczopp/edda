//! E2E-Tests für LLM-Workflows (Phase 20.1.1).
//! WolfRequest→Response, RAG-Context-Integration, Cloud-Limit-Fallback, Model-Selection.

use geri::fallback::{CloudLimitDetector, FallbackManager};
use geri::model::{ModelInfo, ModelType};
use geri::selection::{
    EfficiencyInput, EfficiencyScoreCalculator, ModelSelector, SelectionOptions,
};

// --- WolfRequest → LLM-Call → WolfResponse (mit RAG-Context) ---

#[tokio::test]
async fn e2e_rag_context_integration_to_llm_response() {
    use geri::grpc::geri::geri_service_server::GeriService;
    use geri::grpc::geri::{ProcessPromptRequest, ProcessPromptResponse};
    use geri::grpc::GeriServiceImpl;
    use geri::llm::LocalLLMProvider;
    use geri::model::ModelRegistry;
    use geri::vision::VisionProcessor;
    use std::sync::Arc;
    use tonic::Request;

    let registry = Arc::new(ModelRegistry::new());
    let llm = Arc::new(LocalLLMProvider::new("llm-rag".to_string()));
    let vision = Arc::new(VisionProcessor::new("vision".to_string()));
    let svc = GeriServiceImpl::new(registry, llm, vision);

    let req = Request::new(ProcessPromptRequest {
        prompt: "Summarize the context.".to_string(),
        context: "Document 1: RAG context here.".to_string(),
        model_name: String::new(),
        max_tokens: 100,
    });

    let res: Result<tonic::Response<ProcessPromptResponse>, _> = GeriService::process_prompt(&svc, req).await;
    let out = res.expect("process_prompt").into_inner();
    assert!(!out.text.is_empty());
    assert_eq!(out.model_used, "llm-rag");
}

// --- Cloud-Limit-Fallback → Local-LLM ---

struct E2EMockCloudLimit(bool);
impl CloudLimitDetector for E2EMockCloudLimit {
    fn is_cloud_limit(&self) -> bool {
        self.0
    }
}

#[test]
fn e2e_cloud_limit_fallback_returns_local_model() {
    let selector = ModelSelector::new(EfficiencyScoreCalculator::default());
    let manager = FallbackManager::new(selector);
    let detector = E2EMockCloudLimit(true);
    let local_candidates = vec![
        (
            ModelInfo {
                id: "local-1".to_string(),
                name: "Local".to_string(),
                provider: "local".to_string(),
                model_type: ModelType::Llm,
                parameter_count: Some(8_000_000_000),
                hardware_requirements: None,
                context_window: Some(8192),
            },
            EfficiencyInput {
                parameter_count: Some(8_000_000_000),
                max_parameter_count: 70_000_000_000,
                hardware_score: 1.0,
                uptime_percentage: Some(100.0),
                error_rate: Some(0.0),
                ping_ms: Some(0),
                max_ping_ms: 1000,
                distance_km: Some(0.0),
                max_distance_km: 10_000.0,
                is_local: true,
                cost_per_token: Some(0.0),
                max_cost_per_token: 0.001,
            },
        ),
    ];
    let chosen = manager.get_fallback_model(&local_candidates, &detector);
    assert!(chosen.is_some());
    assert_eq!(chosen.unwrap().id, "local-1");
}

// --- Model-Selection (best by score) ---

#[test]
fn e2e_model_selection_selects_best_by_efficiency_score() {
    let calc = EfficiencyScoreCalculator::default();
    let selector = ModelSelector::new(calc);
    let candidates = vec![
        (
            ModelInfo {
                id: "low".to_string(),
                name: "Low".to_string(),
                provider: "p".to_string(),
                model_type: ModelType::Llm,
                parameter_count: Some(1_000_000_000),
                hardware_requirements: None,
                context_window: Some(4096),
            },
            EfficiencyInput {
                parameter_count: Some(1_000_000_000),
                max_parameter_count: 70_000_000_000,
                hardware_score: 0.3,
                uptime_percentage: Some(80.0),
                error_rate: Some(0.1),
                ping_ms: Some(200),
                max_ping_ms: 1000,
                distance_km: Some(1000.0),
                max_distance_km: 10_000.0,
                is_local: false,
                cost_per_token: Some(0.0001),
                max_cost_per_token: 0.001,
            },
        ),
        (
            ModelInfo {
                id: "high".to_string(),
                name: "High".to_string(),
                provider: "p".to_string(),
                model_type: ModelType::Llm,
                parameter_count: Some(70_000_000_000),
                hardware_requirements: None,
                context_window: Some(128_000),
            },
            EfficiencyInput {
                parameter_count: Some(70_000_000_000),
                max_parameter_count: 70_000_000_000,
                hardware_score: 1.0,
                uptime_percentage: Some(100.0),
                error_rate: Some(0.0),
                ping_ms: Some(10),
                max_ping_ms: 1000,
                distance_km: Some(0.0),
                max_distance_km: 10_000.0,
                is_local: true,
                cost_per_token: Some(0.00001),
                max_cost_per_token: 0.001,
            },
        ),
    ];
    let options = SelectionOptions::default();
    let selected = selector.select(&candidates, &options);
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().id, "high");
}
