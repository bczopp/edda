//! Script executor abstraction (Phase 5.1.1) – for forwarding commands to Loki and testing.

use async_trait::async_trait;
use std::collections::HashMap;

use super::error::RemoteCommandError;

/// Executes a script (e.g. via Loki gRPC). Implemented by LokiClient and test mocks.
#[async_trait]
pub trait ScriptExecutor: Send + Sync {
    async fn execute(
        &mut self,
        script_id: &str,
        script_content: &str,
        script_type: &str,
        parameters: HashMap<String, String>,
    ) -> Result<String, RemoteCommandError>;
}
