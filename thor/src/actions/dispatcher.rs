use crate::actions::{ActionContext, ActionError, ActionRegistry};
use crate::audit::AuditLogger;
use crate::permissions::PermissionChecker;
use std::sync::Arc;

pub struct ActionDispatcher {
    registry: Arc<ActionRegistry>,
    permission_checker: Arc<PermissionChecker>,
    audit_logger: Option<Arc<dyn AuditLogger>>,
}

impl ActionDispatcher {
    pub fn new(
        registry: Arc<ActionRegistry>,
        permission_checker: Arc<PermissionChecker>,
    ) -> Self {
        Self {
            registry,
            permission_checker,
            audit_logger: None,
        }
    }

    pub fn new_with_audit(
        registry: Arc<ActionRegistry>,
        permission_checker: Arc<PermissionChecker>,
        audit_logger: Arc<dyn AuditLogger>,
    ) -> Self {
        Self {
            registry,
            permission_checker,
            audit_logger: Some(audit_logger),
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

        let result = executor.execute(context, action_data).await;
        if let Some(logger) = &self.audit_logger {
            let (success, err_msg) = match &result {
                Ok(_) => (true, None),
                Err(e) => (false, Some(e.to_string().as_str())),
            };
            logger.log_result(context, action_type, success, err_msg).await;
        }
        result
    }
}
