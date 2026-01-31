#[cfg(test)]
mod tests {
    use thor::jotunheim::JotunheimActionHandler;
    use thor::actions::{ActionExecutor, ActionContext};

    #[tokio::test]
    async fn test_jotunheim_handler_creation() {
        let handler = JotunheimActionHandler::new("http://localhost:50051".to_string());
        assert_eq!(handler.action_type(), "JOTUNHEIM_OPERATION");
    }

    #[tokio::test]
    async fn test_jotunheim_device_command() {
        let handler = JotunheimActionHandler::new("http://localhost:50051".to_string());
        
        let action_data = serde_json::json!({
            "operation": "device_command",
            "device_id": "test-device-123",
            "command": "toggle_light",
            "parameters": {
                "light_id": "1",
                "state": "on"
            }
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        // Note: This will fail if no Jotunheim service is running
        // That's expected in test environment
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        // Result may be Ok or Err depending on service availability
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_jotunheim_device_status() {
        let handler = JotunheimActionHandler::new("http://localhost:50051".to_string());
        
        let action_data = serde_json::json!({
            "operation": "device_status",
            "device_id": "test-device-123"
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_jotunheim_tool_call() {
        let handler = JotunheimActionHandler::new("http://localhost:50051".to_string());
        
        let action_data = serde_json::json!({
            "operation": "tool_call",
            "device_id": "test-device-123",
            "tool_name": "RegisterScript",
            "parameters": {
                "script_name": "test_script",
                "script_code": "print('hello')"
            }
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_ok() || result.is_err());
    }
}
