use crate::actions::{ActionExecutor, ActionContext, ActionError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppControlParams {
    pub operation: String, // "start", "stop", "status"
    pub app_path: String,
    pub args: Option<Vec<String>>,
}

pub struct AppControlExecutor;

#[async_trait]
impl ActionExecutor for AppControlExecutor {
    fn action_type(&self) -> &str {
        "APP_CONTROL"
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let params: AppControlParams = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Invalid app control: {}", e)))?;

        match params.operation.as_str() {
            "start" => {
                let mut cmd = Command::new(&params.app_path);
                if let Some(ref args) = params.args {
                    cmd.args(args);
                }
                let mut child = cmd.spawn()
                    .map_err(|e| ActionError::ExecutionFailed(format!("Failed to start app: {}", e)))?;
                
                let pid = child.id();
                tokio::spawn(async move { let _ = child.wait().await; });
                
                Ok(serde_json::to_vec(&serde_json::json!({ "pid": pid, "status": "started" })).unwrap())
            }
            "stop" => {
                // Parse PID from app_path or use process name
                // For now, try to kill by process name
                let process_name = std::path::Path::new(&params.app_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&params.app_path);
                
                #[cfg(unix)]
                {
                    let _ = Command::new("pkill")
                        .arg("-f")
                        .arg(process_name)
                        .output()
                        .await;
                }
                
                #[cfg(windows)]
                {
                    let _ = Command::new("taskkill")
                        .arg("/F")
                        .arg("/IM")
                        .arg(process_name)
                        .output()
                        .await;
                }
                
                Ok(serde_json::to_vec(&serde_json::json!({ "status": "stopped" })).unwrap())
            }
            "status" => {
                let process_name = std::path::Path::new(&params.app_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&params.app_path);
                
                #[cfg(unix)]
                {
                    let output = Command::new("pgrep")
                        .arg("-f")
                        .arg(process_name)
                        .output()
                        .await
                        .ok();
                    
                    let is_running = output
                        .and_then(|o| o.status.success().then_some(true))
                        .unwrap_or(false);
                    
                    return Ok(serde_json::to_vec(&serde_json::json!({ 
                        "status": if is_running { "running" } else { "stopped" }
                    })).unwrap());
                }
                
                #[cfg(windows)]
                {
                    let output = Command::new("tasklist")
                        .arg("/FI")
                        .arg(format!("IMAGENAME eq {}", process_name))
                        .output()
                        .await
                        .ok();
                    
                    let is_running = output
                        .and_then(|o| String::from_utf8(o.stdout).ok())
                        .map(|s| s.contains(process_name))
                        .unwrap_or(false);
                    
                    return Ok(serde_json::to_vec(&serde_json::json!({ 
                        "status": if is_running { "running" } else { "stopped" }
                    })).unwrap());
                }
                
                #[cfg(not(any(unix, windows)))]
                Ok(serde_json::to_vec(&serde_json::json!({ "status": "unknown" })).unwrap())
            }
            _ => Err(ActionError::InvalidAction(format!("Unknown operation: {}", params.operation))),
        }
    }
}
