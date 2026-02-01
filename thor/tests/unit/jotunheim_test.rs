#[cfg(test)]
mod tests {
    use thor::jotunheim::{JotunheimActionHandler, JotunheimClient};
    use thor::actions::{ActionExecutor, ActionContext};

    #[tokio::test]
    async fn test_jotunheim_handler_creation() {
        let handler = JotunheimActionHandler::new("http://127.0.0.1:38473".to_string());
        assert_eq!(handler.action_type(), "JOTUNHEIM_OPERATION");
    }

    /// Connect to Jotunheim: unreachable URL must return Err (no local Jotunheim required).
    #[tokio::test]
    async fn test_jotunheim_client_connect_unreachable() {
        let r = JotunheimClient::new("http://127.0.0.1:38473".to_string()).await;
        assert!(r.is_err(), "connect to unreachable host must fail");
    }

    /// Handler with unreachable URL must return Err on execute (container-friendly).
    #[tokio::test]
    async fn test_jotunheim_handler_unreachable_returns_err() {
        let handler = JotunheimActionHandler::new("http://127.0.0.1:38473".to_string());
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
        assert!(result.is_err(), "unreachable Jotunheim URL must yield ExecutionFailed");
    }

    #[tokio::test]
    async fn test_jotunheim_handler_invalid_operation_returns_err() {
        let handler = JotunheimActionHandler::new("http://127.0.0.1:38473".to_string());
        let action_data = serde_json::json!({
            "operation": "unknown_op",
            "device_id": "test-device-123"
        });
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_jotunheim_handler_missing_operation_returns_err() {
        let handler = JotunheimActionHandler::new("http://127.0.0.1:38473".to_string());
        let action_data = serde_json::json!({ "device_id": "test-device-123" });
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_jotunheim_device_command() {
        let handler = JotunheimActionHandler::new("http://127.0.0.1:38473".to_string());
        let action_data = serde_json::json!({
            "operation": "device_command",
            "device_id": "test-device-123",
            "command": "toggle_light",
            "parameters": { "light_id": "1", "state": "on" }
        });
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_err(), "no Jotunheim service; expect connection error");
    }

    #[tokio::test]
    async fn test_jotunheim_device_status() {
        let handler = JotunheimActionHandler::new("http://127.0.0.1:38473".to_string());
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
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_jotunheim_tool_call() {
        let handler = JotunheimActionHandler::new("http://127.0.0.1:38473".to_string());
        let action_data = serde_json::json!({
            "operation": "tool_call",
            "device_id": "test-device-123",
            "tool_name": "RegisterScript",
            "parameters": { "script_name": "test_script", "script_code": "print('hello')" }
        });
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_err());
    }
}
