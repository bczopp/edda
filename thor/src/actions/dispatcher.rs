use crate::actions::{ActionRegistry, ActionContext, ActionError};
use crate::permissions::PermissionChecker;
use std::sync::Arc;
use tracing::info;

pub struct ActionDispatcher {
    registry: Arc<ActionRegistry>,
    permission_checker: Arc<PermissionChecker>,
}

impl ActionDispatcher {
    pub fn new(
        registry: Arc<ActionRegistry>,
        permission_checker: Arc<PermissionChecker>,
    ) -> Self {
        Self {
            registry,
            permission_checker,
        }
    }

    pub async fn dispatch(
        &self,
        action_type: &str,
        context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        info!("Dispatching action: {} for device: {}", action_type, context.device_id);

        // Get executor
        let executor = self.registry
            .get(action_type)
            .await
            .ok_or_else(|| ActionError::InvalidAction(format!("Unknown action type: {}", action_type)))?;

        // Check permissions
        let has_permission = self.permission_checker
            .check_permission(
                &context.device_id,
                &context.user_id,
                "action",
                action_type,
            )
            .await
            .map_err(|e| ActionError::PermissionDenied(format!("{}", e)))?;

        if !has_permission {
            return Err(ActionError::PermissionDenied(
                format!("Device {} does not have permission for action {}", context.device_id, action_type)
            ));
        }

        // Execute action
        executor.execute(context, action_data).await
    }
}
