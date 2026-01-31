use crate::terminal::PtyWrapper;
use std::sync::Arc;
use std::time::Duration;

/// Interactive program executor using PTY
pub struct InteractiveExecutor {
    pty: Arc<tokio::sync::RwLock<Option<PtyWrapper>>>,
}

impl InteractiveExecutor {
    pub fn new() -> Self {
        Self {
            pty: Arc::new(tokio::sync::RwLock::new(None)),
        }
    }

    /// Execute an interactive program
    pub async fn execute_interactive(
        &self,
        command: &str,
        args: &[String],
        input: Option<&[u8]>,
        timeout: Option<Duration>,
    ) -> Result<(Vec<u8>, Vec<u8>, i32), crate::terminal::PtyError> {
        // Create PTY
        let pty = PtyWrapper::new(command, args, 24, 80)?;
        
        // Store PTY
        {
            let mut pty_guard = self.pty.write().await;
            *pty_guard = Some(pty);
        }

        // Send input if provided
        if let Some(input_data) = input {
            let pty_guard = self.pty.read().await;
            if let Some(ref pty) = *pty_guard {
                pty.write_input(input_data).await?;
            }
        }

        // Read output with timeout
        let mut output = Vec::new();
        let mut error = Vec::new();
        let start_time = std::time::Instant::now();
        let read_timeout = timeout.unwrap_or(Duration::from_secs(30));
        
        // Read output in a loop until process completes or timeout
        loop {
            // Check overall timeout
            if start_time.elapsed() > read_timeout {
                // Kill the process if timeout exceeded
                let mut pty_guard = self.pty.write().await;
                if let Some(ref mut pty) = *pty_guard {
                    let _ = pty.kill().await;
                }
                break;
            }
            
            // Try to read output
            let should_continue = {
                let pty_guard = self.pty.read().await;
                if let Some(ref pty) = *pty_guard {
                    match pty.read_output(Some(Duration::from_millis(100))).await {
                        Ok(data) if !data.is_empty() => {
                            output.extend_from_slice(&data);
                            true
                        }
                        Ok(_) => {
                            // No data, check error
                            if let Ok(err_data) = pty.read_error().await {
                                if !err_data.is_empty() {
                                    error.extend_from_slice(&err_data);
                                }
                            }
                            true
                        }
                        Err(crate::terminal::PtyError::Timeout) => {
                            // Timeout is expected for non-blocking read
                            true
                        }
                        Err(e) => {
                            // Other error - might indicate process finished or error
                            // For EOF, we'll break the loop
                            if matches!(e, crate::terminal::PtyError::IoError(ref io_err) 
                                if io_err.kind() == std::io::ErrorKind::UnexpectedEof) {
                                false
                            } else {
                                return Err(e);
                            }
                        }
                    }
                } else {
                    false
                }
            };
            
            if !should_continue {
                break;
            }
            
            // Small delay to avoid busy waiting
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Wait for process to complete
        let exit_code = {
            let mut pty_guard = self.pty.write().await;
            if let Some(ref mut pty) = *pty_guard {
                pty.wait().await?.code().unwrap_or(-1)
            } else {
                -1
            }
        };

        Ok((output, error, exit_code))
    }
}

impl Default for InteractiveExecutor {
    fn default() -> Self {
        Self::new()
    }
}
