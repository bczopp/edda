use crate::actions::{ActionExecutor, ActionContext, ActionError};
use crate::scheduler::cron::CronScheduler;
use serde_json::Value;
use async_trait::async_trait;

#[cfg(target_os = "windows")]
mod windows_impl {
    use super::*;
    
    pub async fn create_task(_name: &str, _schedule: &str, _command: &str) -> Result<(), ActionError> {
        // Windows Task Scheduler implementation
        // Would use windows-service crate for actual implementation
        Err(ActionError::ExecutionFailed("Windows Task Scheduler not yet fully implemented".to_string()))
    }
    
    pub async fn delete_task(_name: &str) -> Result<(), ActionError> {
        Err(ActionError::ExecutionFailed("Windows Task Scheduler not yet fully implemented".to_string()))
    }
    
    pub async fn list_tasks() -> Result<Vec<String>, ActionError> {
        Err(ActionError::ExecutionFailed("Windows Task Scheduler not yet fully implemented".to_string()))
    }
}

#[cfg(target_os = "macos")]
mod macos_impl {
    use super::*;
    
    pub async fn create_launchd_job(_name: &str, _schedule: &str, _command: &str) -> Result<(), ActionError> {
        // macOS launchd implementation
        // Would use launchctl and plist generation for actual implementation
        Err(ActionError::ExecutionFailed("macOS launchd not yet fully implemented".to_string()))
    }
    
    pub async fn delete_launchd_job(_name: &str) -> Result<(), ActionError> {
        Err(ActionError::ExecutionFailed("macOS launchd not yet fully implemented".to_string()))
    }
    
    pub async fn list_launchd_jobs() -> Result<Vec<String>, ActionError> {
        Err(ActionError::ExecutionFailed("macOS launchd not yet fully implemented".to_string()))
    }
}

/// Scheduler action handler
pub struct SchedulerActionHandler {
    cron_scheduler: CronScheduler,
}

impl SchedulerActionHandler {
    pub fn new() -> Self {
        Self {
            cron_scheduler: CronScheduler::new(),
        }
    }

    fn parse_params(&self, action_data: &[u8]) -> Result<SchedulerParams, ActionError> {
        let value: Value = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Failed to parse action data: {}", e)))?;
        
        let operation = value["operation"]
            .as_str()
            .ok_or_else(|| ActionError::InvalidAction("Missing 'operation' field".to_string()))?
            .to_string();
        
        Ok(SchedulerParams {
            operation,
            job_name: value["job_name"].as_str().map(|s| s.to_string()),
            schedule: value["schedule"].as_str().map(|s| s.to_string()),
            command: value["command"].as_str().map(|s| s.to_string()),
            operating_system: value["operating_system"].as_str()
                .or_else(|| value["platform"].as_str()) // Backward compatibility
                .map(|s| s.to_string()),
        })
    }

    async fn execute_operation(&self, params: &SchedulerParams) -> Result<Value, ActionError> {
        let operating_system = params.operating_system.as_deref().unwrap_or("auto");
        
        match params.operation.as_str() {
            "create" => {
                let job_name = params.job_name.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'job_name' for create operation".to_string()))?;
                let schedule = params.schedule.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'schedule' for create operation".to_string()))?;
                let command = params.command.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'command' for create operation".to_string()))?;
                
                match operating_system {
                    "linux" | "macos" | "auto" => {
                        #[cfg(any(target_os = "linux", target_os = "macos"))]
                        {
                            self.cron_scheduler.create_job(job_name, schedule, command).await
                                .map_err(|e| ActionError::ExecutionFailed(format!("Failed to create cron job: {}", e)))?;
                        }
                        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
                        {
                            return Err(ActionError::ExecutionFailed("Cron scheduler not available on this operating system".to_string()));
                        }
                    }
                    "windows" => {
                        #[cfg(target_os = "windows")]
                        {
                            windows_impl::create_task(job_name, schedule, command).await?;
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            return Err(ActionError::ExecutionFailed("Windows Task Scheduler not available on this operating system".to_string()));
                        }
                    }
                    _ => {
                        return Err(ActionError::InvalidAction(format!("Unsupported operating system: {}", operating_system)));
                    }
                }
                
                Ok(serde_json::json!({
                    "success": true,
                    "operation": "create",
                    "job_name": job_name
                }))
            }
            "delete" => {
                let job_name = params.job_name.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'job_name' for delete operation".to_string()))?;
                
                match operating_system {
                    "linux" | "macos" | "auto" => {
                        #[cfg(any(target_os = "linux", target_os = "macos"))]
                        {
                            self.cron_scheduler.delete_job(job_name).await
                                .map_err(|e| ActionError::ExecutionFailed(format!("Failed to delete cron job: {}", e)))?;
                        }
                        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
                        {
                            return Err(ActionError::ExecutionFailed("Cron scheduler not available on this operating system".to_string()));
                        }
                    }
                    "windows" => {
                        #[cfg(target_os = "windows")]
                        {
                            windows_impl::delete_task(job_name).await?;
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            return Err(ActionError::ExecutionFailed("Windows Task Scheduler not available on this operating system".to_string()));
                        }
                    }
                    _ => {
                        return Err(ActionError::InvalidAction(format!("Unsupported operating system: {}", operating_system)));
                    }
                }
                
                Ok(serde_json::json!({
                    "success": true,
                    "operation": "delete",
                    "job_name": job_name
                }))
            }
            "list" => {
                let jobs = match operating_system {
                    "linux" | "macos" | "auto" => {
                        #[cfg(any(target_os = "linux", target_os = "macos"))]
                        {
                            self.cron_scheduler.list_jobs().await
                                .map_err(|e| ActionError::ExecutionFailed(format!("Failed to list cron jobs: {}", e)))?
                                .into_iter()
                                .map(|job| serde_json::json!({
                                    "name": job.name,
                                    "schedule": job.schedule,
                                    "command": job.command
                                }))
                                .collect::<Vec<_>>()
                        }
                        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
                        {
                            Vec::new()
                        }
                    }
                    "windows" => {
                        #[cfg(target_os = "windows")]
                        {
                            windows_impl::list_tasks().await?
                                .into_iter()
                                .map(|name| serde_json::json!({"name": name}))
                                .collect()
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            Vec::new()
                        }
                    }
                    _ => {
                        return Err(ActionError::InvalidAction(format!("Unsupported operating system: {}", operating_system)));
                    }
                };
                
                Ok(serde_json::json!({
                    "success": true,
                    "operation": "list",
                    "jobs": jobs
                }))
            }
            "update" => {
                let job_name = params.job_name.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'job_name' for update operation".to_string()))?;
                let schedule = params.schedule.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'schedule' for update operation".to_string()))?;
                let command = params.command.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'command' for update operation".to_string()))?;
                
                match operating_system {
                    "linux" | "macos" | "auto" => {
                        #[cfg(any(target_os = "linux", target_os = "macos"))]
                        {
                            self.cron_scheduler.update_job(job_name, schedule, command).await
                                .map_err(|e| ActionError::ExecutionFailed(format!("Failed to update cron job: {}", e)))?;
                        }
                        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
                        {
                            return Err(ActionError::ExecutionFailed("Cron scheduler not available on this operating system".to_string()));
                        }
                    }
                    "windows" => {
                        #[cfg(target_os = "windows")]
                        {
                            // Delete and recreate
                            windows_impl::delete_task(job_name).await?;
                            windows_impl::create_task(job_name, schedule, command).await?;
                        }
                        #[cfg(not(target_os = "windows"))]
                        {
                            return Err(ActionError::ExecutionFailed("Windows Task Scheduler not available on this operating system".to_string()));
                        }
                    }
                    _ => {
                        return Err(ActionError::InvalidAction(format!("Unsupported operating system: {}", operating_system)));
                    }
                }
                
                Ok(serde_json::json!({
                    "success": true,
                    "operation": "update",
                    "job_name": job_name
                }))
            }
            _ => {
                Err(ActionError::InvalidAction(format!("Unknown scheduler operation: {}", params.operation)))
            }
        }
    }
}

#[derive(Debug)]
struct SchedulerParams {
    operation: String,
    job_name: Option<String>,
    schedule: Option<String>,
    command: Option<String>,
    operating_system: Option<String>,
}

#[async_trait]
impl ActionExecutor for SchedulerActionHandler {
    fn action_type(&self) -> &str {
        "SCHEDULER_OPERATION"
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let params = self.parse_params(action_data)?;
        
        let result = self.execute_operation(&params).await?;
        
        serde_json::to_vec(&result)
            .map_err(|e| ActionError::ExecutionFailed(format!("Failed to serialize result: {}", e)))
    }
}

impl Default for SchedulerActionHandler {
    fn default() -> Self {
        Self::new()
    }
}
