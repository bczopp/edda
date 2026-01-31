use crate::actions::{ActionExecutor, ActionContext, ActionError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCommandParams {
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
}

pub struct SystemCommandExecutor;

#[async_trait]
impl ActionExecutor for SystemCommandExecutor {
    fn action_type(&self) -> &str {
        "SYSTEM_COMMAND"
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let params: SystemCommandParams = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Invalid system command: {}", e)))?;

        let mut cmd = Command::new(&params.command);
        cmd.args(&params.args);
        
        if let Some(ref dir) = params.working_dir {
            cmd.current_dir(dir);
        }

        let output = cmd.output().await
            .map_err(|e| ActionError::ExecutionFailed(format!("Failed to execute command: {}", e)))?;

        let result = serde_json::json!({
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
        });

        Ok(serde_json::to_vec(&result).unwrap())
    }
}
