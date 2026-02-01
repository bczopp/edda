use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEvaluation {
    pub model_name: String,
    pub total_score: f64,
    pub performance_score: f64,
    pub reliability_score: f64,
    pub efficiency_score: f64,
}

pub struct ModelEvaluator;

impl ModelEvaluator {
    pub async fn evaluate(&self, model_name: &str) -> Result<ModelEvaluation, Box<dyn std::error::Error>> {
        // Multi-factor evaluation implementation
        // In production, would query metrics from database (Eikthyrnir, Geri)
        
        // Evaluate performance (based on model characteristics)
        let performance_score = self.evaluate_performance(model_name);
        
        // Evaluate reliability (based on historical data)
        let reliability_score = self.evaluate_reliability(model_name);
        
        // Evaluate efficiency (size, speed, resource usage)
        let efficiency_score = self.evaluate_efficiency(model_name);
        
        // Weighted total score
        let total_score = performance_score * 0.4 + reliability_score * 0.4 + efficiency_score * 0.2;
        
        Ok(ModelEvaluation {
            model_name: model_name.to_string(),
            total_score,
            performance_score,
            reliability_score,
            efficiency_score,
        })
    }

    fn evaluate_performance(&self, model_name: &str) -> f64 {
        // Evaluate performance based on model type
        match model_name {
            name if name.contains("gpt-4") => 0.95,
            name if name.contains("llama3-70b") => 0.9,
            name if name.contains("llama3-8b") => 0.75,
            name if name.contains("1-bit") => 0.7, // 1-bit models trade some quality for efficiency
            _ => 0.6,
        }
    }

    fn evaluate_reliability(&self, model_name: &str) -> f64 {
        // Evaluate reliability (would query Eikthyrnir in production)
        match model_name {
            name if name.contains("gpt-4") => 0.95,
            name if name.contains("llama3") => 0.85,
            name if name.contains("local") => 0.8,
            _ => 0.7,
        }
    }

    fn evaluate_efficiency(&self, model_name: &str) -> f64 {
        // Evaluate efficiency (size, speed, resource usage)
        match model_name {
            name if name.contains("1-bit") => 0.95, // Very efficient
            name if name.contains("8b") => 0.8,
            name if name.contains("70b") => 0.4, // Large models
            name if name.contains("local") => 0.85, // Local is efficient
            _ => 0.6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_evaluate_returns_scores_in_range() {
        let evaluator = ModelEvaluator;
        let e = evaluator.evaluate("llama3-8b").await.unwrap();
        assert!(e.total_score >= 0.0 && e.total_score <= 1.0);
        assert!(e.performance_score >= 0.0 && e.performance_score <= 1.0);
        assert!(e.reliability_score >= 0.0 && e.reliability_score <= 1.0);
        assert!(e.efficiency_score >= 0.0 && e.efficiency_score <= 1.0);
        assert_eq!(e.model_name, "llama3-8b");
    }

    #[tokio::test]
    async fn test_evaluate_gpt4_scores_higher_than_default() {
        let evaluator = ModelEvaluator;
        let e_gpt = evaluator.evaluate("gpt-4").await.unwrap();
        let e_unknown = evaluator.evaluate("unknown-model").await.unwrap();
        assert!(e_gpt.total_score > e_unknown.total_score);
    }
}
