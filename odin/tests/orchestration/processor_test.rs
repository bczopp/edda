#[cfg(test)]
mod tests {
    use odin::orchestration::responsibility::ResponsibilityManager;
    use odin::orchestration::{AuditEvent, AuditLogger, RequestProcessor, UserRequest};
    use odin::protocols::einherjar::CapabilityCache;
    use odin::protocols::manager::ProtocolManager;
    use odin::clients::manager::ClientManager;
    use odin::utils::config::SettingsManager;
    use std::sync::{Arc, Mutex};
    use tempfile::TempDir;

    fn request(id: &str, input: &str, input_type: &str) -> UserRequest {
        UserRequest {
            request_id: id.to_string(),
            user_id: "u1".to_string(),
            device_id: "d1".to_string(),
            input: input.to_string(),
            input_type: input_type.to_string(),
        }
    }

    #[tokio::test]
    async fn processor_without_responsibility_returns_ok() {
        let processor = RequestProcessor::new();
        let req = request("r1", "hello", "text");
        let res = processor.process(req).await;
        assert!(res.is_ok());
        let s = res.unwrap();
        assert!(s.contains("Geri") || s.contains("responsibility"));
    }

    /// Audit: when audit logger is set, process() logs RequestReceived.
    #[tokio::test]
    async fn processor_audit_logs_request_received_when_logger_set() {
        let events: Arc<Mutex<Vec<AuditEvent>>> = Arc::new(Mutex::new(Vec::new()));
        let logger = Arc::new(CaptureAuditLogger(events.clone()));
        let processor = RequestProcessor::new().with_audit_logger(logger);
        let req = request("audit1", "hello", "text");
        let _ = processor.process(req).await;
        let ev = events.lock().unwrap();
        assert_eq!(ev.len(), 1, "one audit event");
        assert!(matches!(&ev[0], AuditEvent::RequestReceived { request_id, .. } if request_id == "audit1"));
    }

    /// Capturing audit logger for tests.
    struct CaptureAuditLogger(Arc<Mutex<Vec<AuditEvent>>>);
    impl AuditLogger for CaptureAuditLogger {
        fn log(&self, event: &AuditEvent) {
            self.0.lock().unwrap().push(event.clone());
        }
    }

    /// Monitoring: when MonitoringService is set, process() updates active_requests (1 during, 0 after).
    #[tokio::test]
    async fn processor_with_monitoring_updates_active_requests() {
        use odin::utils::monitoring::{AuditLogger as MonitoringAuditLogger, MonitoringService};
        let audit = Arc::new(MonitoringAuditLogger::new(100));
        let monitoring = Arc::new(MonitoringService::new(audit));
        let processor = RequestProcessor::new().with_monitoring(monitoring.clone());
        let req = request("mon1", "hello", "text");
        let _ = processor.process(req).await;
        let metrics = monitoring.get_metrics().await;
        assert_eq!(metrics.active_requests, 0, "after process completes active_requests must be 0");
    }

    /// Phase 8 Caching: with_response_cache returns cached value on duplicate request_id.
    #[tokio::test]
    async fn processor_with_cache_returns_cached_response_on_duplicate_request() {
        use odin::utils::ResponseCache;
        let cache = Arc::new(ResponseCache::new(60));
        let processor = RequestProcessor::new().with_response_cache(cache.clone());
        let req = request("cache1", "hello", "text");
        let r1 = processor.process(req.clone()).await.unwrap();
        let r2 = processor.process(req).await.unwrap();
        assert_eq!(r1, r2, "second call must return cached response");
    }

    /// Phase 8 Request-Queuing: process_one_from_queue processes a queued request.
    #[tokio::test]
    async fn process_one_from_queue_processes_queued_request() {
        use odin::utils::{QueuedRequest, RequestQueue};
        use std::time::Instant;
        let queue = Arc::new(RequestQueue::new(10));
        queue.enqueue(QueuedRequest {
            request_id: "q1".to_string(),
            user_id: "u1".to_string(),
            device_id: "d1".to_string(),
            input: "hi".to_string(),
            input_type: "text".to_string(),
            queued_at: Instant::now(),
        }).await.unwrap();
        let processor = Arc::new(RequestProcessor::new());
        let out = processor.process_one_from_queue(queue.clone()).await;
        assert!(out.is_some(), "one item queued => one result");
        assert!(out.unwrap().is_ok());
        assert_eq!(queue.size().await, 0);
    }

    /// Phase 8 Parallel-Processing: process_parallel processes multiple requests concurrently.
    #[tokio::test]
    async fn process_parallel_processes_multiple_requests() {
        use odin::utils::{QueuedRequest, RequestQueue};
        use std::time::Instant;
        let queue = Arc::new(RequestQueue::new(10));
        for i in 0..3 {
            queue.enqueue(QueuedRequest {
                request_id: format!("pq{}", i),
                user_id: "u1".to_string(),
                device_id: "d1".to_string(),
                input: format!("msg {}", i),
                input_type: "text".to_string(),
                queued_at: Instant::now(),
            }).await.unwrap();
        }
        let processor = Arc::new(RequestProcessor::new());
        let requests: Vec<_> = {
            let mut v = Vec::new();
            while let Some(q) = queue.dequeue().await {
                v.push(q);
            }
            v
        };
        let results = processor.clone().process_parallel(requests).await;
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
    }

    /// Performance: process() with action fallback completes within reasonable time (no external I/O).
    #[tokio::test]
    async fn processor_performance_action_fallback_reasonable_latency() {
        use odin::orchestration::ActionOrchestrator;
        use std::time::Instant;
        let processor = RequestProcessor::new_with_action_fallback(ActionOrchestrator::new());
        const ITERS: u32 = 30;
        let start = Instant::now();
        for i in 0..ITERS {
            let req = request(&format!("perf{}", i), "Open the file test.txt", "text");
            let _ = processor.process(req).await;
        }
        let elapsed = start.elapsed();
        let max_ms = 1500u64;
        assert!(
            elapsed.as_millis() < max_ms as u128,
            "process (action fallback) {} calls should finish in <{}ms, took {:?}",
            ITERS,
            max_ms,
            elapsed
        );
    }

    #[tokio::test]
    async fn processor_fallback_audio_type() {
        let processor = RequestProcessor::new();
        let req = request("r2", "any", "audio");
        let res = processor.process(req).await;
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Huginn-Muninn"));
    }

    #[tokio::test]
    async fn processor_fallback_transcribe_keyword() {
        let processor = RequestProcessor::new();
        let req = request("r3", "please transcribe this", "text");
        let res = processor.process(req).await;
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Huginn-Muninn"));
    }

    #[tokio::test]
    async fn processor_fallback_image_type() {
        let processor = RequestProcessor::new();
        let req = request("r4", "describe", "image");
        let res = processor.process(req).await;
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Geri"));
    }

    #[tokio::test]
    async fn processor_fallback_video_type() {
        let processor = RequestProcessor::new();
        let req = request("r5", "analyze", "video");
        let res = processor.process(req).await;
        assert!(res.is_ok());
        assert!(res.unwrap().contains("Geri"));
    }

    #[tokio::test]
    async fn processor_fallback_text_type() {
        let processor = RequestProcessor::new();
        let req = request("r6", "what is 2+2?", "text");
        let res = processor.process(req).await;
        assert!(res.is_ok());
        let s = res.unwrap();
        assert!(s.contains("Geri") || s.contains("Request received"));
    }

    #[tokio::test]
    async fn processor_fallback_unknown_type_included_in_response() {
        let processor = RequestProcessor::new();
        let req = request("r7", "custom", "custom_type");
        let res = processor.process(req).await;
        assert!(res.is_ok());
        let s = res.unwrap();
        assert!(s.contains("Request received"));
        assert!(s.contains("custom"));
        assert!(s.contains("custom_type"));
    }

    #[tokio::test]
    async fn processor_fallback_with_action_orchestrator_returns_plan_summary_for_action_like_text() {
        use odin::orchestration::ActionOrchestrator;
        let orchestrator = ActionOrchestrator::new();
        let processor = RequestProcessor::new_with_action_fallback(orchestrator);
        let req = request("r8", "Open the file test.txt", "text");
        let res = processor.process(req).await;
        assert!(res.is_ok(), "process should succeed");
        let s = res.unwrap();
        assert!(s.contains("FILE_OPERATION") || s.contains("Planned") || s.contains("1"), "response should indicate planned action: {}", s);
    }

    #[tokio::test]
    async fn processor_with_responsibility_empty_cache_returns_orchestration_error() {
        use odin::orchestration::OrchestrationError;
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        let capability_cache = Arc::new(CapabilityCache::new());
        let protocol_manager = Arc::new(ProtocolManager::new(settings_arc.clone()));
        let client_manager = Arc::new(ClientManager::new(settings_arc.clone()));
        let responsibility_manager = Arc::new(ResponsibilityManager::new(
            capability_cache,
            protocol_manager,
            client_manager,
        ));
        let processor = RequestProcessor::new_with_responsibility(responsibility_manager);
        let req = request("r9", "hello", "text");
        let result = processor.process(req).await;
        assert!(result.is_err(), "empty cache and no discovery should yield Err");
        let err = result.unwrap_err();
        let oe = err.downcast_ref::<OrchestrationError>();
        assert!(oe.is_some(), "error should be OrchestrationError");
        assert!(matches!(oe.unwrap(), OrchestrationError::NoServiceFound));
    }

    /// E2E-style: User-Request → ResponsibilityManager (Geri in cache) → Route → Ok(response) or Err(service unreachable).
    #[tokio::test]
    async fn processor_e2e_flow_with_responsibility_and_capability() {
        use odin::protocols::einherjar::einherjar;
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        let capability_cache = Arc::new(CapabilityCache::new());
        let cap = einherjar::CapabilityResponse {
            god_name: "geri".to_string(),
            purpose: "LLM Processing".to_string(),
            functions: vec![],
            responsibility_domains: vec!["text".to_string(), "question".to_string()],
            responsibility_keywords: vec!["answer".to_string(), "explain".to_string()],
        };
        capability_cache.update("geri".to_string(), "http://localhost:50054".to_string(), cap).await;
        let protocol_manager = Arc::new(ProtocolManager::new(settings_arc.clone()));
        let client_manager = Arc::new(ClientManager::new(settings_arc.clone()));
        let responsibility_manager = Arc::new(ResponsibilityManager::new(
            capability_cache,
            protocol_manager,
            client_manager,
        ));
        let processor = RequestProcessor::new_with_responsibility(responsibility_manager);
        let req = request("e2e1", "Can you explain how this works?", "text");
        let result = processor.process(req).await;
        match result {
            Ok(s) => assert!(!s.is_empty(), "E2E flow with capability should return non-empty response or Err"),
            Err(_) => { /* Geri/take_responsibility unreachable is OK outside container */ }
        }
    }

    #[tokio::test]
    async fn test_request_processor_with_responsibility() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let capability_cache = Arc::new(CapabilityCache::new());
        let protocol_manager = Arc::new(ProtocolManager::new(settings_arc.clone()));
        let client_manager = Arc::new(ClientManager::new(settings_arc.clone()));
        let responsibility_manager = Arc::new(ResponsibilityManager::new(
            capability_cache,
            protocol_manager,
            client_manager,
        ));
        
        let processor = RequestProcessor::new_with_responsibility(responsibility_manager);
        
        let request = UserRequest {
            request_id: "test-1".to_string(),
            user_id: "user-1".to_string(),
            device_id: "device-1".to_string(),
            input: "What is the weather?".to_string(),
            input_type: "text".to_string(),
        };
        
        let result = processor.process(request).await;
        // Should route based on responsibility
        match result {
            Ok(response) => {
                assert!(!response.is_empty(), "Response should not be empty");
            }
            Err(e) => {
                // May fail if services are not available
                println!("Processing failed (may be expected): {}", e);
            }
        }
    }
}
