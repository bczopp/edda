use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Ordered list of actions produced by [`ActionOrchestrator::plan_actions`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPlan {
    pub actions: Vec<Action>,
}

/// Single action to be executed by a service (e.g. Thor).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_id: String,
    pub action_type: String,
    /// Target service: `"thor"`, `"freki"`, `"geri"`, etc.
    pub service: String,
    pub parameters: serde_json::Value,
}

/// Plans and executes actions via Thor; maps request text to actions.
pub struct ActionOrchestrator {
    client_manager: Option<Arc<crate::clients::manager::ClientManager>>,
}

impl ActionOrchestrator {
    /// Orchestrator without Thor client; `execute_actions` yields "Client manager not available".
    pub fn new() -> Self {
        Self {
            client_manager: None,
        }
    }

    /// Orchestrator with Thor client for executing actions.
    pub fn new_with_client(client_manager: Arc<crate::clients::manager::ClientManager>) -> Self {
        Self {
            client_manager: Some(client_manager),
        }
    }

    /// Generate action plan from request text (keyword-based: file, terminal, app, fallback).
    pub async fn plan_actions(&self, request: &str) -> Result<ActionPlan, Box<dyn std::error::Error + Send + Sync>> {
        let request_lower = request.to_lowercase();
        let mut actions = Vec::new();

        // Detect file operations
        if request_lower.contains("open") && request_lower.contains("file") {
            let file_name = self.extract_file_name(request);
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "FILE_OPERATION".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({
                    "operation": "open",
                    "file": file_name
                }),
            });
        }

        // Detect terminal/command operations
        if request_lower.contains("run") && request_lower.contains("command") 
            || request_lower.contains("execute") 
            || request_lower.starts_with("ls") 
            || request_lower.starts_with("cd") 
            || request_lower.starts_with("mkdir") {
            let command = self.extract_command(request);
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "TERMINAL_OPERATION".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({
                    "command": command
                }),
            });
        }

        // Detect app control operations
        if request_lower.contains("launch") || request_lower.contains("start") || request_lower.contains("open app") {
            let app_name = self.extract_app_name(request);
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "APP_CONTROL".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({
                    "operation": "launch",
                    "app": app_name
                }),
            });
        }

        // If no specific actions detected, create a generic action
        if actions.is_empty() {
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "SYSTEM_COMMAND".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({
                    "request": request
                }),
            });
        }

        Ok(ActionPlan { actions })
    }

    /// Extract file name from request
    fn extract_file_name(&self, request: &str) -> String {
        // Simple extraction: look for quoted strings or words after "file"
        let request_lower = request.to_lowercase();
        if let Some(pos) = request_lower.find("file") {
            let after_file = &request[pos + 4..];
            // Try to find quoted string
            if let Some(start) = after_file.find('"') {
                if let Some(end) = after_file[start + 1..].find('"') {
                    return after_file[start + 1..start + 1 + end].to_string();
                }
            }
            // Otherwise, take next word
            let words: Vec<&str> = after_file.split_whitespace().collect();
            if !words.is_empty() {
                return words[0].to_string();
            }
        }
        "unknown.txt".to_string()
    }

    /// Extract command from request
    fn extract_command(&self, request: &str) -> String {
        // Look for command after "command:" or quoted strings
        let request_lower = request.to_lowercase();
        if let Some(pos) = request_lower.find("command:") {
            return request[pos + 8..].trim().to_string();
        }
        if let Some(start) = request.find('"') {
            if let Some(end) = request[start + 1..].find('"') {
                return request[start + 1..start + 1 + end].to_string();
            }
        }
        // Extract first command-like word sequence
        let words: Vec<&str> = request.split_whitespace().collect();
        if words.len() > 1 {
            return words[1..].join(" ");
        }
        request.to_string()
    }

    /// Extract app name from request
    fn extract_app_name(&self, request: &str) -> String {
        let request_lower = request.to_lowercase();
        if let Some(pos) = request_lower.find("app") {
            let after_app = &request[pos + 3..];
            let words: Vec<&str> = after_app.split_whitespace().collect();
            if !words.is_empty() {
                return words[0].to_string();
            }
        }
        "unknown".to_string()
    }

    /// Execute actions via Thor; returns one string per action (success/error/skip).
    pub async fn execute_actions(&self, plan: ActionPlan) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();

        for action in plan.actions {
            // Only execute Thor actions
            if action.service != "thor" {
                results.push(format!("Skipping non-Thor action: {}", action.service));
                continue;
            }

            // Get client manager
            let client_manager = match &self.client_manager {
                Some(cm) => cm,
                None => {
                    results.push(format!("Client manager not available for action {}", action.action_id));
                    continue;
                }
            };

            // Convert Action to ThorAction
            let thor_action = self.convert_to_thor_action(action)?;

            // Execute via Thor
            match client_manager.execute_thor_action(thor_action).await {
                Ok(thor_result) => {
                    if thor_result.success {
                        let result_data = if !thor_result.result_data.is_empty() {
                            String::from_utf8_lossy(&thor_result.result_data).to_string()
                        } else {
                            "Action completed successfully".to_string()
                        };
                        results.push(result_data);
                    } else {
                        results.push(format!("Action failed: {}", thor_result.error_message));
                    }
                }
                Err(e) => {
                    results.push(format!("Failed to execute action: {}", e));
                }
            }
        }

        Ok(results)
    }

    /// Convert internal Action to ThorAction proto
    fn convert_to_thor_action(&self, action: Action) -> Result<crate::clients::thor::thor::ThorAction, Box<dyn std::error::Error + Send + Sync>> {
        // Serialize parameters to JSON bytes
        let action_data = serde_json::to_vec(&action.parameters)
            .map_err(|e| format!("Failed to serialize action parameters: {}", e))?;

        Ok(crate::clients::thor::thor::ThorAction {
            action_id: action.action_id,
            action_type: action.action_type,
            device_id: String::new(), // Will be set by caller if needed
            user_id: String::new(),    // Will be set by caller if needed
            action_data,
            metadata: std::collections::HashMap::new(),
        })
    }
}
