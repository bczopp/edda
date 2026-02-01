//! Audit logging for action execution (Phase 10).
//! Trait allows swapping in different backends (tracing, external service).

use crate::actions::ActionContext;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait AuditLogger: Send + Sync {
    /// Log that an action was dispatched.
    async fn log_dispatch(&self, context: &ActionContext, action_type: &str);
    /// Log the result of an action (success or failure).
    async fn log_result(
        &self,
        context: &ActionContext,
        action_type: &str,
        success: bool,
        error_message: Option<&str>,
    );
}

/// Audit logger that writes to tracing (info level).
pub struct TracingAuditLogger;

impl TracingAuditLogger {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl Default for TracingAuditLogger {
    fn default() -> Self {
        Self
    }
}

#[async_trait]
impl AuditLogger for TracingAuditLogger {
    async fn log_dispatch(&self, context: &ActionContext, action_type: &str) {
        tracing::info!(
            device_id = %context.device_id,
            user_id = %context.user_id,
            action_id = %context.action_id,
            action_type = %action_type,
            "action_dispatch"
        );
    }

    async fn log_result(
        &self,
        context: &ActionContext,
        action_type: &str,
        success: bool,
        error_message: Option<&str>,
    ) {
        if success {
            tracing::info!(
                device_id = %context.device_id,
                action_type = %action_type,
                "action_result success"
            );
        } else {
            tracing::warn!(
                device_id = %context.device_id,
                action_type = %action_type,
                error = ?error_message,
                "action_result failure"
            );
        }
    }
}
