use crate::actions::{ActionExecutor, ActionContext, ActionError};
use crate::terminal::InteractiveExecutor;
use std::sync::Arc;
use serde_json::Value;
use async_trait::async_trait;

/// Terminal action handler
pub struct TerminalActionHandler {
    executor: Arc<InteractiveExecutor>,
}

impl TerminalActionHandler {
    pub fn new() -> Self {
        Self {
            executor: Arc::new(InteractiveExecutor::new()),
        }
    }

    /// Check if a command is interactive (pub for tests)
    pub fn is_interactive(&self, command: &str) -> bool {
        let interactive_commands = ["vim", "nano", "htop", "top", "less", "more", "vi", "emacs"];
        interactive_commands.iter().any(|&cmd| command.contains(cmd))
    }

    /// Parse action parameters (public for testing)
    pub(crate) fn parse_params(&self, action_data: &[u8]) -> Result<TerminalParams, ActionError> {
        let value: Value = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Failed to parse action data: {}", e)))?;
        
        Ok(TerminalParams {
            command: value["command"]
                .as_str()
                .ok_or_else(|| ActionError::InvalidAction("Missing 'command' field".to_string()))?
                .to_string(),
            args: value["args"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                .unwrap_or_default(),
            input: value["input"].as_str().map(|s| s.as_bytes().to_vec()),
            timeout_seconds: value["timeout_seconds"].as_u64().map(|v| v as u64),
            interactive: value["interactive"].as_bool(),
        })
    }
}

#[derive(Debug)]
pub(crate) struct TerminalParams {
    command: String,
    args: Vec<String>,
    input: Option<Vec<u8>>,
    timeout_seconds: Option<u64>,
    interactive: Option<bool>,
}

#[async_trait]
impl ActionExecutor for TerminalActionHandler {
    fn action_type(&self) -> &str {
        "TERMINAL_OPERATION"
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let params = self.parse_params(action_data)?;
        
        // Determine if interactive
        let is_interactive = params.interactive
            .unwrap_or_else(|| self.is_interactive(&params.command));
        
        let timeout = params.timeout_seconds.map(|s| std::time::Duration::from_secs(s));
        
        if is_interactive {
            // Use PTY for interactive programs
            let (output, error, exit_code) = self.executor
                .execute_interactive(
                    &params.command,
                    &params.args,
                    params.input.as_deref(),
                    timeout,
                )
                .await
                .map_err(|e| ActionError::ExecutionFailed(e.to_string()))?;
            
            let result_data = serde_json::json!({
                "stdout": String::from_utf8_lossy(&output),
                "stderr": String::from_utf8_lossy(&error),
                "exit_code": exit_code,
            });
            
            if exit_code != 0 {
                return Err(ActionError::ExecutionFailed(format!(
                    "Process exited with code {}: {}",
                    exit_code,
                    String::from_utf8_lossy(&error)
                )));
            }
            
            serde_json::to_vec(&result_data)
                .map_err(|e| ActionError::ExecutionFailed(format!("Failed to serialize result: {}", e)))
        } else {
            // Use standard execution for non-interactive
            let output = tokio::process::Command::new(&params.command)
                .args(&params.args)
                .stdin(if params.input.is_some() {
                    std::process::Stdio::piped()
                } else {
                    std::process::Stdio::null()
                })
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .output()
                .await
                .map_err(|e| ActionError::ExecutionFailed(e.to_string()))?;
            
            let result_data = serde_json::json!({
                "stdout": String::from_utf8_lossy(&output.stdout),
                "stderr": String::from_utf8_lossy(&output.stderr),
                "exit_code": output.status.code().unwrap_or(-1),
            });
            
            if !output.status.success() {
                return Err(ActionError::ExecutionFailed(format!(
                    "Process failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
            
            serde_json::to_vec(&result_data)
                .map_err(|e| ActionError::ExecutionFailed(format!("Failed to serialize result: {}", e)))
        }
    }
}
