//! Tests für Model-Health-Checker (Phase 6.2.1).

#[cfg(test)]
mod tests {
    use geri::model::{ModelHealthChecker, ModelHealthProbe, ModelHealthStatus};
    use geri::model::{ModelInfo, ModelRegistry, ModelType};
    use std::sync::atomic::{AtomicBool, Ordering};

    fn sample_llm() -> ModelInfo {
        ModelInfo {
            id: "gpt-4-001".to_string(),
            name: "GPT-4".to_string(),
            provider: "openai".to_string(),
            model_type: ModelType::Llm,
            parameter_count: Some(1_000_000_000),
            hardware_requirements: None,
            context_window: Some(8192),
        }
    }

    fn sample_vision() -> ModelInfo {
        ModelInfo {
            id: "gpt-4v-001".to_string(),
            name: "GPT-4V".to_string(),
            provider: "openai".to_string(),
            model_type: ModelType::Vision,
            parameter_count: None,
            hardware_requirements: None,
            context_window: None,
        }
    }

    /// Probe that returns a configurable value (for tests).
    struct MockProbe {
        result: AtomicBool,
    }
    impl MockProbe {
        fn new(result: bool) -> Self {
            Self {
                result: AtomicBool::new(result),
            }
        }
        fn set_result(&self, result: bool) {
            self.result.store(result, Ordering::SeqCst);
        }
    }
    impl ModelHealthProbe for MockProbe {
        fn check(&self, _model_id: &str) -> bool {
            self.result.load(Ordering::SeqCst)
        }
    }

    /// Shared-bool probe so tests can change result after passing probe to checker.
    struct SharedBoolProbe(std::sync::Arc<AtomicBool>);
    impl ModelHealthProbe for SharedBoolProbe {
        fn check(&self, _model_id: &str) -> bool {
            self.0.load(Ordering::SeqCst)
        }
    }

    #[test]
    fn new_with_registry_and_probe() {
        let registry = ModelRegistry::default().register(sample_llm());
        let probe = MockProbe::new(true);
        let _checker = ModelHealthChecker::new(registry, Box::new(probe));
    }

    #[test]
    fn run_check_updates_availability() {
        let registry = ModelRegistry::default().register(sample_llm());
        let probe = MockProbe::new(true);
        let mut checker = ModelHealthChecker::new(registry, Box::new(probe));
        checker.run_check_all();
        assert_eq!(checker.get_availability("gpt-4-001"), Some(true));
    }

    #[test]
    fn run_check_reflects_probe_failure() {
        let registry = ModelRegistry::default().register(sample_llm());
        let probe = MockProbe::new(false);
        let mut checker = ModelHealthChecker::new(registry, Box::new(probe));
        checker.run_check_all();
        assert_eq!(checker.get_availability("gpt-4-001"), Some(false));
    }

    #[test]
    fn run_check_all_updates_all_models() {
        let registry = ModelRegistry::default()
            .register(sample_llm())
            .register(sample_vision());
        let probe = MockProbe::new(true);
        let mut checker = ModelHealthChecker::new(registry, Box::new(probe));
        checker.run_check_all();
        assert_eq!(checker.get_availability("gpt-4-001"), Some(true));
        assert_eq!(checker.get_availability("gpt-4v-001"), Some(true));
    }

    #[test]
    fn get_availability_returns_none_for_unknown_model() {
        let registry = ModelRegistry::default().register(sample_llm());
        let probe = MockProbe::new(true);
        let mut checker = ModelHealthChecker::new(registry, Box::new(probe));
        checker.run_check_all();
        assert_eq!(checker.get_availability("unknown"), None);
    }

    #[test]
    fn get_uptime_percentage_returns_none_when_no_checks_yet() {
        let registry = ModelRegistry::default().register(sample_llm());
        let probe = MockProbe::new(true);
        let checker = ModelHealthChecker::new(registry, Box::new(probe));
        assert_eq!(checker.get_uptime_percentage("gpt-4-001"), None);
    }

    #[test]
    fn get_uptime_percentage_after_several_checks() {
        let registry = ModelRegistry::default().register(sample_llm());
        let probe = MockProbe::new(true);
        let mut checker = ModelHealthChecker::new(registry, Box::new(probe));
        checker.run_check_all(); // 1 ok
        checker.run_check_all(); // 2 ok
        checker.run_check_all(); // 3 ok
        let pct = checker.get_uptime_percentage("gpt-4-001").unwrap();
        assert!((pct - 100.0).abs() < 0.01);
    }

    #[test]
    fn get_uptime_percentage_mixed_success_failure() {
        use std::sync::Arc;
        let registry = ModelRegistry::default().register(sample_llm());
        let result = Arc::new(AtomicBool::new(true));
        let probe = SharedBoolProbe(Arc::clone(&result));
        let mut checker = ModelHealthChecker::new(registry, Box::new(probe));
        checker.run_check_all(); // ok
        checker.run_check_all(); // ok
        checker.run_check_all(); // ok
        result.store(false, Ordering::SeqCst);
        checker.run_check_all(); // fail
        let pct = checker.get_uptime_percentage("gpt-4-001").unwrap();
        assert!((pct - 75.0).abs() < 0.01);
    }

    #[test]
    fn get_uptime_percentage_returns_none_for_unknown_model() {
        let registry = ModelRegistry::default().register(sample_llm());
        let probe = MockProbe::new(true);
        let mut checker = ModelHealthChecker::new(registry, Box::new(probe));
        checker.run_check_all();
        assert_eq!(checker.get_uptime_percentage("unknown"), None);
    }

    #[test]
    fn model_health_status_default_unknown() {
        let s = ModelHealthStatus::default();
        assert_eq!(s.available(), None);
        assert_eq!(s.uptime_percentage(), None);
    }
}
