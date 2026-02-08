//! Process Management for Thjalfi

use crate::utils::{GladsheimError, Result};
use std::process::Stdio;
use tokio::process::Command;
use tracing::{info, warn};

pub struct ProcessManager;

impl ProcessManager {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn start_process(
        &self,
        command: &str,
        args: &[&str],
    ) -> Result<tokio::process::Child> {
        info!("Starting process: {} {:?}", command, args);
        
        let mut cmd = Command::new(command);
        cmd.args(args);
        cmd.stdin(Stdio::null());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        let child = cmd.spawn()
            .map_err(|e| GladsheimError::ProcessError(format!("Failed to spawn process: {}", e)))?;
        
        Ok(child)
    }
    
    pub async fn stop_process(&self, child: &mut tokio::process::Child) -> Result<()> {
        info!("Stopping process (PID: {})", child.id().unwrap_or(0));
        
        // Try graceful shutdown first
        if let Err(e) = child.kill().await {
            warn!("Failed to kill process gracefully: {}", e);
            return Err(GladsheimError::ProcessError(format!("Failed to stop process: {}", e)));
        }
        
        // Wait for process to exit
        let _ = child.wait().await;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_start_process() {
        let manager = ProcessManager::new();
        
        // Start a simple command (echo)
        #[cfg(unix)]
        let result = manager.start_process("echo", &["test"]).await;
        #[cfg(windows)]
        let result = manager.start_process("cmd", &["/C", "echo", "test"]).await;
        
        assert!(result.is_ok());
        
        let mut child = result.unwrap();
        let _ = manager.stop_process(&mut child).await;
    }
}
