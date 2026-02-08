// RemoteCommandHandler tests (Phase 5.1.1, TDD).

use jotunheim_esp32::remote::{RemoteCommandHandler, ScriptExecutor};
use std::collections::HashMap;

struct MockExecutor {
    result: Result<String, String>,
}

#[async_trait::async_trait]
impl ScriptExecutor for MockExecutor {
    async fn execute(
        &mut self,
        _script_id: &str,
        _script_content: &str,
        _script_type: &str,
        _parameters: HashMap<String, String>,
    ) -> Result<String, jotunheim_esp32::remote::RemoteCommandError> {
        self.result
            .clone()
            .map_err(jotunheim_esp32::remote::RemoteCommandError::ExecutionFailed)
    }
}

#[tokio::test]
async fn handle_command_forwards_to_executor_and_returns_result() {
    let executor = MockExecutor {
        result: Ok("done".to_string()),
    };
    let mut handler = RemoteCommandHandler::new(executor);
    let params = HashMap::new();
    let res = handler
        .handle_command("led_on", "print('on')", "lua", params)
        .await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "done");
}

#[tokio::test]
async fn handle_command_returns_error_when_executor_fails() {
    let executor = MockExecutor {
        result: Err("Loki unavailable".to_string()),
    };
    let mut handler = RemoteCommandHandler::new(executor);
    let params = HashMap::new();
    let res = handler
        .handle_command("x", "y", "lua", params)
        .await;
    assert!(res.is_err());
}
