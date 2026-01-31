use serde::{Deserialize, Serialize};
use thiserror::Error;
use async_trait::async_trait;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContext {
    pub device_id: String,
    pub user_id: String,
    pub action_id: String,
}

#[derive(Debug, Error)]
pub enum ActionError {
    #[error("Action execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Invalid action: {0}")]
    InvalidAction(String),
    #[error("Action timeout")]
    Timeout,
}

#[async_trait]
pub trait ActionExecutor: Send + Sync {
    fn action_type(&self) -> &str;

    async fn execute(
        &self,
        context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError>;
}
