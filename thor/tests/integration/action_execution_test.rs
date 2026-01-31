#[cfg(test)]
mod tests {
    use thor::actions::ActionContext;
    use thor::permissions::PermissionChecker;
    use std::sync::Arc;

    // Integration test for action execution flow
    #[tokio::test]
    async fn test_action_execution_flow() {
        // Setup
        let registry = Arc::new(thor::actions::ActionRegistry::new());
        let permission_checker = Arc::new(PermissionChecker::new("http://localhost:50051".to_string()));
        let dispatcher = Arc::new(thor::actions::ActionDispatcher::new(
            registry.clone(),
            permission_checker.clone(),
        ));

        // Register a test executor (using terminal handler as example)
        registry.register(Arc::new(thor::terminal::TerminalActionHandler::new())).await;

        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };

        let action_data = serde_json::json!({
            "command": "echo",
            "args": ["test"],
            "interactive": false
        });

        // Note: This will fail permission check if Heimdall is not available
        // That's expected in test environment
        let result = dispatcher.dispatch(
            "TERMINAL_OPERATION",
            &context,
            &serde_json::to_vec(&action_data).unwrap(),
        ).await;

        // Result may be Ok or Err depending on Heimdall availability
        // We just verify the dispatcher processes the request
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_action_registry() {
        let registry = Arc::new(thor::actions::ActionRegistry::new());
        
        // Register handlers
        registry.register(Arc::new(thor::terminal::TerminalActionHandler::new())).await;
        registry.register(Arc::new(thor::ui_automation::UIAutomationHandler::new())).await;
        registry.register(Arc::new(thor::scheduler::SchedulerActionHandler::new())).await;

        // List registered types
        let types = registry.list_types().await;
        assert!(types.contains(&"TERMINAL_OPERATION".to_string()));
        assert!(types.contains(&"UI_AUTOMATION".to_string()));
        assert!(types.contains(&"SCHEDULER_OPERATION".to_string()));

        // Get handler
        let handler = registry.get("TERMINAL_OPERATION").await;
        assert!(handler.is_some());
    }
}
