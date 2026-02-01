use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CronError {
    #[error("Invalid cron expression: {0}")]
    InvalidExpression(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Crontab storage abstraction (for tests and real crontab).
#[async_trait]
pub trait CrontabStore: Send + Sync {
    async fn read(&self) -> Result<String, CronError>;
    async fn write(&self, content: &str) -> Result<(), CronError>;
}

/// In-memory crontab store for tests (no local crontab required).
pub struct InMemoryCrontabStore {
    content: RwLock<String>,
}

impl InMemoryCrontabStore {
    pub fn new() -> Self {
        Self { content: RwLock::new(String::new()) }
    }
}

#[async_trait]
impl CrontabStore for InMemoryCrontabStore {
    async fn read(&self) -> Result<String, CronError> {
        Ok(self.content.read().await.clone())
    }
    async fn write(&self, content: &str) -> Result<(), CronError> {
        *self.content.write().await = content.to_string();
        Ok(())
    }
}

impl Default for InMemoryCrontabStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse crontab content (Thor format: "# Thor job: name=<name>" followed by "<schedule> <command>").
pub fn parse_crontab(content: &str) -> Result<Vec<CronJob>, CronError> {
    let mut jobs = Vec::new();
    let mut pending_name: Option<String> = None;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            pending_name = None;
            continue;
        }
        if let Some(name) = trimmed.strip_prefix("# Thor job: name=") {
            pending_name = Some(name.trim().to_string());
            continue;
        }
        if trimmed.starts_with('#') {
            pending_name = None;
            continue;
        }
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 6 {
            continue;
        }
        let schedule = parts[0..5].join(" ");
        let command = parts[5..].join(" ").trim().to_string();
        if cron_parser::parse(&schedule, &chrono::Utc::now()).is_err() {
            return Err(CronError::InvalidExpression(schedule.clone()));
        }
        let name = pending_name.take().unwrap_or_else(|| format!("job_{}", jobs.len()));
        jobs.push(CronJob { name, schedule, command });
    }
    Ok(jobs)
}

/// Format jobs as crontab content (Thor format).
pub fn format_crontab(jobs: &[CronJob]) -> String {
    let mut out = String::new();
    for job in jobs {
        out.push_str("# Thor job: name=");
        out.push_str(&job.name);
        out.push('\n');
        out.push_str(&job.schedule);
        out.push(' ');
        out.push_str(&job.command);
        out.push('\n');
    }
    out
}

#[derive(Debug, Clone)]
pub struct CronJob {
    pub name: String,
    pub schedule: String,
    pub command: String,
}

/// Cron scheduler for Linux/macOS.
pub struct CronScheduler {
    store: Option<Arc<dyn CrontabStore>>,
}

impl CronScheduler {
    pub fn new() -> Self {
        Self { store: None }
    }

    /// New scheduler with a store (e.g. InMemoryCrontabStore for tests).
    pub fn new_with_store(store: Arc<dyn CrontabStore>) -> Self {
        Self { store: Some(store) }
    }

    /// Validate cron expression using cron-parser.
    pub fn validate_expression(expr: &str) -> Result<(), CronError> {
        cron_parser::parse(expr, &chrono::Utc::now())
            .map_err(|e| CronError::InvalidExpression(format!("{}", e)))?;
        Ok(())
    }

    /// Create a cron job.
    pub async fn create_job(
        &self,
        job_name: &str,
        schedule: &str,
        command: &str,
    ) -> Result<(), CronError> {
        Self::validate_expression(schedule)?;
        if let Some(ref store) = self.store {
            let mut jobs = parse_crontab(&store.read().await?)?;
            if jobs.iter().any(|j| j.name == job_name) {
                return Err(CronError::ParseError(format!("Job already exists: {}", job_name)));
            }
            jobs.push(CronJob {
                name: job_name.to_string(),
                schedule: schedule.to_string(),
                command: command.to_string(),
            });
            store.write(&format_crontab(&jobs)).await?;
        }
        Ok(())
    }

    /// Delete a cron job.
    pub async fn delete_job(&self, job_name: &str) -> Result<(), CronError> {
        if let Some(ref store) = self.store {
            let mut jobs = parse_crontab(&store.read().await?)?;
            let len_before = jobs.len();
            jobs.retain(|j| j.name != job_name);
            if jobs.len() == len_before {
                return Err(CronError::ParseError(format!("Job not found: {}", job_name)));
            }
            store.write(&format_crontab(&jobs)).await?;
        }
        Ok(())
    }

    /// List all cron jobs.
    pub async fn list_jobs(&self) -> Result<Vec<CronJob>, CronError> {
        if let Some(ref store) = self.store {
            return parse_crontab(&store.read().await?);
        }
        Ok(Vec::new())
    }

    /// Update a cron job.
    pub async fn update_job(
        &self,
        job_name: &str,
        schedule: &str,
        command: &str,
    ) -> Result<(), CronError> {
        Self::validate_expression(schedule)?;
        if let Some(ref store) = self.store {
            let mut jobs = parse_crontab(&store.read().await?)?;
            let found = jobs.iter_mut().find(|j| j.name == job_name);
            match found {
                Some(j) => {
                    j.schedule = schedule.to_string();
                    j.command = command.to_string();
                }
                None => {
                    jobs.push(CronJob {
                        name: job_name.to_string(),
                        schedule: schedule.to_string(),
                        command: command.to_string(),
                    });
                }
            }
            store.write(&format_crontab(&jobs)).await?;
        }
        Ok(())
    }
}

impl Default for CronScheduler {
    fn default() -> Self {
        Self::new()
    }
}
