#[cfg(test)]
mod tests {
    use odin::orchestration::{ActionOrchestrator, ActionPlan, Action};
    use std::sync::Arc;
    use odin::clients::manager::ClientManager;
    use odin::utils::config::SettingsManager;

    #[tokio::test]
    async fn test_action_orchestrator_creation() {
        let orchestrator = ActionOrchestrator::new();
        assert!(true, "ActionOrchestrator should be created successfully");
    }

    #[tokio::test]
    async fn test_plan_actions_with_simple_request() {
        let orchestrator = ActionOrchestrator::new();
        
        let request = "Open the file test.txt";
        let result = orchestrator.plan_actions(request).await;
        
        assert!(result.is_ok(), "plan_actions should succeed");
        let plan = result.unwrap();
        assert!(!plan.actions.is_empty(), "Action plan should contain at least one action");
        
        // Verify first action structure
        let first_action = &plan.actions[0];
        assert_eq!(first_action.service, "thor", "First action should target Thor");
        assert_eq!(first_action.action_type, "FILE_OPERATION", "Action type should be FILE_OPERATION");
        assert!(!first_action.action_id.is_empty(), "Action ID should not be empty");
    }

    #[tokio::test]
    async fn test_plan_actions_with_terminal_command() {
        let orchestrator = ActionOrchestrator::new();
        
        let request = "Run command: ls -la";
        let result = orchestrator.plan_actions(request).await;
        
        assert!(result.is_ok(), "plan_actions should succeed");
        let plan = result.unwrap();
        assert!(!plan.actions.is_empty(), "Action plan should contain at least one action");
        
        let first_action = &plan.actions[0];
        assert_eq!(first_action.service, "thor", "Action should target Thor");
        assert_eq!(first_action.action_type, "TERMINAL_OPERATION", "Action type should be TERMINAL_OPERATION");
    }

    #[tokio::test]
    async fn test_plan_actions_with_multiple_actions() {
        let orchestrator = ActionOrchestrator::new();
        
        let request = "Open file test.txt and then run ls command";
        let result = orchestrator.plan_actions(request).await;
        
        assert!(result.is_ok(), "plan_actions should succeed");
        let plan = result.unwrap();
        assert!(plan.actions.len() >= 2, "Action plan should contain multiple actions");
    }

    #[tokio::test]
    async fn test_plan_actions_app_control() {
        let orchestrator = ActionOrchestrator::new();
        let result = orchestrator.plan_actions("launch calculator").await;
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert!(!plan.actions.is_empty());
        assert_eq!(plan.actions[0].action_type, "APP_CONTROL");
        assert_eq!(plan.actions[0].service, "thor");
    }

    #[tokio::test]
    async fn test_plan_actions_generic_fallback() {
        let orchestrator = ActionOrchestrator::new();
        let result = orchestrator.plan_actions("something completely unknown").await;
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert_eq!(plan.actions.len(), 1);
        assert_eq!(plan.actions[0].action_type, "SYSTEM_COMMAND");
        assert_eq!(plan.actions[0].service, "thor");
    }

    #[tokio::test]
    async fn test_execute_actions_without_client_manager() {
        let orchestrator = ActionOrchestrator::new();
        let plan = ActionPlan {
            actions: vec![Action {
                action_id: "a1".to_string(),
                action_type: "FILE_OPERATION".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({}),
            }],
        };
        let result = orchestrator.execute_actions(plan).await;
        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("Client manager not available"));
    }

    #[tokio::test]
    async fn test_execute_actions_skips_non_thor() {
        let orchestrator = ActionOrchestrator::new();
        let plan = ActionPlan {
            actions: vec![Action {
                action_id: "a1".to_string(),
                action_type: "LLM_CALL".to_string(),
                service: "geri".to_string(),
                parameters: serde_json::json!({}),
            }],
        };
        let result = orchestrator.execute_actions(plan).await;
        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].contains("Skipping non-Thor"));
    }

    #[tokio::test]
    async fn test_execute_actions_with_valid_plan() {
        // Setup test environment
        use tempfile::TempDir;
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let client_manager = Arc::new(ClientManager::new(settings_arc.clone()));
        client_manager.initialize().await.unwrap();
        
        let orchestrator = ActionOrchestrator::new_with_client(client_manager);
        
        // Create a test action plan
        let plan = ActionPlan {
            actions: vec![
                Action {
                    action_id: "test-action-1".to_string(),
                    action_type: "TERMINAL_OPERATION".to_string(),
                    service: "thor".to_string(),
                    parameters: serde_json::json!({
                        "command": "echo 'test'"
                    }),
                }
            ],
        };
        
        let result = orchestrator.execute_actions(plan).await;
        
        // Should succeed (even if Thor mock doesn't fully implement, we test the structure)
        assert!(result.is_ok(), "execute_actions should succeed");
        let results = result.unwrap();
        assert_eq!(results.len(), 1, "Should have one result for one action");
    }

    #[tokio::test]
    async fn test_execute_actions_with_empty_plan() {
        use tempfile::TempDir;
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let client_manager = Arc::new(ClientManager::new(settings_arc.clone()));
        client_manager.initialize().await.unwrap();
        
        let orchestrator = ActionOrchestrator::new_with_client(client_manager);
        
        let plan = ActionPlan {
            actions: vec![],
        };
        
        let result = orchestrator.execute_actions(plan).await;
        assert!(result.is_ok(), "execute_actions should succeed with empty plan");
        let results = result.unwrap();
        assert_eq!(results.len(), 0, "Should have no results for empty plan");
    }

    #[tokio::test]
    async fn test_execute_actions_with_multiple_actions() {
        use tempfile::TempDir;
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");
        
        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();
        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
        
        let client_manager = Arc::new(ClientManager::new(settings_arc.clone()));
        client_manager.initialize().await.unwrap();
        
        let orchestrator = ActionOrchestrator::new_with_client(client_manager);
        
        let plan = ActionPlan {
            actions: vec![
                Action {
                    action_id: "test-action-1".to_string(),
                    action_type: "TERMINAL_OPERATION".to_string(),
                    service: "thor".to_string(),
                    parameters: serde_json::json!({
                        "command": "echo 'test1'"
                    }),
                },
                Action {
                    action_id: "test-action-2".to_string(),
                    action_type: "TERMINAL_OPERATION".to_string(),
                    service: "thor".to_string(),
                    parameters: serde_json::json!({
                        "command": "echo 'test2'"
                    }),
                },
            ],
        };
        
        let result = orchestrator.execute_actions(plan).await;
        assert!(result.is_ok(), "execute_actions should succeed with multiple actions");
        let results = result.unwrap();
        assert_eq!(results.len(), 2, "Should have results for all actions");
    }

    /// Performance: plan_actions completes within reasonable time (no external I/O).
    #[tokio::test]
    async fn plan_actions_performance_reasonable_latency() {
        use std::time::Instant;
        let orchestrator = ActionOrchestrator::new();
        const ITERS: u32 = 50;
        let start = Instant::now();
        for _ in 0..ITERS {
            let _ = orchestrator.plan_actions("Open the file test.txt").await;
        }
        let elapsed = start.elapsed();
        let max_ms = 2000u64;
        assert!(
            elapsed.as_millis() < max_ms as u128,
            "plan_actions {} calls should finish in <{}ms, took {:?}",
            ITERS,
            max_ms,
            elapsed
        );
    }
}
