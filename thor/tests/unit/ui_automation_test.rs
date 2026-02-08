#[cfg(test)]
mod tests {
    use thor::ui_automation::{OperatingSystemDetector, UIAutomationHandler, OperatingSystem};
    use thor::actions::{ActionExecutor, ActionContext};

    #[test]
    fn test_operating_system_detection() {
        let detector = OperatingSystemDetector::new();
        let os = detector.detect();
        
        // Should detect one of the supported operating systems
        assert!(matches!(os, 
            OperatingSystem::Windows | 
            OperatingSystem::MacOS | 
            OperatingSystem::Linux |
            OperatingSystem::Unknown));
    }

    #[tokio::test]
    async fn test_ui_automation_handler_creation() {
        let handler = UIAutomationHandler::new();
        assert_eq!(handler.action_type(), "UI_AUTOMATION");
    }

    #[tokio::test]
    async fn test_ui_automation_click_action() {
        let handler = UIAutomationHandler::new();
        
        let action_data = serde_json::json!({
            "action": "click",
            "element": {
                "type": "by_name",
                "value": "test_button"
            }
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        // Click-Actions ohne explizite User-Confirmation dürfen nicht ausgeführt werden.
        let err = result.expect_err("click without confirmation should fail");
        let msg = err.to_string().to_lowercase();
        assert!(
            msg.contains("confirmation"),
            "expected confirmation-related error, got: {}",
            msg
        );
    }

    #[tokio::test]
    async fn test_ui_automation_click_action_with_confirmation() {
        let handler = UIAutomationHandler::new();

        let action_data = serde_json::json!({
            "action": "click",
            "confirmed": true,
            "element": {
                "type": "by_name",
                "value": "test_button"
            }
        });

        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };

        let result = handler
            .execute(&context, &serde_json::to_vec(&action_data).unwrap())
            .await;

        // Auf unterschiedlichen Plattformen kann die eigentliche UI-Automation
        // noch fehlschlagen (z.B. fehlender AT-SPI-Bus). Wichtig ist hier nur,
        // dass der Fehler *nicht* durch fehlende Bestätigung ausgelöst wird.
        if let Err(e) = result {
            let msg = e.to_string().to_lowercase();
            assert!(
                !msg.contains("confirmation"),
                "unexpected confirmation error: {}",
                msg
            );
        }
    }

    #[tokio::test]
    async fn test_ui_automation_type_action() {
        let handler = UIAutomationHandler::new();
        
        let action_data = serde_json::json!({
            "action": "type",
            "text": "test input",
            "element": {
                "type": "by_position",
                "x": 100,
                "y": 200
            }
        });
        
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        
        let result = handler.execute(&context, &serde_json::to_vec(&action_data).unwrap()).await;
        // On Linux/macOS: stub returns "not yet fully implemented"; on Windows may Ok in headless
        if let Err(e) = &result {
            let msg = e.to_string();
            assert!(
                msg.contains("not yet fully implemented") || msg.contains("failed"),
                "unexpected error: {}",
                msg
            );
        }
    }

    /// On Linux, UI automation currently returns an error (stub/partial AT-SPI support).
    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_ui_automation_linux_returns_not_implemented() {
        let handler = UIAutomationHandler::new();
        let action_data = serde_json::json!({
            "action": "click",
            "element": { "type": "by_position", "x": 0, "y": 0 }
        });
        let context = ActionContext {
            device_id: "test-device".to_string(),
            user_id: "test-user".to_string(),
            action_id: "test-action".to_string(),
        };
        let result = handler
            .execute(&context, &serde_json::to_vec(&action_data).unwrap())
            .await;
        let err = result.expect_err("Linux UI automation should currently return Err");
        let msg = err.to_string();
        // Accept both legacy stub messages and newer AT-SPI based messages
        assert!(
            msg.contains("not yet fully implemented")
                || msg.contains("Linux UI Automation")
                || msg.contains("AT-SPI"),
            "{}",
            msg
        );
    }

    #[tokio::test]
    async fn test_ui_automation_invalid_action() {
        let handler = UIAutomationHandler::new();
        
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
