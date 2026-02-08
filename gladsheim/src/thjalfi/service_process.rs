//! ServiceProcess wrapper for managing service processes

use crate::utils::{GladsheimError, Result};
use tokio::process::Child;
use std::time::{Duration, Instant};
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub enum ProcessStatus {
    Starting,
    Running,
    Stopping,
    Finished { exit_code: Option<i32> },
    Killed,
}

impl ProcessStatus {
    pub fn is_running(&self) -> bool {
        matches!(self, ProcessStatus::Running)
    }
    
    pub fn is_finished(&self) -> bool {
        matches!(self, ProcessStatus::Finished { .. } | ProcessStatus::Killed)
    }
}

pub struct ServiceProcess {
    child: Child,
    process_id: u32,
    start_time: Instant,
    status: ProcessStatus,
}

impl ServiceProcess {
    pub fn new(child: Child) -> Result<Self> {
        let process_id = child.id()
            .ok_or_else(|| GladsheimError::ProcessError("Failed to get process ID".to_string()))?;
        
        info!("Created ServiceProcess with PID: {}", process_id);
        
        Ok(Self {
            child,
            process_id,
            start_time: Instant::now(),
            status: ProcessStatus::Starting,
        })
    }
    
    pub fn process_id(&self) -> Option<u32> {
        Some(self.process_id)
    }
    
    pub fn start_time(&self) -> Instant {
        self.start_time
    }
    
    pub async fn status(&mut self) -> ProcessStatus {
        // Check if process is still running
        if let Ok(Some(exit_status)) = self.child.try_wait() {
            self.status = ProcessStatus::Finished {
                exit_code: exit_status.code(),
            };
        } else {
            self.status = ProcessStatus::Running;
        }
        
        self.status.clone()
    }
    
    pub async fn stop_graceful(&mut self, timeout: Duration) -> Result<()> {
        info!("Stopping process {} gracefully (timeout: {:?})", self.process_id, timeout);
        
        self.status = ProcessStatus::Stopping;
        
        // Try to kill gracefully (SIGTERM on Unix, Ctrl+Break on Windows)
        if let Err(e) = self.child.kill().await {
            warn!("Failed to send kill signal: {}", e);
            return Err(GladsheimError::ProcessError(format!("Failed to stop process: {}", e)));
        }
        
        // Wait for process to exit with timeout
        let start = Instant::now();
        while start.elapsed() < timeout {
            if let Ok(Some(_)) = self.child.try_wait() {
                self.status = ProcessStatus::Finished { exit_code: self.child.id().map(|_| 0) };
                info!("Process {} stopped gracefully", self.process_id);
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        // Timeout reached, force kill
        warn!("Graceful shutdown timeout, force killing process {}", self.process_id);
        self.force_kill().await
    }
    
    pub async fn force_kill(&mut self) -> Result<()> {
        info!("Force killing process {}", self.process_id);
        
        // Process should already be killed, but ensure it's dead
        let _ = self.child.kill().await;
        
        // Wait a bit for process to die
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        self.status = ProcessStatus::Killed;
        Ok(())
    }
    
    pub fn into_child(self) -> Child {
        self.child
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::process::Command;
    
    #[tokio::test]
    async fn test_service_process_creation() {
        let mut cmd = Command::new("echo");
        cmd.arg("test");
        cmd.stdout(std::process::Stdio::piped());
        
        let child = cmd.spawn().unwrap();
        let process = ServiceProcess::new(child);
        assert!(process.is_ok());
        
        let process = process.unwrap();
        assert!(process.process_id().is_some());
    }
    
    #[tokio::test]
    async fn test_process_status() {
        let mut cmd = Command::new("echo");
        cmd.arg("test");
        cmd.stdout(std::process::Stdio::piped());
        
        let child = cmd.spawn().unwrap();
        let mut process = ServiceProcess::new(child).unwrap();
        
        // Wait a bit for process to finish
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let status = process.status().await;
        assert!(status.is_finished());
    }
}
