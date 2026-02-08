use crate::actions::{ActionContext, ActionError, ActionRegistry};
use crate::audit::AuditLogger;
use crate::permissions::PermissionChecker;
use std::sync::Arc;

pub struct ActionDispatcher {
    registry: Arc<ActionRegistry>,
    permission_checker: Arc<PermissionChecker>,
    audit_logger: Option<Arc<dyn AuditLogger>>,
    strict_sandboxing: bool,
}

impl ActionDispatcher {
    pub fn new(
        registry: Arc<ActionRegistry>,
        permission_checker: Arc<PermissionChecker>,
        strict_sandboxing: bool,
    ) -> Self {
        Self {
            registry,
            permission_checker,
            audit_logger: None,
            strict_sandboxing,
        }
    }

    pub fn new_with_audit(
        registry: Arc<ActionRegistry>,
        permission_checker: Arc<PermissionChecker>,
        audit_logger: Arc<dyn AuditLogger>,
        strict_sandboxing: bool,
    ) -> Self {
        Self {
            registry,
            permission_checker,
            audit_logger: Some(audit_logger),
            strict_sandboxing,
        }
    }

    pub async fn dispatch(
        &self,
        action_type: &str,
        context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        if let Some(logger) = &self.audit_logger {
            logger.log_dispatch(context, action_type).await;
        }

        let executor = self
            .registry
            .get(action_type)
            .await
            .ok_or_else(|| ActionError::InvalidAction(format!("Unknown action type: {}", action_type)))?;

        let has_permission = self
            .permission_checker
            .check_permission(&context.device_id, &context.user_id, "action", action_type)
            .await
            .map_err(|e| ActionError::PermissionDenied(format!("{}", e)))?;

        if !has_permission {
            if let Some(logger) = &self.audit_logger {
                logger
                    .log_result(context, action_type, false, Some("permission denied"))
                    .await;
            }
            return Err(ActionError::PermissionDenied(format!(
                "Device {} does not have permission for action {}",
                context.device_id, action_type
            )));
        }

        // Enforcement: If strict_sandboxing is enabled, only allow sandboxed executors
        if self.strict_sandboxing && !executor.is_sandboxed() {
            let err_msg = format!("Action {} is blocked: strict sandboxing is enabled and this executor is not sandboxed", action_type);
            if let Some(logger) = &self.audit_logger {
                logger.log_result(context, action_type, false, Some(&err_msg)).await;
            }
            return Err(ActionError::PermissionDenied(err_msg));
        }

        let result = executor.execute(context, action_data).await;
        if let Some(logger) = &self.audit_logger {
            let err_msg = result.as_ref().err().map(|e| e.to_string());
            let (success, err_msg_ref) = match &result {
                Ok(_) => (true, None),
                Err(_) => (false, err_msg.as_deref()),
            };
            logger.log_result(context, action_type, success, err_msg_ref).await;
        }
        result
    }
}
