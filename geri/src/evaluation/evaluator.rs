use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelEvaluation {
    pub model_name: String,
    pub score: f64,
    pub factors: EvaluationFactors,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationFactors {
    pub size_score: f64,
    pub hardware_score: f64,
    pub reliability_score: f64,
    pub latency_score: f64,
    pub distance_score: f64,
    pub cost_score: f64,
}

pub struct ModelEvaluator;

impl ModelEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&self, model_name: &str) -> ModelEvaluation {
        // Multi-factor evaluation implementation
        // Each factor is scored 0.0 to 1.0, then weighted and combined
        
        // Size score (smaller is better, normalized)
        let size_score = self.evaluate_size(model_name);
        
        // Hardware score (compatibility with available hardware)
        let hardware_score = self.evaluate_hardware_compatibility(model_name);
        
        // Reliability score (based on model stability, error rate)
        let reliability_score = self.evaluate_reliability(model_name);
        
        // Latency score (lower latency is better)
        let latency_score = self.evaluate_latency(model_name);
        
        // Distance score (local vs cloud - local is better for privacy)
        let distance_score = self.evaluate_distance(model_name);
        
        // Cost score (lower cost is better)
        let cost_score = self.evaluate_cost(model_name);
        
        // Weighted combination (all factors equally weighted for now)
        let weights = [0.15, 0.15, 0.20, 0.20, 0.15, 0.15];
        let score = size_score * weights[0]
            + hardware_score * weights[1]
            + reliability_score * weights[2]
            + latency_score * weights[3]
            + distance_score * weights[4]
            + cost_score * weights[5];
        
        ModelEvaluation {
            model_name: model_name.to_string(),
            score,
            factors: EvaluationFactors {
                size_score,
                hardware_score,
                reliability_score,
                latency_score,
                distance_score,
                cost_score,
            },
        }
    }

    fn evaluate_size(&self, model_name: &str) -> f64 {
        // Evaluate model size (smaller models score higher)
        // In real implementation, would query model registry for actual size
        match model_name {
            name if name.contains("8b") => 0.8,  // 8B models are good size
            name if name.contains("70b") => 0.3, // 70B models are large
            name if name.contains("1-bit") => 0.9, // 1-bit models are very efficient
            _ => 0.5, // Default
        }
    }

    fn evaluate_hardware_compatibility(&self, model_name: &str) -> f64 {
        // Evaluate hardware compatibility
        // In real implementation, would check available hardware
        match model_name {
            name if name.contains("1-bit") => 0.9, // 1-bit models work on more hardware
            name if name.contains("8b") => 0.7,   // 8B models need moderate hardware
            name if name.contains("70b") => 0.3,   // 70B models need powerful hardware
            _ => 0.5,
        }
    }

    fn evaluate_reliability(&self, model_name: &str) -> f64 {
        // Evaluate model reliability (based on known stable models)
        match model_name {
            name if name.contains("llama3") => 0.9,  // Llama 3 is very reliable
            name if name.contains("gpt-4") => 0.95, // GPT-4 is highly reliable
            name if name.contains("1-bit") => 0.7,  // 1-bit models are newer
            _ => 0.6, // Default
        }
    }

    fn evaluate_latency(&self, model_name: &str) -> f64 {
        // Evaluate latency (local models typically faster)
        match model_name {
            name if name.contains("local") || name.contains("llama") => 0.8, // Local is fast
            name if name.contains("1-bit") => 0.9, // 1-bit is very fast
            name if name.contains("gpt-4") => 0.6,  // Cloud has network latency
            _ => 0.5,
        }
    }

    fn evaluate_distance(&self, model_name: &str) -> f64 {
        // Evaluate distance (local = 1.0, cloud = 0.5)
        if model_name.contains("local") || model_name.contains("llama") {
            1.0 // Local models
        } else if model_name.contains("gpt") || model_name.contains("claude") {
            0.5 // Cloud models
        } else {
            0.7 // Unknown, assume hybrid
        }
    }

    fn evaluate_cost(&self, model_name: &str) -> f64 {
        // Evaluate cost (local = free, cloud = paid)
        if model_name.contains("local") || model_name.contains("llama") {
            1.0 // Free local models
        } else if model_name.contains("1-bit") {
            0.95 // Very efficient, almost free
        } else if model_name.contains("gpt-4") {
            0.4 // Expensive cloud models
        } else {
            0.6 // Moderate cost
        }
    }
}

impl Default for ModelEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
