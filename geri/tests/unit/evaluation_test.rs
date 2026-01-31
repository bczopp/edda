#[cfg(test)]
mod tests {
    use geri::evaluation::ModelEvaluator;

    #[test]
    fn test_model_evaluation() {
        let evaluator = ModelEvaluator::new();
        
        let evaluation = evaluator.evaluate("llama3-8b");
        
        assert_eq!(evaluation.model_name, "llama3-8b");
        assert!(evaluation.score >= 0.0 && evaluation.score <= 1.0);
        assert!(evaluation.factors.size_score >= 0.0 && evaluation.factors.size_score <= 1.0);
        assert!(evaluation.factors.hardware_score >= 0.0 && evaluation.factors.hardware_score <= 1.0);
        assert!(evaluation.factors.reliability_score >= 0.0 && evaluation.factors.reliability_score <= 1.0);
        assert!(evaluation.factors.latency_score >= 0.0 && evaluation.factors.latency_score <= 1.0);
        assert!(evaluation.factors.distance_score >= 0.0 && evaluation.factors.distance_score <= 1.0);
        assert!(evaluation.factors.cost_score >= 0.0 && evaluation.factors.cost_score <= 1.0);
    }

    #[test]
    fn test_evaluation_comparison() {
        let evaluator = ModelEvaluator::new();
        
        let eval_local = evaluator.evaluate("llama3-8b-local");
        let eval_cloud = evaluator.evaluate("gpt-4");
        
        // Local models should generally score higher on distance and cost
        assert!(eval_local.factors.distance_score >= eval_cloud.factors.distance_score);
        assert!(eval_local.factors.cost_score >= eval_cloud.factors.cost_score);
    }

    #[test]
    fn test_one_bit_model_evaluation() {
        let evaluator = ModelEvaluator::new();
        
        let evaluation = evaluator.evaluate("llama3-8b-1bit");
        
        // 1-bit models should score high on size, hardware, and cost
        assert!(evaluation.factors.size_score > 0.7);
        assert!(evaluation.factors.hardware_score > 0.7);
        assert!(evaluation.factors.cost_score > 0.7);
    }
}
