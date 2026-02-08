//! Tests für Fallback-Manager (Phase 10.1.1).

#[cfg(test)]
mod tests {
    use geri::fallback::{CloudLimitDetector, FallbackManager};
    use geri::model::{ModelInfo, ModelType};
    use geri::selection::{
        EfficiencyInput, EfficiencyScoreCalculator, ModelSelector, SelectionOptions,
    };

    fn local_model(id: &str) -> ModelInfo {
        ModelInfo {
            id: id.to_string(),
            name: id.to_string(),
            provider: "local".to_string(),
            model_type: ModelType::Llm,
            parameter_count: Some(8_000_000_000),
            hardware_requirements: None,
            context_window: Some(8192),
        }
    }

    fn local_input(score_relative: f64) -> EfficiencyInput {
        EfficiencyInput {
            parameter_count: Some(8_000_000_000),
            max_parameter_count: 70_000_000_000,
            hardware_score: score_relative,
            uptime_percentage: Some(100.0),
            error_rate: Some(0.0),
            ping_ms: Some(0),
            max_ping_ms: 1000,
            distance_km: Some(0.0),
            max_distance_km: 10_000.0,
            is_local: true,
            cost_per_token: Some(0.0),
            max_cost_per_token: 0.001,
        }
    }

    struct MockCloudLimitDetector(bool);
    impl CloudLimitDetector for MockCloudLimitDetector {
        fn is_cloud_limit(&self) -> bool {
            self.0
        }
    }

    #[test]
    fn get_fallback_returns_none_when_no_cloud_limit() {
        let selector = ModelSelector::new(EfficiencyScoreCalculator::default());
        let manager = FallbackManager::new(selector);
        let detector = MockCloudLimitDetector(false);
        let local_candidates = vec![
            (local_model("llama-1"), local_input(1.0)),
        ];
        assert!(manager
            .get_fallback_model(&local_candidates, &detector)
            .is_none());
    }

    #[test]
    fn get_fallback_returns_best_local_when_cloud_limit() {
        let selector = ModelSelector::new(EfficiencyScoreCalculator::default());
        let manager = FallbackManager::new(selector);
        let detector = MockCloudLimitDetector(true);
        let local_candidates = vec![
            (local_model("llama-1"), local_input(0.5)),
            (local_model("llama-2"), local_input(1.0)),
        ];
        let chosen = manager.get_fallback_model(&local_candidates, &detector);
        assert!(chosen.is_some());
        assert_eq!(chosen.unwrap().id, "llama-2");
    }

    #[test]
    fn get_fallback_returns_none_when_cloud_limit_but_no_local_candidates() {
        let selector = ModelSelector::new(EfficiencyScoreCalculator::default());
        let manager = FallbackManager::new(selector);
        let detector = MockCloudLimitDetector(true);
        let local_candidates: Vec<(ModelInfo, EfficiencyInput)> = vec![];
        assert!(manager
            .get_fallback_model(&local_candidates, &detector)
            .is_none());
    }

    #[test]
    fn get_fallback_uses_selector_for_single_local() {
        let selector = ModelSelector::new(EfficiencyScoreCalculator::default());
        let manager = FallbackManager::new(selector);
        let detector = MockCloudLimitDetector(true);
        let local_candidates = vec![(local_model("llama-1"), local_input(1.0))];
        let chosen = manager.get_fallback_model(&local_candidates, &detector);
        assert!(chosen.is_some());
        assert_eq!(chosen.unwrap().id, "llama-1");
    }
}
