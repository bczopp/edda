#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use geri::model::{ModelDiscovery, ModelInfo, ModelRegistry, ModelType};

    use super::super::super::model::discovery::{DiscoveryError, EinherjarCapabilityClient};

    /// Simple mock Einherjar client for unit tests.
    struct MockEinherjarClient {
        result: Result<Vec<ModelInfo>, DiscoveryError>,
    }

    impl MockEinherjarClient {
        fn with_models(models: Vec<ModelInfo>) -> Self {
            Self {
                result: Ok(models),
            }
        }

        fn with_error(error: DiscoveryError) -> Self {
            Self { result: Err(error) }
        }
    }

    impl EinherjarCapabilityClient for MockEinherjarClient {
        fn discover_models(&self, _service_url: &str) -> Result<Vec<ModelInfo>, DiscoveryError> {
            self.result.clone()
        }
    }

    fn sample_model(id: &str) -> ModelInfo {
        ModelInfo {
            id: id.to_string(),
            name: format!("Model {}", id),
            provider: "remote".to_string(),
            model_type: ModelType::Llm,
            parameter_count: None,
            hardware_requirements: None,
            context_window: None,
        }
    }

    #[test]
    fn discover_and_register_registers_discovered_models() {
        let registry = ModelRegistry::default();
        let models = vec![sample_model("m1"), sample_model("m2")];
        let client = Arc::new(MockEinherjarClient::with_models(models));

        let discovery = ModelDiscovery::new(registry, client);
        let updated = discovery
            .discover_and_register("grpc://einherjar:50051")
            .unwrap();

        assert!(updated.get_by_id("m1").is_some());
        assert!(updated.get_by_id("m2").is_some());
    }

    #[test]
    fn discover_and_register_handles_client_error() {
        let registry = ModelRegistry::default();
        let client = Arc::new(MockEinherjarClient::with_error(
            DiscoveryError::ServiceUnavailable("down".to_string()),
        ));

        let discovery = ModelDiscovery::new(registry, client);
        let result = discovery.discover_and_register("grpc://einherjar:50051");

        assert!(matches!(
            result,
            Err(DiscoveryError::ServiceUnavailable(msg)) if msg == "down"
        ));
    }

    #[test]
    fn discover_and_register_preserves_existing_models() {
        let existing = sample_model("existing");
        let registry = ModelRegistry::default().register(existing.clone());

        let new_model = sample_model("new");
        let client = Arc::new(MockEinherjarClient::with_models(vec![new_model.clone()]));

        let discovery = ModelDiscovery::new(registry, client);
        let updated = discovery
            .discover_and_register("grpc://einherjar:50051")
            .unwrap();

        assert!(updated.get_by_id("existing").is_some());
        assert!(updated.get_by_id("new").is_some());
    }
}

