use sqlx::{PgPool, Row};
use sqlx::types::Json;
use uuid::Uuid;
use serde_json::Value;
use chrono::Utc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct AuditLogger {
    pool: PgPool,
}

impl AuditLogger {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn log_event(
        &self,
        event_type: &str,
        device_id: Option<Uuid>,
        user_id: Option<Uuid>,
        resource_type: Option<&str>,
        action: Option<&str>,
        status: &str,
        details: Option<Value>,
    ) -> Result<(), AuditError> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (event_type, device_id, user_id, resource_type, action, status, details, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(event_type)
        .bind(device_id)
        .bind(user_id)
        .bind(resource_type)
        .bind(action)
        .bind(status)
        .bind(details.map(Json))
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[allow(dead_code)]
pub struct ThreatDetector {
    pool: PgPool,
    audit_logger: AuditLogger,
    brute_force_threshold: u32,
}

impl ThreatDetector {
    pub fn new(pool: PgPool, audit_logger: AuditLogger, brute_force_threshold: u32) -> Self {
        Self {
            pool,
            audit_logger,
            brute_force_threshold,
        }
    }

    pub async fn detect_threats(&self, device_id: Uuid, user_id: Uuid) -> Result<bool, AuditError> {
        // Check for brute force attacks
        if self.detect_brute_force(device_id, user_id).await? {
            return Ok(true);
        }

        // Check for rate limiting violations
        if self.detect_rate_limiting_violation(device_id, user_id).await? {
            return Ok(true);
        }

        // Check for anomaly patterns
        if self.detect_anomaly_patterns(device_id, user_id).await? {
            return Ok(true);
        }

        Ok(false)
    }

    async fn detect_brute_force(&self, device_id: Uuid, user_id: Uuid) -> Result<bool, AuditError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM audit_logs
            WHERE device_id = $1 AND user_id = $2 AND event_type = 'authentication'
            AND status = 'failure' AND created_at > CURRENT_TIMESTAMP - INTERVAL '15 minutes'
            "#,
        )
        .bind(device_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        let count: i64 = row.get("count");
        Ok(count > self.brute_force_threshold as i64)
    }

    async fn detect_rate_limiting_violation(&self, device_id: Uuid, user_id: Uuid) -> Result<bool, AuditError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM audit_logs
            WHERE device_id = $1 AND user_id = $2 AND created_at > CURRENT_TIMESTAMP - INTERVAL '1 minute'
            "#,
        )
        .bind(device_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        let count: i64 = row.get("count");
        Ok(count > 100)
    }

    async fn detect_anomaly_patterns(&self, device_id: Uuid, user_id: Uuid) -> Result<bool, AuditError> {
        let rows = sqlx::query(
            r#"
            SELECT event_type, status, COUNT(*) as count
            FROM audit_logs
            WHERE device_id = $1 AND user_id = $2 AND created_at > CURRENT_TIMESTAMP - INTERVAL '1 hour'
            GROUP BY event_type, status
            "#,
        )
        .bind(device_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        let unique_events: std::collections::HashSet<String> = rows
            .iter()
            .map(|r| r.get::<String, _>("event_type"))
            .collect();
        Ok(unique_events.len() > 20)
    }
}

#[allow(dead_code)]
pub struct IncidentResponseManager {
    pool: PgPool,
    audit_logger: AuditLogger,
    threat_detector: ThreatDetector,
}

impl IncidentResponseManager {
    pub fn new(pool: PgPool, audit_logger: AuditLogger, threat_detector: ThreatDetector) -> Self {
        Self {
            pool,
            audit_logger,
            threat_detector,
        }
    }

    pub async fn handle_incident(
        &self,
        device_id: Uuid,
        user_id: Uuid,
        threat_type: &str,
    ) -> Result<(), AuditError> {
        // Log the incident
        self.audit_logger.log_event(
            "security_incident",
            Some(device_id),
            Some(user_id),
            Some("device"),
            Some("threat_detected"),
            "blocked",
            Some(serde_json::json!({"threat_type": threat_type})),
        ).await?;

        // Block the device
        self.block_device(device_id, user_id, threat_type, false).await?;

        Ok(())
    }

    async fn block_device(
        &self,
        device_id: Uuid,
        user_id: Uuid,
        reason: &str,
        is_permanent: bool,
    ) -> Result<(), AuditError> {
        sqlx::query(
            r#"
            INSERT INTO blocked_devices (device_id, user_id, reason, is_permanent, expires_at)
            VALUES ($1, $2, $3, $4, CASE WHEN $4 THEN NULL ELSE CURRENT_TIMESTAMP + INTERVAL '24 hours' END)
            ON CONFLICT (device_id) DO UPDATE
            SET reason = $3, blocked_at = CURRENT_TIMESTAMP, expires_at = CASE WHEN $4 THEN NULL ELSE CURRENT_TIMESTAMP + INTERVAL '24 hours' END
            "#,
        )
        .bind(device_id)
        .bind(user_id)
        .bind(reason)
        .bind(is_permanent)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SecurityMetrics {
    pub total_events: u64,
    pub successful_events: u64,
    pub failed_events: u64,
    pub authentication_events: u64,
    pub authorization_events: u64,
    pub token_events: u64,
    pub connection_events: u64,
}

#[allow(dead_code)]
pub struct SecurityAnalyticsEngine {
    pool: PgPool,
    audit_logger: AuditLogger,
}

impl SecurityAnalyticsEngine {
    pub fn new(pool: PgPool, audit_logger: AuditLogger) -> Self {
        Self {
            pool,
            audit_logger,
        }
    }

    pub async fn analyze_events(
        &self,
        device_id: Option<Uuid>,
        user_id: Option<Uuid>,
        time_range_hours: Option<i64>,
    ) -> Result<SecurityMetrics, AuditError> {
        let hours = time_range_hours.unwrap_or(24);

        let (total_events, successful_events, failed_events, authentication_events, authorization_events, token_events, connection_events) =
            if let (Some(did), Some(uid)) = (device_id, user_id) {
                let row = sqlx::query(
                    r#"
                    SELECT
                        COUNT(*)::bigint as total_events,
                        COUNT(*) FILTER (WHERE status = 'success')::bigint as successful_events,
                        COUNT(*) FILTER (WHERE status = 'failure')::bigint as failed_events,
                        COUNT(*) FILTER (WHERE event_type = 'authentication')::bigint as authentication_events,
                        COUNT(*) FILTER (WHERE event_type = 'authorization')::bigint as authorization_events,
                        COUNT(*) FILTER (WHERE event_type = 'token')::bigint as token_events,
                        COUNT(*) FILTER (WHERE event_type = 'connection')::bigint as connection_events
                    FROM audit_logs
                    WHERE created_at > CURRENT_TIMESTAMP - INTERVAL '1 hour' * $1 AND device_id = $2 AND user_id = $3
                    "#,
                )
                .bind(hours)
                .bind(did)
                .bind(uid)
                .fetch_one(&self.pool)
                .await?;
                (
                    row.get::<i64, _>("total_events"),
                    row.get::<i64, _>("successful_events"),
                    row.get::<i64, _>("failed_events"),
                    row.get::<i64, _>("authentication_events"),
                    row.get::<i64, _>("authorization_events"),
                    row.get::<i64, _>("token_events"),
                    row.get::<i64, _>("connection_events"),
                )
            } else if let Some(did) = device_id {
                let row = sqlx::query(
                    r#"
                    SELECT
                        COUNT(*)::bigint as total_events,
                        COUNT(*) FILTER (WHERE status = 'success')::bigint as successful_events,
                        COUNT(*) FILTER (WHERE status = 'failure')::bigint as failed_events,
                        COUNT(*) FILTER (WHERE event_type = 'authentication')::bigint as authentication_events,
                        COUNT(*) FILTER (WHERE event_type = 'authorization')::bigint as authorization_events,
                        COUNT(*) FILTER (WHERE event_type = 'token')::bigint as token_events,
                        COUNT(*) FILTER (WHERE event_type = 'connection')::bigint as connection_events
                    FROM audit_logs
                    WHERE created_at > CURRENT_TIMESTAMP - INTERVAL '1 hour' * $1 AND device_id = $2
                    "#,
                )
                .bind(hours)
                .bind(did)
                .fetch_one(&self.pool)
                .await?;
                (
                    row.get::<i64, _>("total_events"),
                    row.get::<i64, _>("successful_events"),
                    row.get::<i64, _>("failed_events"),
                    row.get::<i64, _>("authentication_events"),
                    row.get::<i64, _>("authorization_events"),
                    row.get::<i64, _>("token_events"),
                    row.get::<i64, _>("connection_events"),
                )
            } else if let Some(uid) = user_id {
                let row = sqlx::query(
                    r#"
                    SELECT
                        COUNT(*)::bigint as total_events,
                        COUNT(*) FILTER (WHERE status = 'success')::bigint as successful_events,
                        COUNT(*) FILTER (WHERE status = 'failure')::bigint as failed_events,
                        COUNT(*) FILTER (WHERE event_type = 'authentication')::bigint as authentication_events,
                        COUNT(*) FILTER (WHERE event_type = 'authorization')::bigint as authorization_events,
                        COUNT(*) FILTER (WHERE event_type = 'token')::bigint as token_events,
                        COUNT(*) FILTER (WHERE event_type = 'connection')::bigint as connection_events
                    FROM audit_logs
                    WHERE created_at > CURRENT_TIMESTAMP - INTERVAL '1 hour' * $1 AND user_id = $2
                    "#,
                )
                .bind(hours)
                .bind(uid)
                .fetch_one(&self.pool)
                .await?;
                (
                    row.get::<i64, _>("total_events"),
                    row.get::<i64, _>("successful_events"),
                    row.get::<i64, _>("failed_events"),
                    row.get::<i64, _>("authentication_events"),
                    row.get::<i64, _>("authorization_events"),
                    row.get::<i64, _>("token_events"),
                    row.get::<i64, _>("connection_events"),
                )
            } else {
                let row = sqlx::query(
                    r#"
                    SELECT
                        COUNT(*)::bigint as total_events,
                        COUNT(*) FILTER (WHERE status = 'success')::bigint as successful_events,
                        COUNT(*) FILTER (WHERE status = 'failure')::bigint as failed_events,
                        COUNT(*) FILTER (WHERE event_type = 'authentication')::bigint as authentication_events,
                        COUNT(*) FILTER (WHERE event_type = 'authorization')::bigint as authorization_events,
                        COUNT(*) FILTER (WHERE event_type = 'token')::bigint as token_events,
                        COUNT(*) FILTER (WHERE event_type = 'connection')::bigint as connection_events
                    FROM audit_logs
                    WHERE created_at > CURRENT_TIMESTAMP - INTERVAL '1 hour' * $1
                    "#,
                )
                .bind(hours)
                .fetch_one(&self.pool)
                .await?;
                (
                    row.get::<i64, _>("total_events"),
                    row.get::<i64, _>("successful_events"),
                    row.get::<i64, _>("failed_events"),
                    row.get::<i64, _>("authentication_events"),
                    row.get::<i64, _>("authorization_events"),
                    row.get::<i64, _>("token_events"),
                    row.get::<i64, _>("connection_events"),
                )
            };

        Ok(SecurityMetrics {
            total_events: total_events as u64,
            successful_events: successful_events as u64,
            failed_events: failed_events as u64,
            authentication_events: authentication_events as u64,
            authorization_events: authorization_events as u64,
            token_events: token_events as u64,
            connection_events: connection_events as u64,
        })
    }
}
