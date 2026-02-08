//! RemoteCommandHandler (Phase 5.1.1, TDD).

use super::error::RemoteCommandError;
use super::executor::ScriptExecutor;
use async_trait::async_trait;
use std::collections::HashMap;

/// Receives commands from controller, forwards to Loki, returns results.
pub struct RemoteCommandHandler<E> {
    executor: E,
}

impl<E> RemoteCommandHandler<E>
where
    E: ScriptExecutor,
{
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    /// Receive command from controller, forward to Loki, return result to controller.
    pub async fn handle_command(
        &mut self,
        script_id: &str,
        script_content: &str,
        script_type: &str,
        parameters: HashMap<String, String>,
    ) -> Result<String, RemoteCommandError> {
        self.executor
            .execute(script_id, script_content, script_type, parameters)
            .await
    }
}

#[async_trait]
impl ScriptExecutor for crate::grpc::LokiClient {
    async fn execute(
        &mut self,
        script_id: &str,
        script_content: &str,
        script_type: &str,
        parameters: HashMap<String, String>,
    ) -> Result<String, RemoteCommandError> {
        self.call_function(script_id, script_content, script_type, parameters)
            .await
            .map_err(Into::into)
    }
}
