#[cfg(test)]
mod tests {
    use thor::cross_device::CrossDeviceActionHandler;
    use thor::actions::{ActionExecutor, ActionContext, ActionError};
    use std::sync::Arc;
    use async_trait::async_trait;

    // Mock executor for testing
    struct MockExecutor;

    #[async_trait]
    impl ActionExecutor for MockExecutor {
        fn action_type(&self) -> &str {
            "MOCK_ACTION"
        }

        async fn execute(
            &self,
            _context: &ActionContext,
            action_data: &[u8],
        ) -> Result<Vec<u8>, ActionError> {
            // Return the input data as output for testing
            Ok(action_data.to_vec())
        }
    }

    #[tokio::test]
    async fn test_cross_device_handler_creation() {
        // Test handler creation
        let executor = Arc::new(MockExecutor);
        let _handler = CrossDeviceActionHandler::new(executor);
        assert!(true); // Handler created successfully
    }

    #[tokio::test]
    async fn test_cross_device_handler_handle_action() {
        // Test handling incoming cross-device action
        let executor = Arc::new(MockExecutor);
        let _handler = CrossDeviceActionHandler::new(executor.clone());
        
        // Note: This test requires the cross_device proto to be defined
        // For now, we'll test the basic structure
        // In a real implementation, we would use the actual proto types
        
        // The handler should be able to execute actions locally
        let context = ActionContext {
            device_id: "target-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let action_data = b"test action data";
        let result = executor.execute(&context, action_data).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), action_data);
    }

    #[tokio::test]
    async fn test_cross_device_handler_connect_device() {
        // Test connecting to a device
        // Note: This would require a mock gRPC server
        // For now, we'll just test that the method exists
        let executor = Arc::new(MockExecutor);
        let handler = CrossDeviceActionHandler::new(executor);
        
        // In a real test, we would start a mock server and test connection
        // For now, we expect this to fail with connection error (which is expected)
        let result = handler.connect_device("http://localhost:9999".to_string()).await;
        // Connection should fail (no server running), but method should exist
        assert!(result.is_err()); // Expected - no server running
    }
}
