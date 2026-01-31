use crate::actions::{ActionExecutor, ActionContext, ActionError};
use crate::jotunheim::client::JotunheimClient;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value;
use async_trait::async_trait;

/// Jotunheim action handler for IoT device operations
pub struct JotunheimActionHandler {
    jotunheim_url: String,
    client: Arc<RwLock<Option<JotunheimClient>>>,
}

impl JotunheimActionHandler {
    pub fn new(jotunheim_url: String) -> Self {
        Self {
            jotunheim_url,
            client: Arc::new(RwLock::new(None)),
        }
    }

    async fn get_client(&self) -> Result<JotunheimClient, ActionError> {
        // Check if client exists
        {
            let client_guard = self.client.read().await;
            if let Some(ref _client) = *client_guard {
                // Client exists, but we can't clone it easily
                // We'll need to create a new one each time or use a connection pool
            }
        }
        
        // Create new client
        JotunheimClient::new(self.jotunheim_url.clone())
            .await
            .map_err(|e| ActionError::ExecutionFailed(format!("Failed to connect to Jotunheim: {}", e)))
    }

    fn parse_params(&self, action_data: &[u8]) -> Result<JotunheimParams, ActionError> {
        let value: Value = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Failed to parse action data: {}", e)))?;
        
        let operation = value["operation"]
            .as_str()
            .ok_or_else(|| ActionError::InvalidAction("Missing 'operation' field".to_string()))?
            .to_string();
        
        Ok(JotunheimParams {
            operation,
            device_id: value["device_id"].as_str().map(|s| s.to_string()),
            command: value["command"].as_str().map(|s| s.to_string()),
            tool_name: value["tool_name"].as_str().map(|s| s.to_string()),
            parameters: value["parameters"].clone(),
        })
    }

    async fn execute_operation(&self, params: &JotunheimParams) -> Result<Value, ActionError> {
        let mut client = self.get_client().await?;
        
        match params.operation.as_str() {
            "device_command" => {
                let device_id = params.device_id.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'device_id' for device_command".to_string()))?;
                let command = params.command.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'command' for device_command".to_string()))?;
                
                let request = crate::jotunheim::client::jotunheim::DeviceCommandRequest {
                    device_id: device_id.clone(),
                    command: command.clone(),
                    parameters: serde_json::to_string(&params.parameters)
                        .map_err(|e| ActionError::InvalidAction(format!("Failed to serialize parameters: {}", e)))?,
                };
                
                let response = client.send_device_command(request).await
                    .map_err(|e| ActionError::ExecutionFailed(format!("Device command failed: {}", e)))?;
                
                Ok(serde_json::json!({
                    "success": response.success,
                    "result": response.result,
                    "error": response.error_message
                }))
            }
            "device_status" => {
                let device_id = params.device_id.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'device_id' for device_status".to_string()))?;
                
                let request = crate::jotunheim::client::jotunheim::DeviceStatusRequest {
                    device_id: device_id.clone(),
                };
                
                let response = client.get_device_status(request).await
                    .map_err(|e| ActionError::ExecutionFailed(format!("Device status query failed: {}", e)))?;
                
                Ok(serde_json::json!({
                    "device_id": response.device_id,
                    "status": response.status,
                    "capabilities": response.capabilities,
                    "last_seen": response.last_seen
                }))
            }
            "tool_call" => {
                let device_id = params.device_id.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'device_id' for tool_call".to_string()))?;
                let tool_name = params.tool_name.as_ref()
                    .ok_or_else(|| ActionError::InvalidAction("Missing 'tool_name' for tool_call".to_string()))?;
                
                let request = crate::jotunheim::client::jotunheim::ToolCallRequest {
                    device_id: device_id.clone(),
                    tool_name: tool_name.clone(),
                    parameters: serde_json::to_string(&params.parameters)
                        .map_err(|e| ActionError::InvalidAction(format!("Failed to serialize parameters: {}", e)))?,
                };
                
                let response = client.call_tool(request).await
                    .map_err(|e| ActionError::ExecutionFailed(format!("Tool call failed: {}", e)))?;
                
                Ok(serde_json::json!({
                    "success": response.success,
                    "result": response.result,
                    "error": response.error_message
                }))
            }
            _ => {
                Err(ActionError::InvalidAction(format!("Unknown Jotunheim operation: {}", params.operation)))
            }
        }
    }
}

#[derive(Debug)]
struct JotunheimParams {
    operation: String,
    device_id: Option<String>,
    command: Option<String>,
    tool_name: Option<String>,
    parameters: Value,
}

#[async_trait]
impl ActionExecutor for JotunheimActionHandler {
    fn action_type(&self) -> &str {
        "JOTUNHEIM_OPERATION"
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let params = self.parse_params(action_data)?;
        
        let result = self.execute_operation(&params).await?;
        
        serde_json::to_vec(&result)
            .map_err(|e| ActionError::ExecutionFailed(format!("Failed to serialize result: {}", e)))
    }
}
