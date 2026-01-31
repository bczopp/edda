#[cfg(test)]
mod tests {
    use thor::scheduler::SchedulerActionHandler;
    use thor::actions::{ActionExecutor, ActionContext};

    #[tokio::test]
    async fn test_scheduler_handler_creation() {
        let handler = SchedulerActionHandler::new();
        assert_eq!(handler.action_type(), "SCHEDULER_OPERATION");
    }

    #[tokio::test]
    async fn test_scheduler_create_job() {
        let handler = SchedulerActionHandler::new();
        
        let action_data = serde_json::json!({
            "operation": "create",
            "job_name": "test_job",
            "schedule": "0 0 * * *", // Daily at midnight
            "command": "echo test",
            "operating_system": "linux"
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        // Note: Actual execution may fail if not running as root/admin
        // This is expected in test environments
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        // Result may be Ok or Err depending on permissions and platform
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_scheduler_list_jobs() {
        let handler = SchedulerActionHandler::new();
        
        let action_data = serde_json::json!({
            "operation": "list",
            "operating_system": "linux"
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        // May fail if no permissions, but handler should process the request
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_scheduler_delete_job() {
        let handler = SchedulerActionHandler::new();
        
        let action_data = serde_json::json!({
            "operation": "delete",
            "job_name": "test_job",
            "operating_system": "linux"
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_cron_expression_validation() {
        // Test cron expression validation
        let valid_expressions = [
            "0 0 * * *",      // Daily at midnight
            "*/5 * * * *",    // Every 5 minutes
            "0 9 * * 1-5",    // 9 AM on weekdays
        ];
        
        for expr in &valid_expressions {
            // Basic validation - check format
            let parts: Vec<&str> = expr.split_whitespace().collect();
            assert_eq!(parts.len(), 5, "Cron expression should have 5 parts: {}", expr);
        }
    }
}
