use crate::evaluation::ModelEvaluation;
use crate::registry::ModelRegistry;
use std::sync::Arc;

pub struct ModelRequirements {
    pub max_size: Option<u64>,
    pub min_reliability: Option<f64>,
    pub max_latency_ms: Option<u64>,
}

impl ModelRequirements {
    /// Stabile Cache-Key-Darstellung für Model-Selection-Cache (Phase 8).
    pub fn cache_key(&self) -> String {
        format!(
            "{:?}_{:?}_{:?}",
            self.max_size, self.min_reliability, self.max_latency_ms
        )
    }
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

    /// Liefert das beste Modell für die Anforderungen. Evaluationen laufen parallel (Query-Optimization Phase 8).
    pub async fn select_best_model(&self, _requirements: ModelRequirements) -> Result<String, Box<dyn std::error::Error>> {
        let models = self.registry.list_models().await?;
        if models.is_empty() {
            return Err("No suitable model found".into());
        }

        let handles: Vec<_> = models
            .iter()
            .map(|name| {
                let evaluator = self.evaluator.clone();
                let name = name.clone();
                tokio::spawn(async move { evaluator.evaluate(&name).await })
            })
            .collect();

        let mut best_model = None;
        let mut best_score = 0.0f64;
        for join_handle in handles {
            let evaluation = join_handle.await.map_err(|e| format!("evaluation join: {}", e))??;
            if evaluation.total_score > best_score {
                best_score = evaluation.total_score;
                best_model = Some(evaluation.model_name);
            }
        }
        best_model.ok_or_else(|| "No suitable model found".into())
    }
}
