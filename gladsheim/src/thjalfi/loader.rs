//! Thjalfi - Service Loader

use crate::utils::{GladsheimError, Result};
use crate::thjalfi::process::ProcessManager;
use crate::thjalfi::service_process::ServiceProcess;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, error};

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub working_directory: Option<PathBuf>,
    pub environment_vars: HashMap<String, String>,
}

pub struct Thjalfi {
    #[allow(dead_code)]
    process_manager: ProcessManager,
    running_services: Arc<RwLock<HashMap<String, ServiceProcess>>>,
}

impl Thjalfi {
    pub fn new() -> Result<Self> {
        info!("Initializing Thjalfi (Service Loader)");
        
        Ok(Self {
            process_manager: ProcessManager::new(),
            running_services: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn start_process(
        &self,
        command: &str,
        args: &[&str],
        working_dir: Option<&PathBuf>,
        env_vars: Option<&HashMap<String, String>>,
    ) -> Result<ServiceProcess> {
        info!("Starting process: {} {:?}", command, args);
        
        let mut cmd = tokio::process::Command::new(command);
        cmd.args(args);
        cmd.stdin(std::process::Stdio::null());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        
        if let Some(dir) = working_dir {
            cmd.current_dir(dir);
        }
        
        if let Some(env) = env_vars {
            for (key, value) in env {
                cmd.env(key, value);
            }
        }
        
        let child = cmd.spawn()
            .map_err(|e| GladsheimError::ProcessError(format!("Failed to spawn process: {}", e)))?;
        
        ServiceProcess::new(child)
    }
    
    pub async fn start_service(
        &self,
        config: ServiceConfig,
        startup_timeout: Duration,
    ) -> Result<()> {
        info!("Starting service: {}", config.name);
        
        // Check if service is already running
        {
            let services = self.running_services.read().await;
            if services.contains_key(&config.name) {
                return Err(GladsheimError::ServiceError(
                    format!("Service '{}' is already running", config.name)
                ));
            }
        }
        
        // Start process
        let args: Vec<&str> = config.args.iter().map(|s| s.as_str()).collect();
        let mut process = self.start_process(
            &config.command,
            &args,
            config.working_directory.as_ref(),
            Some(&config.environment_vars),
        ).await?;
        
        // Wait for startup validation (simple check: process is still running)
        let start = std::time::Instant::now();
        while start.elapsed() < startup_timeout {
            let status = process.status().await;
            if status.is_running() {
                // Service started successfully
                {
                    let mut services = self.running_services.write().await;
                    services.insert(config.name.clone(), process);
                }
                info!("Service '{}' started successfully", config.name);
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        // Timeout reached
        error!("Service '{}' startup timeout", config.name);
        Err(GladsheimError::ServiceError(
            format!("Service '{}' startup timeout", config.name)
        ))
    }
    
    pub async fn stop_process(
        &self,
        process: &mut ServiceProcess,
        force: bool,
        timeout: Option<Duration>,
    ) -> Result<()> {
        let timeout = timeout.unwrap_or(Duration::from_secs(5));
        
        if force {
            process.force_kill().await
        } else {
            process.stop_graceful(timeout).await
        }
    }
    
    pub async fn stop_service(
        &self,
        service_name: &str,
        force: bool,
        timeout: Option<Duration>,
    ) -> Result<()> {
        info!("Stopping service: {} (force: {})", service_name, force);
        
        let mut process = {
            let mut services = self.running_services.write().await;
            services.remove(service_name)
                .ok_or_else(|| GladsheimError::ServiceError(
                    format!("Service '{}' not found", service_name)
                ))?
        };
        
        self.stop_process(&mut process, force, timeout).await?;
        
        info!("Service '{}' stopped", service_name);
        Ok(())
    }

    /// Restart a running service: stop then start with the given config.
    pub async fn restart_service(
        &self,
        service_name: &str,
        config: ServiceConfig,
        startup_timeout: Duration,
        shutdown_timeout: Option<Duration>,
    ) -> Result<()> {
        info!("Restarting service: {}", service_name);
        self.stop_service(service_name, false, shutdown_timeout).await?;
        self.start_service(config, startup_timeout).await
    }
    
    pub async fn has_service(&self, service_name: &str) -> bool {
        let services = self.running_services.read().await;
        services.contains_key(service_name)
    }
    
    pub async fn get_service_pid(&self, service_name: &str) -> Option<u32> {
        let services = self.running_services.read().await;
        services.get(service_name).and_then(|p| p.process_id())
    }
    
    pub async fn list_running_services(&self) -> Vec<String> {
        let services = self.running_services.read().await;
        services.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_thjalfi_creation() {
        let thjalfi = Thjalfi::new();
        assert!(thjalfi.is_ok());
    }
    
    #[tokio::test]
    async fn test_start_process() {
        let thjalfi = Thjalfi::new().unwrap();
        
        #[cfg(unix)]
        let result = thjalfi.start_process("echo", &["test"], None, None).await;
        #[cfg(windows)]
        let result = thjalfi.start_process("cmd", &["/C", "echo", "test"], None, None).await;
        
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_list_running_services() {
        let thjalfi = Thjalfi::new().unwrap();
        let services = thjalfi.list_running_services().await;
        assert_eq!(services.len(), 0);
    }
}
