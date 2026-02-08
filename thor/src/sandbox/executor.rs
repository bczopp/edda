use crate::actions::{ActionExecutor, ActionContext, ActionError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxParams {
    pub command: String,
    pub image: Option<String>,
    pub args: Vec<String>,
    pub workdir: Option<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
}

/// Executor that runs commands inside an isolated Docker container
pub struct SandboxActionExecutor;

#[async_trait]
impl ActionExecutor for SandboxActionExecutor {
    fn action_type(&self) -> &str {
        "SANDBOX_COMMAND"
    }

    fn is_sandboxed(&self) -> bool {
        true
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let params: SandboxParams = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Invalid sandbox params: {}", e)))?;

        let image = params.image.unwrap_or_else(|| "alpine:latest".to_string());
        
        info!("Executing sandboxed command in image {}: {}", image, params.command);

        // Prepare docker run command
        let mut docker_cmd = Command::new("docker");
        docker_cmd.arg("run");
        docker_cmd.arg("--rm");
        
        // Add environment variables
        if let Some(env_vars) = params.env {
            for (k, v) in env_vars {
                docker_cmd.arg("-e").arg(format!("{}={}", k, v));
            }
        }

        // Add workdir
        if let Some(dir) = params.workdir {
            docker_cmd.arg("-w").arg(dir);
        }

        docker_cmd.arg(image);
        docker_cmd.arg("sh");
        docker_cmd.arg("-c");
        
        // Combine command and args
        let full_cmd = if params.args.is_empty() {
            params.command
        } else {
            format!("{} {}", params.command, params.args.join(" "))
        };
        docker_cmd.arg(full_cmd);

        let output = docker_cmd.output().await
            .map_err(|e| ActionError::ExecutionFailed(format!("Failed to start docker container: {}", e)))?;

        let result = serde_json::json!({
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "image": image,
        });

        if !output.status.success() {
            warn!("Sandboxed command failed with exit code {}", output.status.code().unwrap_or(-1));
        }

        Ok(serde_json::to_vec(&result).unwrap())
    }
}
