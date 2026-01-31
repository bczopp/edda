//! Service-Discovery tests for ServiceRegistry.
//! Keine externen Dependencies – lauffähig im Container.

#[cfg(test)]
mod tests {
    use odin::services::{ServiceRegistry, ServiceInfo};
    use std::sync::Arc;

    fn make_registry() -> ServiceRegistry {
        ServiceRegistry::new()
    }

    #[tokio::test]
    async fn registry_register_and_get() {
        let registry = make_registry();
        let info = ServiceInfo {
            service_name: "thor".to_string(),
            service_url: "http://localhost:50052".to_string(),
            capabilities: vec!["FILE_OPERATION".to_string(), "APP_CONTROL".to_string()],
        };
        registry.register(info).await;
        let out = registry.get("thor").await;
        assert!(out.is_some());
        let out = out.unwrap();
        assert_eq!(out.service_name, "thor");
        assert_eq!(out.service_url, "http://localhost:50052");
        assert_eq!(out.capabilities.len(), 2);
    }

    #[tokio::test]
    async fn registry_get_returns_none_for_unknown() {
        let registry = make_registry();
        let out = registry.get("unknown").await;
        assert!(out.is_none());
    }

    #[tokio::test]
    async fn registry_list_empty_initially() {
        let registry = make_registry();
        let list = registry.list().await;
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn registry_list_after_register() {
        let registry = make_registry();
        registry
            .register(ServiceInfo {
                service_name: "thor".to_string(),
                service_url: "http://localhost:50052".to_string(),
                capabilities: vec![],
            })
            .await;
        registry
            .register(ServiceInfo {
                service_name: "geri".to_string(),
                service_url: "http://localhost:50054".to_string(),
                capabilities: vec![],
            })
            .await;
        let list = registry.list().await;
        assert_eq!(list.len(), 2);
        let names: Vec<&str> = list.iter().map(|s| s.service_name.as_str()).collect();
        assert!(names.contains(&"thor"));
        assert!(names.contains(&"geri"));
    }

    #[tokio::test]
    async fn registry_register_overwrites_same_name() {
        let registry = make_registry();
        registry
            .register(ServiceInfo {
                service_name: "thor".to_string(),
                service_url: "http://old:50052".to_string(),
                capabilities: vec![],
            })
            .await;
        registry
            .register(ServiceInfo {
                service_name: "thor".to_string(),
                service_url: "http://new:50052".to_string(),
                capabilities: vec!["X".to_string()],
            })
            .await;
        let out = registry.get("thor").await.unwrap();
        assert_eq!(out.service_url, "http://new:50052");
        assert_eq!(out.capabilities, vec!["X"]);
    }

    /// Phase 3 Einherjar-Protocol: register_from_capability registers service from CapabilityResponse.
    #[tokio::test]
    async fn registry_register_from_capability_registers_service_with_domains_and_keywords() {
        use odin::protocols::einherjar::einherjar::CapabilityResponse;
        let registry = make_registry();
        let response = CapabilityResponse {
            god_name: "geri".to_string(),
            purpose: "LLM".to_string(),
            functions: vec![],
            responsibility_domains: vec!["text".to_string(), "question".to_string()],
            responsibility_keywords: vec!["answer".to_string()],
        };
        registry
            .register_from_capability("http://localhost:50054", response)
            .await;
        let out = registry.get("geri").await.unwrap();
        assert_eq!(out.service_name, "geri");
        assert_eq!(out.service_url, "http://localhost:50054");
        assert!(out.capabilities.contains(&"text".to_string()));
        assert!(out.capabilities.contains(&"answer".to_string()));
    }
}
