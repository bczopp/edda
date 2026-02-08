//! Tests für Load-Balancer (Phase 7.3.1).

#[cfg(test)]
mod tests {
    use geri::model::{ModelInfo, ModelType};
    use geri::selection::{
        EfficiencyInput, EfficiencyScoreCalculator, LoadBalancer,
    };

    fn model(id: &str, provider: &str) -> ModelInfo {
        ModelInfo {
            id: id.to_string(),
            name: id.to_string(),
            provider: provider.to_string(),
            model_type: ModelType::Llm,
            parameter_count: Some(8_000_000_000),
            hardware_requirements: None,
            context_window: Some(8192),
        }
    }

    fn same_input() -> EfficiencyInput {
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
            cost_per_token: Some(0.000_001),
            max_cost_per_token: 0.001,
        }
    }

    #[test]
    fn next_returns_model_when_candidates_not_empty() {
        let calc = EfficiencyScoreCalculator::default();
        let balancer = LoadBalancer::new(0.8, 100);
        let candidates = vec![
            (model("m1", "openai"), same_input()),
            (model("m2", "anthropic"), same_input()),
        ];
        let chosen = balancer.next(&candidates, &calc);
        assert!(chosen.is_some());
        assert!(candidates.iter().any(|(m, _)| m.id == chosen.as_ref().unwrap().id));
    }

    #[test]
    fn next_returns_none_when_no_candidates() {
        let calc = EfficiencyScoreCalculator::default();
        let balancer = LoadBalancer::new(0.8, 100);
        let candidates: Vec<(ModelInfo, EfficiencyInput)> = vec![];
        assert!(balancer.next(&candidates, &calc).is_none());
    }

    #[test]
    fn record_request_increases_load() {
        let mut balancer = LoadBalancer::new(0.8, 100);
        assert!(balancer.get_load("openai") < 0.01);
        for _ in 0..50 {
            balancer.record_request("openai");
        }
        assert!((balancer.get_load("openai") - 0.5).abs() < 0.01);
    }

    #[test]
    fn overloaded_provider_gets_lower_weight() {
        let calc = EfficiencyScoreCalculator::default();
        let mut balancer = LoadBalancer::new(0.8, 100);
        let candidates = vec![
            (model("openai-1", "openai"), same_input()),
            (model("anthropic-1", "anthropic"), same_input()),
        ];
        for _ in 0..90 {
            balancer.record_request("openai");
        }
        let chosen = balancer.next(&candidates, &calc);
        assert!(chosen.is_some());
        assert_eq!(chosen.unwrap().provider, "anthropic");
    }

    #[test]
    fn get_load_returns_zero_for_unknown_provider() {
        let balancer = LoadBalancer::new(0.8, 100);
        assert_eq!(balancer.get_load("unknown"), 0.0);
    }

    #[test]
    fn load_at_threshold_reduces_weight() {
        let calc = EfficiencyScoreCalculator::default();
        let mut balancer = LoadBalancer::new(0.8, 100);
        let candidates = vec![
            (model("a", "p1"), same_input()),
            (model("b", "p2"), same_input()),
        ];
        for _ in 0..80 {
            balancer.record_request("p1");
        }
        assert!((balancer.get_load("p1") - 0.8).abs() < 0.01);
        let chosen = balancer.next(&candidates, &calc);
        assert!(chosen.is_some());
        assert_eq!(chosen.unwrap().provider, "p2");
    }
}
