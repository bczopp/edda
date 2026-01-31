//! Service-Discovery tests for ServiceLifecycleManager.
//! Keine externen Dependencies – lauffähig im Container.

#[cfg(test)]
mod tests {
    use odin::services::{ServiceInfo, ServiceRegistry, ServiceLifecycleManager};
    use std::sync::Arc;

    fn make_lifecycle_manager() -> ServiceLifecycleManager {
        let registry = Arc::new(ServiceRegistry::new());
        ServiceLifecycleManager::new(registry)
    }

    #[tokio::test]
    async fn lifecycle_start_service_fails_if_not_in_registry() {
        let mgr = make_lifecycle_manager();
        let res = mgr.start_service("thor").await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn lifecycle_start_service_ok_if_in_registry() {
        let registry = Arc::new(ServiceRegistry::new());
        registry
            .register(ServiceInfo {
                service_name: "thor".to_string(),
                service_url: "http://localhost:50052".to_string(),
                capabilities: vec![],
            })
            .await;
        let mgr = ServiceLifecycleManager::new(registry);
        let res = mgr.start_service("thor").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn lifecycle_stop_service_fails_if_not_in_registry() {
        let mgr = make_lifecycle_manager();
        let res = mgr.stop_service("thor").await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn lifecycle_stop_service_ok_if_in_registry() {
        let registry = Arc::new(ServiceRegistry::new());
        registry
            .register(ServiceInfo {
                service_name: "thor".to_string(),
                service_url: "http://localhost:50052".to_string(),
                capabilities: vec![],
            })
            .await;
        let mgr = ServiceLifecycleManager::new(registry);
        let res = mgr.stop_service("thor").await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn lifecycle_health_check_false_when_not_in_registry() {
        let mgr = make_lifecycle_manager();
        let res = mgr.health_check("thor").await;
        assert!(res.is_ok());
        assert!(!res.unwrap());
    }

    #[tokio::test]
    async fn lifecycle_health_check_true_when_in_registry() {
        let registry = Arc::new(ServiceRegistry::new());
        registry
            .register(ServiceInfo {
                service_name: "thor".to_string(),
                service_url: "http://localhost:50052".to_string(),
                capabilities: vec![],
            })
            .await;
        let mgr = ServiceLifecycleManager::new(registry);
        let res = mgr.health_check("thor").await;
        assert!(res.is_ok());
        assert!(res.unwrap());
    }

    /// Phase 3 Lifecycle: Health-Check via Netzwerk – false when nothing listens.
    #[tokio::test]
    async fn lifecycle_health_check_reachable_false_when_port_unreachable() {
        let registry = Arc::new(ServiceRegistry::new());
        registry
            .register(ServiceInfo {
                service_name: "unreachable".to_string(),
                service_url: "http://127.0.0.1:39999".to_string(),
                capabilities: vec![],
            })
            .await;
        let mgr = ServiceLifecycleManager::new(registry);
        let res = mgr.health_check_reachable("unreachable").await;
        assert!(res.is_ok());
        assert!(!res.unwrap(), "nothing listens on 39999");
    }

    /// Phase 3 Lifecycle: Health-Check via Netzwerk – true when TCP port is listening.
    #[tokio::test]
    async fn lifecycle_health_check_reachable_true_when_port_listening() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let registry = Arc::new(ServiceRegistry::new());
        registry
            .register(ServiceInfo {
                service_name: "listening".to_string(),
                service_url: format!("http://127.0.0.1:{}", port),
                capabilities: vec![],
            })
            .await;
        let mgr = ServiceLifecycleManager::new(registry);
        let res = mgr.health_check_reachable("listening").await;
        assert!(res.is_ok(), "health_check_reachable should succeed");
        assert!(res.unwrap(), "port is listening => reachable");
        drop(listener);
    }

    /// Phase 3 Lifecycle: echte Prozess-Start/Stop – start_service calls ProcessRunner when set.
    #[tokio::test]
    async fn lifecycle_start_service_calls_process_runner_when_set() {
        use odin::services::ProcessRunner;
        use std::sync::Mutex;
        let registry = Arc::new(ServiceRegistry::new());
        registry
            .register(ServiceInfo {
                service_name: "thor".to_string(),
                service_url: "http://localhost:50052".to_string(),
                capabilities: vec![],
            })
            .await;
        let started: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
        let started_clone = started.clone();
        struct MockRunner(Arc<Mutex<Vec<(String, String)>>>);
        impl ProcessRunner for MockRunner {
            fn start(&self, name: &str, service_url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                self.0.lock().unwrap().push((name.to_string(), service_url.to_string()));
                Ok(())
            }
            fn stop(&self, name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                self.0.lock().unwrap().push((name.to_string(), String::new()));
                Ok(())
            }
        }
        let runner = Arc::new(MockRunner(started_clone));
        let mgr = ServiceLifecycleManager::new(registry).with_process_runner(runner);
        let res = mgr.start_service("thor").await;
        assert!(res.is_ok());
        let calls = started.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, "thor");
        assert_eq!(calls[0].1, "http://localhost:50052");
    }

    /// Phase 3 Lifecycle: echte Prozess-Start/Stop – stop_service calls ProcessRunner when set.
    #[tokio::test]
    async fn lifecycle_stop_service_calls_process_runner_when_set() {
        use odin::services::ProcessRunner;
        use std::sync::Mutex;
        let registry = Arc::new(ServiceRegistry::new());
        registry
            .register(ServiceInfo {
                service_name: "thor".to_string(),
                service_url: "http://localhost:50052".to_string(),
                capabilities: vec![],
            })
            .await;
        let stopped: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let stopped_clone = stopped.clone();
        struct MockRunner(Arc<Mutex<Vec<String>>>);
        impl ProcessRunner for MockRunner {
            fn start(&self, _: &str, _: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                Ok(())
            }
            fn stop(&self, name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                self.0.lock().unwrap().push(name.to_string());
                Ok(())
            }
        }
        let runner = Arc::new(MockRunner(stopped_clone));
        let mgr = ServiceLifecycleManager::new(registry).with_process_runner(runner);
        let res = mgr.stop_service("thor").await;
        assert!(res.is_ok());
        let calls = stopped.lock().unwrap();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0], "thor");
    }
}
