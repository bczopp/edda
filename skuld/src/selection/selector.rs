use crate::evaluation::ModelEvaluation;
use crate::registry::ModelRegistry;
use std::sync::Arc;

pub struct ModelRequirements {
    pub max_size: Option<u64>,
    pub min_reliability: Option<f64>,
    pub max_latency_ms: Option<u64>,
}

pub struct ModelSelector {
    pub registry: Arc<ModelRegistry>,
    pub evaluator: Arc<crate::evaluation::ModelEvaluator>,
}

impl ModelSelector {
    pub fn evaluator(&self) -> &Arc<crate::evaluation::ModelEvaluator> {
        &self.evaluator
    }
}

impl ModelSelector {
    pub fn new(registry: Arc<ModelRegistry>, evaluator: Arc<crate::evaluation::ModelEvaluator>) -> Self {
        Self { registry, evaluator }
    }

    pub async fn select_best_model(&self, _requirements: ModelRequirements) -> Result<String, Box<dyn std::error::Error>> {
        let models = self.registry.list_models().await?;
        
        let mut best_model = None;
        let mut best_score = 0.0;
        
        for model_name in models {
            let evaluation = self.evaluator.evaluate(&model_name).await?;
            if evaluation.total_score > best_score {
                best_score = evaluation.total_score;
                best_model = Some(model_name);
            }
        }
        
        best_model.ok_or_else(|| "No suitable model found".into())
    }
}
