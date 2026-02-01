use serde_json::Value;
use std::sync::Arc;
use tracing::warn;
use chrono::Utc;
use sqlx::PgPool;
use crate::mimir_client::MimirClient;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub event_type: String,
    pub entity_type: String,
    pub entity_id: String,
    pub user_id: Option<String>,
    pub details: Value,
    pub timestamp: chrono::DateTime<Utc>,
}

#[async_trait]
pub trait AuditLogger: Send + Sync {
    async fn log(&self, event: AuditEvent) -> Result<(), AuditError>;
}

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Mimir error: {0}")]
    MimirError(#[from] crate::mimir_client::MimirClientError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// PostgreSQL-based audit logger
pub struct PostgresAuditLogger {
    pool: Arc<PgPool>,
}

impl PostgresAuditLogger {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditLogger for PostgresAuditLogger {
    async fn log(&self, event: AuditEvent) -> Result<(), AuditError> {
        // Note: Using provider_id as entity_id for compatibility with existing schema
        sqlx::query!(
            "INSERT INTO audit_logs (event_type, provider_id, details, event_timestamp) VALUES ($1, $2, $3, $4)",
            event.event_type,
            event.entity_id,
            event.details,
            event.timestamp
        )
        .execute(&*self.pool)
        .await?;
        
        Ok(())
    }
}

/// Mimir-based audit logger
pub struct MimirAuditLogger {
    mimir_client: Arc<MimirClient>,
    user_id: String,
}

impl MimirAuditLogger {
    pub fn new(mimir_client: Arc<MimirClient>, user_id: String) -> Self {
        Self { mimir_client, user_id }
    }
}

#[async_trait]
impl AuditLogger for MimirAuditLogger {
    async fn log(&self, event: AuditEvent) -> Result<(), AuditError> {
        // Store audit event in Mimir
        let audit_data = serde_json::json!({
            "event_type": event.event_type,
            "entity_type": event.entity_type,
            "entity_id": event.entity_id,
            "user_id": event.user_id,
            "details": event.details,
            "timestamp": event.timestamp.to_rfc3339(),
        });
        
        let data = serde_json::to_vec(&audit_data)?;
        self.mimir_client.store_data(&self.user_id, &data).await?;
        
        Ok(())
    }
}

/// Composite audit logger that logs to multiple backends
pub struct CompositeAuditLogger {
    loggers: Vec<Arc<dyn AuditLogger>>,
}

impl CompositeAuditLogger {
    pub fn new() -> Self {
        Self {
            loggers: Vec::new(),
        }
    }

    pub fn add_logger(&mut self, logger: Arc<dyn AuditLogger>) {
        self.loggers.push(logger);
    }
}

#[async_trait]
impl AuditLogger for CompositeAuditLogger {
    async fn log(&self, event: AuditEvent) -> Result<(), AuditError> {
        // Log to all loggers, but don't fail if one fails
        for logger in &self.loggers {
            if let Err(e) = logger.log(event.clone()).await {
                warn!("Failed to log audit event: {}", e);
            }
        }
        Ok(())
    }
}

impl Default for CompositeAuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to create audit events
impl AuditEvent {
    pub fn new(
        event_type: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: impl Into<String>,
        details: Value,
    ) -> Self {
        Self {
            event_type: event_type.into(),
            entity_type: entity_type.into(),
            entity_id: entity_id.into(),
            user_id: None,
            details,
            timestamp: Utc::now(),
        }
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }
}
