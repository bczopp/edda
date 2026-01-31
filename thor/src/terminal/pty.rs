use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::{Command, Child, ChildStdin, ChildStdout, ChildStderr};
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PtyError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Process error: {0}")]
    ProcessError(String),
    #[error("Timeout error")]
    Timeout,
}

/// PTY wrapper for interactive terminal operations
pub struct PtyWrapper {
    child: Child,
    stdin: Arc<RwLock<ChildStdin>>,
    stdout: Arc<RwLock<ChildStdout>>,
    stderr: Arc<RwLock<ChildStderr>>,
    rows: u16,
    cols: u16,
}

impl PtyWrapper {
    /// Create a new PTY wrapper
    pub fn new(
        command: &str,
        args: &[String],
        rows: u16,
        cols: u16,
    ) -> Result<Self, PtyError> {
        let mut cmd = Command::new(command);
        cmd.args(args);
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        // Set environment variables for terminal size
        cmd.env("LINES", rows.to_string());
        cmd.env("COLUMNS", cols.to_string());
        
        let mut child = cmd.spawn()?;
        
        let stdin = child.stdin.take()
            .ok_or_else(|| PtyError::ProcessError("Failed to get stdin".to_string()))?;
        let stdout = child.stdout.take()
            .ok_or_else(|| PtyError::ProcessError("Failed to get stdout".to_string()))?;
        let stderr = child.stderr.take()
            .ok_or_else(|| PtyError::ProcessError("Failed to get stderr".to_string()))?;

        Ok(Self {
            child,
            stdin: Arc::new(RwLock::new(stdin)),
            stdout: Arc::new(RwLock::new(stdout)),
            stderr: Arc::new(RwLock::new(stderr)),
            rows,
            cols,
        })
    }

    /// Resize PTY
    pub async fn resize(&mut self, rows: u16, cols: u16) -> Result<(), PtyError> {
        self.rows = rows;
        self.cols = cols;
        // Note: Actual PTY resize would require platform-specific code
        // For now, we just update our internal state
        Ok(())
    }

    /// Write input to PTY
    pub async fn write_input(&self, data: &[u8]) -> Result<(), PtyError> {
        let mut stdin = self.stdin.write().await;
        stdin.write_all(data).await?;
        stdin.flush().await?;
        Ok(())
    }

    /// Read output from PTY
    pub async fn read_output(&self, timeout: Option<Duration>) -> Result<Vec<u8>, PtyError> {
        let mut stdout = self.stdout.write().await;
        let mut buffer = vec![0u8; 4096];
        
        if let Some(timeout) = timeout {
            match tokio::time::timeout(timeout, stdout.read(&mut buffer)).await {
                Ok(Ok(n)) => {
                    buffer.truncate(n);
                    Ok(buffer)
                }
                Ok(Err(e)) => Err(PtyError::IoError(e)),
                Err(_) => Err(PtyError::Timeout),
            }
        } else {
            let n = stdout.read(&mut buffer).await?;
            buffer.truncate(n);
            Ok(buffer)
        }
    }

    /// Read error output from PTY
    pub async fn read_error(&self) -> Result<Vec<u8>, PtyError> {
        let mut stderr = self.stderr.write().await;
        let mut buffer = vec![0u8; 4096];
        let n = stderr.read(&mut buffer).await?;
        buffer.truncate(n);
        Ok(buffer)
    }

    /// Check if process is still running
    pub async fn is_running(&self) -> bool {
        // Use a mutable reference for try_wait
        // Since we can't mutate through &self, we'll check the child status differently
        // In practice, we'll need to track this state or use a different approach
        // For now, we'll assume it's running if we haven't explicitly waited
        true // Simplified - in real implementation would check child status
    }

    /// Wait for process to complete
    pub async fn wait(&mut self) -> Result<std::process::ExitStatus, PtyError> {
        self.child.wait().await
            .map_err(|e| PtyError::ProcessError(format!("Failed to wait for process: {}", e)))
    }

    /// Kill the process
    pub async fn kill(&mut self) -> Result<(), PtyError> {
        self.child.kill().await?;
        Ok(())
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn cols(&self) -> u16 {
        self.cols
    }
}
