use geri::llm::{GeriEngine, PromptRequest, LLMProvider, PromptResponse, LLMError, ProviderFactory};
use geri::model::{ModelInfo, ModelType, ModelRegistryTrait};
use geri::performance::PerformanceTracker;
use geri::selection::SelectionOptions;
use async_trait::async_trait;

struct MockRegistry {
    models: Vec<ModelInfo>,
}

#[async_trait]
impl ModelRegistryTrait for MockRegistry {
    async fn register(&self, _model: ModelInfo) -> Result<(), sqlx::Error> { Ok(()) }
    async fn unregister(&self, _id: &str) -> Result<(), sqlx::Error> { Ok(()) }
    async fn get_by_id(&self, id: &str) -> Result<Option<ModelInfo>, sqlx::Error> { 
        Ok(self.models.iter().find(|m| m.id == id).cloned())
    }
    async fn list_all(&self) -> Result<Vec<ModelInfo>, sqlx::Error> { Ok(self.models.clone()) }
    async fn filter_by_type(&self, _type: ModelType) -> Result<Vec<ModelInfo>, sqlx::Error> { Ok(self.models.clone()) }
    async fn filter_by_provider(&self, _provider: &str) -> Result<Vec<ModelInfo>, sqlx::Error> { Ok(self.models.clone()) }
}

struct MockProvider {
    name: String,
}

#[async_trait]
impl LLMProvider for MockProvider {
    fn model_name(&self) -> &str { &self.name }
    async fn process_prompt(&self, request: PromptRequest) -> Result<PromptResponse, LLMError> {
        Ok(PromptResponse {
            text: format!("Mock response from {} for: {}", self.name, request.prompt),
            tokens_used: 10,
        })
    }
}

#[tokio::test]
async fn test_engine_orchestration() {
    let models = vec![
        ModelInfo {
            id: "gpt-4".to_string(),
            name: "GPT-4".to_string(),
            provider: "openai".to_string(),
            model_type: ModelType::Llm,
            parameter_count: None,
            hardware_requirements: None,
            context_window: Some(8192),
            is_local: false,
            cost_per_token_input: Some(0.00003),
            cost_per_token_output: Some(0.00006),
        },
    ];

    let registry = Arc::new(MockRegistry { models });
    let performance_tracker = Arc::new(PerformanceTracker::new().unwrap());
    let factory = Arc::new(ProviderFactory::new());
    let budget_tracker = Arc::new(geri::cost::BudgetTracker::new(10.0));
    
    factory.register("gpt-4", Arc::new(MockProvider { name: "gpt-4".to_string() })).await;
    
    let engine = GeriEngine::new(registry, performance_tracker, factory, budget_tracker);

    let request = PromptRequest {
        prompt: "Tell me a joke".to_string(),
        context: None,
        max_tokens: Some(50),
    };

    let result = engine.process(request, SelectionOptions::default()).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert!(response.text.contains("Mock response from gpt-4"));
}

#[tokio::test]
async fn test_engine_user_preference() {
    let models = vec![
        ModelInfo {
            id: "gpt-4".to_string(),
            name: "GPT-4".to_string(),
            provider: "openai".to_string(),
            model_type: ModelType::Llm,
            parameter_count: None,
            hardware_requirements: None,
            context_window: Some(8192),
            is_local: false,
            cost_per_token_input: Some(0.00003),
            cost_per_token_output: Some(0.00006),
        },
    ];

    let registry = Arc::new(MockRegistry { models });
    let performance_tracker = Arc::new(PerformanceTracker::new().unwrap());
    let factory = Arc::new(ProviderFactory::new());
    let budget_tracker = Arc::new(geri::cost::BudgetTracker::new(10.0));
    
    factory.register("gpt-4", Arc::new(MockProvider { name: "gpt-4".to_string() })).await;
    
    let engine = GeriEngine::new(registry, performance_tracker, factory, budget_tracker);

    let request = PromptRequest {
        prompt: "Hello".to_string(),
        context: None,
        max_tokens: None,
    };

    let options = SelectionOptions {
        user_preferred_model_id: Some("gpt-4".to_string()),
        ..Default::default()
    };

    let result = engine.process(request, options).await;
    assert!(result.is_ok());
    assert!(result.unwrap().text.contains("gpt-4"));
}

#[tokio::test]
async fn test_engine_fallback_on_budget_exceeded() {
    let models = vec![
        ModelInfo {
            id: "cloud-model".to_string(),
            name: "Cloud Model".to_string(),
            provider: "openai".to_string(),
            model_type: ModelType::Llm,
            parameter_count: None,
            hardware_requirements: None,
            context_window: Some(8192),
            is_local: false,
            cost_per_token_input: Some(1.0), // Expensive!
            cost_per_token_output: Some(1.0),
        },
        ModelInfo {
            id: "local-model".to_string(),
            name: "Local Model".to_string(),
            provider: "llamacpp".to_string(),
            model_type: ModelType::Llm,
            parameter_count: Some(8_000_000_000),
            hardware_requirements: None,
            context_window: Some(4096),
            is_local: true,
            cost_per_token_input: Some(0.0),
            cost_per_token_output: Some(0.0),
        },
    ];

    let registry = Arc::new(MockRegistry { models });
    let performance_tracker = Arc::new(PerformanceTracker::new().unwrap());
    let factory = Arc::new(ProviderFactory::new());
    
    // Low budget: 0.1
    let budget_tracker = Arc::new(geri::cost::BudgetTracker::new(0.1));
    
    factory.register("cloud-model", Arc::new(MockProvider { name: "cloud-model".to_string() })).await;
    factory.register("local-model", Arc::new(MockProvider { name: "local-model".to_string() })).await;
    
    let engine = GeriEngine::new(registry, performance_tracker, factory, budget_tracker.clone());

    // 1. Manually exceed budget
    budget_tracker.add_usage(1.0).await;
    
    let request = PromptRequest {
        prompt: "Hello".to_string(),
        context: None,
        max_tokens: None,
    };

    // 2. Process should choose local model despite no user preference
    let result = engine.process(request, SelectionOptions::default()).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    // It should choose local-model because cloud-model is restricted by budget
    assert!(response.text.contains("local-model"));
}
