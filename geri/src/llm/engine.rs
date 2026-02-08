use std::sync::Arc;
use crate::llm::{LLMProvider, LLMError, PromptRequest, PromptResponse};
use crate::model::{ModelInfo, ModelRegistryTrait};
use crate::selection::{ModelSelector, SelectionOptions, EfficiencyInput, EfficiencyScoreCalculator, EfficiencyWeights};
use crate::performance::PerformanceTracker;
use crate::fallback::{FallbackManager, CloudLimitDetector};
use crate::llm::ProviderFactory;
use crate::cost::{BudgetTracker, CostCalculator};

pub struct GeriEngine {
    registry: Arc<dyn ModelRegistryTrait>,
    performance_tracker: Arc<PerformanceTracker>,
    factory: Arc<ProviderFactory>,
    budget_tracker: Arc<BudgetTracker>,
    cost_calculator: CostCalculator,
    selector: ModelSelector,
}

impl GeriEngine {
    pub fn new(
        registry: Arc<dyn ModelRegistryTrait>,
        performance_tracker: Arc<PerformanceTracker>,
        factory: Arc<ProviderFactory>,
        budget_tracker: Arc<BudgetTracker>,
    ) -> Self {
        let calculator = EfficiencyScoreCalculator::new(EfficiencyWeights::default());
        let selector = ModelSelector::new(calculator);
        
        Self {
            registry,
            performance_tracker,
            factory,
            budget_tracker,
            cost_calculator: CostCalculator::default(),
            selector,
        }
    }

    pub async fn process(&self, request: PromptRequest, mut options: SelectionOptions) -> Result<PromptResponse, LLMError> {
        // 1. Get all candidates
        let models = self.registry.list_all().await
            .map_err(|e| LLMError::ProcessingFailed(e.to_string()))?;

        // 2. Initial selection
        let mut candidates = self.prepare_candidates(&models).await;
        
        // Check budget and adjust selection if needed
        let is_over_limit = self.budget_tracker.is_over_limit().await;
        
        if is_over_limit && options.user_preferred_model_id.is_none() {
            // Force local models if budget is exceeded
            tracing::warn!("Budget exceeded, filtering for local models");
            candidates.retain(|(m, _)| m.is_local);
        }

        let mut selected_model = self.selector.select(&candidates, &options)
            .ok_or_else(|| LLMError::ModelNotAvailable("No suitable model found (check budget/availability)".to_string()))?;

        // 3. Fallback logic (if selected is cloud but we're over budget - double check)
        if !selected_model.is_local && is_over_limit {
             let local_candidates: Vec<_> = candidates.iter().filter(|(m, _)| m.is_local).cloned().collect();
             if let Some(fallback) = self.selector.select(&local_candidates, &SelectionOptions::default()) {
                 tracing::info!("Falling back to local model: {}", fallback.id);
                 selected_model = fallback;
             } else {
                 return Err(LLMError::ModelNotAvailable("Budget exceeded and no local fallback available".to_string()));
             }
        }

        // 4. Execution
        let provider = self.factory.get(&selected_model.id).await
            .ok_or_else(|| LLMError::ModelNotAvailable(format!("Provider for model {} not registered", selected_model.id)))?;
        
        self.performance_tracker.record_request_start(&selected_model.provider, &selected_model.id).await;
        let start_time = std::time::Instant::now();
        
        let response = provider.process_prompt(request).await;
        
        let latency_ms = start_time.elapsed().as_millis() as u64;

        match &response {
            Ok(resp) => {
                // Track performance
                self.performance_tracker.record_request_success(
                    &selected_model.provider, &selected_model.id, latency_ms, resp.tokens_used as u64
                ).await;
                
                // Track cost
                let cost = self.cost_calculator.total_cost(
                    0, // We need to estimate input tokens or get them from provider
                    resp.tokens_used, 
                    &selected_model.provider, 
                    &selected_model.id
                );
                self.budget_tracker.add_usage(cost).await;
            }
            Err(e) => {
                self.performance_tracker.record_request_failure(
                    &selected_model.provider, &selected_model.id, &e.to_string()
                ).await;
            }
        }

        response
    }

    async fn prepare_candidates(&self, models: &[ModelInfo]) -> Vec<(ModelInfo, EfficiencyInput)> {
        let mut candidates = Vec::new();
        for model in models {
            let metrics = self.performance_tracker.get_metrics(&model.provider, &model.id).await;
            
            let input = EfficiencyInput {
                parameter_count: model.parameter_count,
                max_parameter_count: 175_000_000_000,
                hardware_score: if model.is_local { 1.0 } else { 0.5 },
                uptime_percentage: metrics.as_ref().map(|m| m.success_rate() * 100.0),
                error_rate: metrics.as_ref().map(|m| 1.0 - m.success_rate()),
                ping_ms: metrics.as_ref().map(|m| m.average_latency_ms() as u32),
                max_ping_ms: 2000,
                distance_km: if model.is_local { Some(0.0) } else { Some(500.0) },
                max_distance_km: 2000.0,
                is_local: model.is_local,
                cost_per_token: model.cost_per_token_input,
                max_cost_per_token: 0.0001,
            };
            candidates.push((model.clone(), input));
        }
        candidates
    }
}
