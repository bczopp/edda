//! Tests für Model-Info und Model-Registry (Phase 6.1.1, 6.1.2).

#[cfg(test)]
mod tests {
    use geri::model::{ModelInfo, ModelRegistry, ModelType};

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

    #[test]
    fn model_info_has_required_fields() {
        let m = sample_llm();
        assert_eq!(m.id, "gpt-4-001");
        assert_eq!(m.name, "GPT-4");
        assert_eq!(m.provider, "openai");
        assert_eq!(m.model_type, ModelType::Llm);
    }

    #[test]
    fn register_and_get_by_id() {
        let registry = ModelRegistry::default();
        let m = sample_llm();
        let registry = registry.register(m.clone());
        let found = registry.get_by_id("gpt-4-001");
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, m.id);
    }

    #[test]
    fn get_by_id_returns_none_for_unknown() {
        let registry = ModelRegistry::default();
        assert!(registry.get_by_id("unknown").is_none());
    }

    #[test]
    fn list_all_returns_registered_models() {
        let registry = ModelRegistry::default()
            .register(sample_llm())
            .register(sample_vision());
        let all = registry.list_all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn filter_by_type_returns_matching() {
        let registry = ModelRegistry::default()
            .register(sample_llm())
            .register(sample_vision());
        let llms = registry.filter_by_type(ModelType::Llm);
        assert_eq!(llms.len(), 1);
        assert_eq!(llms[0].model_type, ModelType::Llm);
        let visions = registry.filter_by_type(ModelType::Vision);
        assert_eq!(visions.len(), 1);
    }

    #[test]
    fn filter_by_provider_returns_matching() {
        let registry = ModelRegistry::default()
            .register(sample_llm())
            .register(sample_vision());
        let openai = registry.filter_by_provider("openai");
        assert_eq!(openai.len(), 2);
    }

    #[test]
    fn unregister_removes_model() {
        let registry = ModelRegistry::default().register(sample_llm());
        let registry = registry.unregister("gpt-4-001");
        assert!(registry.get_by_id("gpt-4-001").is_none());
    }
}
