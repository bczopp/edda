//! Tests für Model-Selection-Engine (Phase 7.2.1).

#[cfg(test)]
mod tests {
    use geri::model::{ModelInfo, ModelType};
    use geri::selection::{EfficiencyInput, EfficiencyScoreCalculator, ModelSelector, SelectionOptions};

    fn sample_llm(id: &str, name: &str) -> ModelInfo {
        ModelInfo {
            id: id.to_string(),
            name: name.to_string(),
            provider: "openai".to_string(),
            model_type: ModelType::Llm,
            parameter_count: Some(8_000_000_000),
            hardware_requirements: None,
            context_window: Some(8192),
        }
    }

    fn high_score_input() -> EfficiencyInput {
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

    fn low_score_input() -> EfficiencyInput {
        EfficiencyInput {
            parameter_count: Some(1_000_000_000),
            max_parameter_count: 70_000_000_000,
            hardware_score: 0.3,
            uptime_percentage: Some(50.0),
            error_rate: Some(0.2),
            ping_ms: Some(800),
            max_ping_ms: 1000,
            distance_km: Some(5000.0),
            max_distance_km: 10_000.0,
            is_local: false,
            cost_per_token: Some(0.0005),
            max_cost_per_token: 0.001,
        }
    }

    #[test]
    fn select_returns_user_preferred_when_set_and_in_candidates() {
        let calc = EfficiencyScoreCalculator::default();
        let selector = ModelSelector::new(calc);
        let candidates = vec![
            (sample_llm("gpt-4", "GPT-4"), high_score_input()),
            (sample_llm("gpt-3", "GPT-3"), low_score_input()),
        ];
        let options = SelectionOptions {
            user_preferred_model_id: Some("gpt-3".to_string()),
            max_latency_ms: None,
            max_cost_per_token: None,
        };
        let selected = selector.select(&candidates, &options);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().id, "gpt-3");
    }

    #[test]
    fn select_returns_best_by_score_when_no_user_preference() {
        let calc = EfficiencyScoreCalculator::default();
        let selector = ModelSelector::new(calc);
        let candidates = vec![
            (sample_llm("gpt-3", "GPT-3"), low_score_input()),
            (sample_llm("gpt-4", "GPT-4"), high_score_input()),
        ];
        let options = SelectionOptions::default();
        let selected = selector.select(&candidates, &options);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().id, "gpt-4");
    }

    #[test]
    fn select_filters_by_max_latency() {
        let calc = EfficiencyScoreCalculator::default();
        let selector = ModelSelector::new(calc);
        let mut slow_input = high_score_input();
        slow_input.ping_ms = Some(900);
        let candidates = vec![
            (sample_llm("fast", "Fast"), high_score_input()),
            (sample_llm("slow", "Slow"), slow_input),
        ];
        let options = SelectionOptions {
            user_preferred_model_id: None,
            max_latency_ms: Some(100),
            max_cost_per_token: None,
        };
        let selected = selector.select(&candidates, &options);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().id, "fast");
    }

    #[test]
    fn select_filters_by_max_cost() {
        let calc = EfficiencyScoreCalculator::default();
        let selector = ModelSelector::new(calc);
        let mut cheap = high_score_input();
        cheap.cost_per_token = Some(0.000_01);
        let mut expensive = high_score_input();
        expensive.cost_per_token = Some(0.001);
        let candidates = vec![
            (sample_llm("expensive", "Exp"), expensive),
            (sample_llm("cheap", "Cheap"), cheap),
        ];
        let options = SelectionOptions {
            user_preferred_model_id: None,
            max_latency_ms: None,
            max_cost_per_token: Some(0.0001),
        };
        let selected = selector.select(&candidates, &options);
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().id, "cheap");
    }

    #[test]
    fn select_returns_none_when_no_candidates() {
        let calc = EfficiencyScoreCalculator::default();
        let selector = ModelSelector::new(calc);
        let candidates: Vec<(ModelInfo, EfficiencyInput)> = vec![];
        let options = SelectionOptions::default();
        assert!(selector.select(&candidates, &options).is_none());
    }

    #[test]
    fn select_returns_none_when_user_preferred_not_in_candidates() {
        let calc = EfficiencyScoreCalculator::default();
        let selector = ModelSelector::new(calc);
        let candidates = vec![(sample_llm("gpt-4", "GPT-4"), high_score_input())];
        let options = SelectionOptions {
            user_preferred_model_id: Some("unknown".to_string()),
            max_latency_ms: None,
            max_cost_per_token: None,
        };
        assert!(selector.select(&candidates, &options).is_none());
    }

    #[test]
    fn select_returns_none_when_all_filtered_out_by_constraints() {
        let calc = EfficiencyScoreCalculator::default();
        let selector = ModelSelector::new(calc);
        let mut high_latency = high_score_input();
        high_latency.ping_ms = Some(900);
        let candidates = vec![(sample_llm("slow", "Slow"), high_latency)];
        let options = SelectionOptions {
            user_preferred_model_id: None,
            max_latency_ms: Some(50),
            max_cost_per_token: None,
        };
        assert!(selector.select(&candidates, &options).is_none());
    }
}
