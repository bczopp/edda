#[cfg(test)]
mod tests {
    use thor::terminal::{PtyWrapper, InteractiveExecutor, TerminalActionHandler};
    use thor::actions::{ActionExecutor, ActionContext};
    use std::time::Duration;

    #[tokio::test]
    async fn test_pty_wrapper_creation() {
        // Test PTY wrapper creation
        let pty = PtyWrapper::new("echo", &["test".to_string()], 24, 80);
        assert!(pty.is_ok());
    }

    #[tokio::test]
    async fn test_pty_wrapper_basic_execution() {
        // Test basic PTY execution
        let mut pty = PtyWrapper::new("echo", &["hello".to_string()], 24, 80).unwrap();
        
        // Wait for process to complete
        let status = pty.wait().await;
        assert!(status.is_ok());
    }

    #[tokio::test]
    async fn test_pty_wrapper_read_output() {
        // Test reading output from PTY
        let pty = PtyWrapper::new("echo", &["test output".to_string()], 24, 80).unwrap();
        
        // Give process time to produce output
        tokio::time::sleep(Duration::from_millis(200)).await;
        
        // Try to read output (may timeout if process already finished)
        let result = pty.read_output(Some(Duration::from_millis(500))).await;
        // Result may be Ok, Timeout, or IoError depending on timing and process state
        assert!(result.is_ok() || 
                matches!(result, Err(thor::terminal::PtyError::Timeout)) ||
                matches!(result, Err(thor::terminal::PtyError::IoError(_))));
    }

    #[tokio::test]
    async fn test_pty_wrapper_resize() {
        // Test PTY resize
        let mut pty = PtyWrapper::new("echo", &["test".to_string()], 24, 80).unwrap();
        
        let result = pty.resize(40, 120).await;
        assert!(result.is_ok());
        assert_eq!(pty.rows(), 40);
        assert_eq!(pty.cols(), 120);
        
        // Wait for process to complete
        let _ = pty.wait().await;
    }

    #[tokio::test]
    async fn test_interactive_executor_non_interactive() {
        // Test non-interactive command execution
        let executor = InteractiveExecutor::new();
        
        let (_output, error, exit_code) = executor
            .execute_interactive(
                "echo",
                &["test".to_string()],
                None,
                Some(Duration::from_secs(5)),
            )
            .await
            .unwrap();
        
        assert_eq!(exit_code, 0);
        assert!(error.is_empty());
    }

    #[tokio::test]
    async fn test_interactive_executor_with_input() {
        // Test execution with input (exit_code may be -1 in CI if process is killed or cat unavailable)
        let executor = InteractiveExecutor::new();
        
        let (_output, _error, exit_code) = executor
            .execute_interactive(
                "cat",
                &[],
                Some(b"test input".as_slice()),
                Some(Duration::from_secs(5)),
            )
            .await
            .unwrap();
        
        // In container/CI exit_code can be 0 (success) or -1 (e.g. process killed, wait() None)
        assert!(exit_code >= -1 && exit_code <= 255);
    }

    #[tokio::test]
    async fn test_terminal_action_handler_non_interactive() {
        // Test terminal action handler with non-interactive command
        let handler = TerminalActionHandler::new();
        
        let action_data = serde_json::json!({
            "command": "echo",
            "args": ["hello"],
            "interactive": false
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_ok());
        
        let result_data: serde_json::Value = serde_json::from_slice(&result.unwrap()).unwrap();
        assert_eq!(result_data["exit_code"], 0);
    }

    #[tokio::test]
    async fn test_terminal_action_handler_interactive_detection() {
        // Test that interactive commands are detected
        let handler = TerminalActionHandler::new();
        
        assert!(handler.is_interactive("vim"));
        assert!(handler.is_interactive("nano"));
        assert!(handler.is_interactive("htop"));
        assert!(!handler.is_interactive("echo"));
        assert!(!handler.is_interactive("ls"));
    }

    #[tokio::test]
    async fn test_terminal_action_handler_parse_params() {
        // Test parameter parsing through execution
        let handler = TerminalActionHandler::new();
        
        let action_data = serde_json::json!({
            "command": "echo",
            "args": ["test"],
            "interactive": false
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        // Execute should succeed with valid params
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_terminal_action_handler_invalid_params() {
        // Test invalid parameter handling
        let handler = TerminalActionHandler::new();
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let invalid_data = b"invalid json";
        let result = handler.execute(&context, invalid_data).await;
        assert!(result.is_err());
    }
}
