// Performance tests (Phase 13.2.1). Tool execution and basic timing.

use jotunheim_esp32::remote::{RemoteCommandHandler, ScriptExecutor};
use std::collections::HashMap;
use std::time::Instant;

struct FastExecutor;
#[async_trait::async_trait]
impl ScriptExecutor for FastExecutor {
    async fn execute(
        &mut self,
        _: &str,
        _: &str,
        _: &str,
        _: HashMap<String, String>,
    ) -> Result<String, jotunheim_esp32::remote::RemoteCommandError> {
        Ok("ok".to_string())
    }
}

#[tokio::test]
async fn tool_execution_completes_within_reasonable_time() {
    let mut handler = RemoteCommandHandler::new(FastExecutor);
    let start = Instant::now();
    let _ = handler
        .handle_command("test", "x", "lua", HashMap::new())
        .await;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() < 2000,
        "tool execution should complete in < 2s, took {:?}",
        elapsed
    );
}
