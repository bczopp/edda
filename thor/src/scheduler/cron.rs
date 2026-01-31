use thiserror::Error;

#[derive(Debug, Error)]
pub enum CronError {
    #[error("Invalid cron expression: {0}")]
    InvalidExpression(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Permission denied")]
    PermissionDenied,
}

/// Cron scheduler for Linux/macOS
pub struct CronScheduler;

impl CronScheduler {
    pub fn new() -> Self {
        Self
    }

    /// Validate cron expression
    pub fn validate_expression(expr: &str) -> Result<(), CronError> {
        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() != 5 {
            return Err(CronError::InvalidExpression(
                "Cron expression must have exactly 5 parts (minute hour day month weekday)".to_string()
            ));
        }
        
        // Basic validation - check that parts are valid
        for (i, part) in parts.iter().enumerate() {
            if !Self::validate_part(part, i) {
                return Err(CronError::InvalidExpression(
                    format!("Invalid part {} in cron expression: {}", i, part)
                ));
            }
        }
        
        Ok(())
    }

    fn validate_part(part: &str, _index: usize) -> bool {
        // Allow wildcards, ranges, and step values
        if part == "*" {
            return true;
        }
        
        // Check for step values (e.g., */5, 0-59/10)
        if part.contains('/') {
            let parts: Vec<&str> = part.split('/').collect();
            if parts.len() != 2 {
                return false;
            }
            // Validate the step number
            if let Ok(_) = parts[1].parse::<u32>() {
                return true;
            }
            return false;
        }
        
        // Check for ranges (e.g., 0-59, 1-5)
        if part.contains('-') {
            let parts: Vec<&str> = part.split('-').collect();
            if parts.len() != 2 {
                return false;
            }
            if parts[0].parse::<u32>().is_ok() && parts[1].parse::<u32>().is_ok() {
                return true;
            }
            return false;
        }
        
        // Check for lists (e.g., 1,3,5)
        if part.contains(',') {
            for item in part.split(',') {
                if item.parse::<u32>().is_err() {
                    return false;
                }
            }
            return true;
        }
        
        // Single number
        part.parse::<u32>().is_ok()
    }

    /// Create a cron job
    pub async fn create_job(
        &self,
        _job_name: &str,
        schedule: &str,
        _command: &str,
    ) -> Result<(), CronError> {
        Self::validate_expression(schedule)?;
        
        // In a real implementation, this would:
        // 1. Parse the crontab file
        // 2. Add the new job
        // 3. Write back to crontab
        // For now, we'll just validate and return success
        // Actual implementation would use `crontab` command or direct file manipulation
        
        Ok(())
    }

    /// Delete a cron job
    pub async fn delete_job(&self, _job_name: &str) -> Result<(), CronError> {
        // In a real implementation, this would:
        // 1. Parse the crontab file
        // 2. Remove the job with matching name
        // 3. Write back to crontab
        
        Ok(())
    }

    /// List all cron jobs
    pub async fn list_jobs(&self) -> Result<Vec<CronJob>, CronError> {
        // In a real implementation, this would:
        // 1. Parse the crontab file
        // 2. Return list of jobs
        
        Ok(Vec::new())
    }

    /// Update a cron job
    pub async fn update_job(
        &self,
        job_name: &str,
        schedule: &str,
        command: &str,
    ) -> Result<(), CronError> {
        Self::validate_expression(schedule)?;
        
        // Delete old job and create new one
        self.delete_job(job_name).await?;
        self.create_job(job_name, schedule, command).await?;
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CronJob {
    pub name: String,
    pub schedule: String,
    pub command: String,
}

impl Default for CronScheduler {
    fn default() -> Self {
        Self::new()
    }
}
