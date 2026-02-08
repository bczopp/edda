use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use chrono::{DateTime, Utc};
use tracing::{debug, error};

#[derive(Debug, Error)]
pub enum AuditLogError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Represents different types of audit events
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditEvent {
    DataStored,
    DataRetrieved,
    DataDeleted,
    DataUpdated,
    DataExported,
    UserDataDeleted,
    AccessDenied,
    PermissionChecked,
}

impl AuditEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuditEvent::DataStored => "DATA_STORED",
            AuditEvent::DataRetrieved => "DATA_RETRIEVED",
            AuditEvent::DataDeleted => "DATA_DELETED",
            AuditEvent::DataUpdated => "DATA_UPDATED",
            AuditEvent::DataExported => "DATA_EXPORTED",
            AuditEvent::UserDataDeleted => "USER_DATA_DELETED",
            AuditEvent::AccessDenied => "ACCESS_DENIED",
            AuditEvent::PermissionChecked => "PERMISSION_CHECKED",
        }
    }
}

/// Represents an audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Option<i32>,
    pub event_type: AuditEvent,
    pub user_id: Option<String>,
    pub data_id: Option<String>,
    pub event_timestamp: DateTime<Utc>,
    pub details: serde_json::Value,
}

impl AuditLog {
    pub fn new(
        event_type: AuditEvent,
        user_id: Option<String>,
        data_id: Option<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            id: None,
            event_type,
            user_id,
            data_id,
            event_timestamp: Utc::now(),
            details,
        }
    }
}

/// Audit Log Manager for GDPR-compliant logging
pub struct AuditLogManager {
    pool: PgPool,
}

impl AuditLogManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Log an audit event (immutable - cannot be modified after creation)
    pub async fn log_event(
        &self,
        event_type: AuditEvent,
        user_id: Option<String>,
        data_id: Option<String>,
        details: serde_json::Value,
    ) -> Result<i32, AuditLogError> {
        let event_type_str = event_type.as_str();
        
        let record = sqlx::query!(
            r#"
            INSERT INTO audit_logs (event_type, user_id, data_id, details)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            event_type_str,
            user_id,
            data_id,
            details
        )
        .fetch_one(&self.pool)
        .await?;
        
        debug!(
            "Audit log created: event={}, user_id={:?}, data_id={:?}, id={}",
            event_type_str, user_id, data_id, record.id
        );
        
        Ok(record.id)
    }

    /// Get all audit logs for a specific user (GDPR Right to Access)
    pub async fn get_user_audit_logs(&self, user_id: &str) -> Result<Vec<AuditLog>, AuditLogError> {
        let records = sqlx::query!(
            r#"
            SELECT id, event_type, user_id, data_id, event_timestamp, details
            FROM audit_logs
            WHERE user_id = $1
            ORDER BY event_timestamp DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut logs = Vec::new();
        for record in records {
            let event_type = match record.event_type.as_str() {
                "DATA_STORED" => AuditEvent::DataStored,
                "DATA_RETRIEVED" => AuditEvent::DataRetrieved,
                "DATA_DELETED" => AuditEvent::DataDeleted,
                "DATA_UPDATED" => AuditEvent::DataUpdated,
                "DATA_EXPORTED" => AuditEvent::DataExported,
                "USER_DATA_DELETED" => AuditEvent::UserDataDeleted,
                "ACCESS_DENIED" => AuditEvent::AccessDenied,
                "PERMISSION_CHECKED" => AuditEvent::PermissionChecked,
                _ => continue, // Skip unknown event types
            };
            
            logs.push(AuditLog {
                id: Some(record.id),
                event_type,
                user_id: record.user_id,
                data_id: record.data_id,
                event_timestamp: record.event_timestamp.and_utc(),
                details: record.details.unwrap_or(serde_json::json!({})),
            });
        }
        
        debug!("Retrieved {} audit logs for user {}", logs.len(), user_id);
        Ok(logs)
    }

    /// Get all audit logs for a specific data entry
    pub async fn get_data_audit_logs(&self, data_id: &str) -> Result<Vec<AuditLog>, AuditLogError> {
        let records = sqlx::query!(
            r#"
            SELECT id, event_type, user_id, data_id, event_timestamp, details
            FROM audit_logs
            WHERE data_id = $1
            ORDER BY event_timestamp DESC
            "#,
            data_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut logs = Vec::new();
        for record in records {
            let event_type = match record.event_type.as_str() {
                "DATA_STORED" => AuditEvent::DataStored,
                "DATA_RETRIEVED" => AuditEvent::DataRetrieved,
                "DATA_DELETED" => AuditEvent::DataDeleted,
                "DATA_UPDATED" => AuditEvent::DataUpdated,
                "DATA_EXPORTED" => AuditEvent::DataExported,
                "USER_DATA_DELETED" => AuditEvent::UserDataDeleted,
                "ACCESS_DENIED" => AuditEvent::AccessDenied,
                "PERMISSION_CHECKED" => AuditEvent::PermissionChecked,
                _ => continue,
            };
            
            logs.push(AuditLog {
                id: Some(record.id),
                event_type,
                user_id: record.user_id,
                data_id: record.data_id,
                event_timestamp: record.event_timestamp.and_utc(),
                details: record.details.unwrap_or(serde_json::json!({})),
            });
        }
        
        debug!("Retrieved {} audit logs for data {}", logs.len(), data_id);
        Ok(logs)
    }

    /// Get all audit logs in a time range (for compliance reporting)
    pub async fn get_logs_in_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<AuditLog>, AuditLogError> {
        let records = sqlx::query!(
            r#"
            SELECT id, event_type, user_id, data_id, event_timestamp, details
            FROM audit_logs
            WHERE event_timestamp >= $1 AND event_timestamp <= $2
            ORDER BY event_timestamp DESC
            "#,
            start.naive_utc(),
            end.naive_utc()
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut logs = Vec::new();
        for record in records {
            let event_type = match record.event_type.as_str() {
                "DATA_STORED" => AuditEvent::DataStored,
                "DATA_RETRIEVED" => AuditEvent::DataRetrieved,
                "DATA_DELETED" => AuditEvent::DataDeleted,
                "DATA_UPDATED" => AuditEvent::DataUpdated,
                "DATA_EXPORTED" => AuditEvent::DataExported,
                "USER_DATA_DELETED" => AuditEvent::UserDataDeleted,
                "ACCESS_DENIED" => AuditEvent::AccessDenied,
                "PERMISSION_CHECKED" => AuditEvent::PermissionChecked,
                _ => continue,
            };
            
            logs.push(AuditLog {
                id: Some(record.id),
                event_type,
                user_id: record.user_id,
                data_id: record.data_id,
                event_timestamp: record.event_timestamp.and_utc(),
                details: record.details.unwrap_or(serde_json::json!({})),
            });
        }
        
        debug!("Retrieved {} audit logs for time range", logs.len());
        Ok(logs)
    }

    /// Count audit events by type (for monitoring)
    pub async fn count_events_by_type(&self, event_type: AuditEvent) -> Result<i64, AuditLogError> {
        let event_type_str = event_type.as_str();
        
        let record = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM audit_logs
            WHERE event_type = $1
            "#,
            event_type_str
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(record.count.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_as_str() {
        assert_eq!(AuditEvent::DataStored.as_str(), "DATA_STORED");
        assert_eq!(AuditEvent::DataRetrieved.as_str(), "DATA_RETRIEVED");
        assert_eq!(AuditEvent::DataDeleted.as_str(), "DATA_DELETED");
        assert_eq!(AuditEvent::AccessDenied.as_str(), "ACCESS_DENIED");
    }

    #[test]
    fn test_audit_log_creation() {
        let log = AuditLog::new(
            AuditEvent::DataStored,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({"size": 1024}),
        );
        
        assert_eq!(log.user_id, Some("user1".to_string()));
        assert_eq!(log.data_id, Some("data1".to_string()));
        assert!(matches!(log.event_type, AuditEvent::DataStored));
    }
}
