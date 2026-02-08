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

    /// Generate action plan from request text (XML Task-based).
    pub async fn plan_actions(&self, request: &str) -> Result<ActionPlan, Box<dyn std::error::Error + Send + Sync>> {
        let request_lower = request.to_lowercase();
        let mut actions = Vec::new();

        // High-level intent: File Collection
        if (request_lower.contains("list") || request_lower.contains("show") || request_lower.contains("open")) && request_lower.contains("file") {
            let file_name = self.extract_file_name(request);
            let location = if file_name == "unknown.txt" { "." } else { &file_name };
            
            let xml_task = format!(
                "<task><collection type=\"file\" location=\"{}\"></collection></task>",
                location
            );

            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({
                    "xml": xml_task
                }),
            });
        }

        // High-level intent: Process Collection
        if request_lower.contains("list") && (request_lower.contains("process") || request_lower.contains("tasks")) {
            let xml_task = "<task><collection type=\"process\"></collection></task>".to_string();
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({ "xml": xml_task }),
            });
        }

        // High-level intent: Network Collection
        if request_lower.contains("network") || request_lower.contains("ip") || request_lower.contains("connection") {
            let xml_task = "<task><collection type=\"network\"></collection></task>".to_string();
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({ "xml": xml_task }),
            });
        }

        // High-level intent: Analysis Collection (Resource/Performance)
        if request_lower.contains("analyze") || request_lower.contains("status") || request_lower.contains("performance") || request_lower.contains("usage") {
            let xml_task = "<task><analysis type=\"resource\"></analysis></task>".to_string();
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({ "xml": xml_task }),
            });
        }

        // High-level intent: Instruction/Command
        if request_lower.contains("run") || request_lower.contains("execute") {
            let command = self.extract_command(request);
            let xml_task = format!(
                "<task><instruction>Execute the following command: {}</instruction></task>",
                command
            );

            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({
                    "xml": xml_task
                }),
            });
        }

        // High-level intent: Log Collection
        if request_lower.contains("log") || request_lower.contains("journal") {
            let xml_task = "<task><collection type=\"logs\"></collection></task>".to_string();
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({ "xml": xml_task }),
            });
        }

        // High-level intent: Security Analysis
        if request_lower.contains("security") || request_lower.contains("port") || request_lower.contains("firewall") {
            let xml_task = "<task><analysis type=\"security\"></analysis></task>".to_string();
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({ "xml": xml_task }),
            });
        }

        // High-level intent: Hardware Analysis
        if request_lower.contains("hardware") || request_lower.contains("cpu") || request_lower.contains("memory") || request_lower.contains("specs") {
            let xml_task = "<task><analysis type=\"hardware\"></analysis></task>".to_string();
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({ "xml": xml_task }),
            });
        }

        // Fallback for unspecified intents
        if actions.is_empty() {
            let xml_task = format!(
                "<task><instruction>Determine how to handle: {}</instruction></task>",
                request
            );
            actions.push(Action {
                action_id: Uuid::new_v4().to_string(),
                action_type: "XML_TASK".to_string(),
                service: "thor".to_string(),
                parameters: serde_json::json!({
                    "xml": xml_task
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

                        // Post-process XML response if present
                        if result_data.starts_with("<response") {
                            results.push(self.parse_xml_response(&result_data));
                        } else {
                            results.push(result_data);
                        }
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

    /// Parses a standard XML response and returns the payload or error.
    fn parse_xml_response(&self, xml: &str) -> String {
        // Simple extraction logic for demonstration
        if let Some(payload_start) = xml.find("<payload>") {
            if let Some(payload_end) = xml.find("</payload>") {
                return xml[payload_start + 9..payload_end].to_string();
            }
        }
        
        if xml.contains("status=\"error\"") {
             return format!("Error in XML response: {}", xml);
        }

        xml.to_string()
    }

    /// Convert internal Action to ThorAction proto
    fn convert_to_thor_action(&self, action: Action) -> Result<crate::clients::thor::thor::ThorAction, Box<dyn std::error::Error + Send + Sync>> {
        // For XML types, we want the raw XML string as action_data
        let action_data = if action.action_type == "XML_TASK" || action.action_type == "XML_CALL" {
            action.parameters["xml"].as_str().unwrap_or_default().as_bytes().to_vec()
        } else {
            // Serialize parameters to JSON bytes for standard actions
            serde_json::to_vec(&action.parameters)
                .map_err(|e| format!("Failed to serialize action parameters: {}", e))?
        };

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
