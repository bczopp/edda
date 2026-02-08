//! Remote-Control-Example (Phase 12.2.1).
//! Demonstrates RemoteCommandHandler with a mock executor (no real Loki).
//! On device you would use LokiClient::connect(...) as the executor.

use jotunheim_esp32::remote::{RemoteCommandHandler, ScriptExecutor};
use std::collections::HashMap;

struct MockExecutor;
#[async_trait::async_trait]
impl ScriptExecutor for MockExecutor {
    async fn execute(
        &mut self,
        script_id: &str,
        _script_content: &str,
        _script_type: &str,
        _parameters: HashMap<String, String>,
    ) -> Result<String, jotunheim_esp32::remote::RemoteCommandError> {
        Ok(format!("mock result for script {}", script_id))
    }
}

#[tokio::main]
async fn main() {
    let mut handler = RemoteCommandHandler::new(MockExecutor);
    let params = HashMap::new();
    let res = handler
        .handle_command("led_on", "print('on')", "lua", params)
        .await;
    println!("Remote command result: {:?}", res);
}
