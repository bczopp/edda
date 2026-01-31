use crate::actions::{ActionExecutor, ActionContext, ActionError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequestParams {
    pub url: String,
    pub method: String,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub body: Option<String>,
}

pub struct NetworkActionExecutor;

#[async_trait]
impl ActionExecutor for NetworkActionExecutor {
    fn action_type(&self) -> &str {
        "NETWORK_OPERATION"
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let params: HttpRequestParams = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Invalid HTTP request: {}", e)))?;

        let client = reqwest::Client::new();
        let mut request = match params.method.as_str() {
            "GET" => client.get(&params.url),
            "POST" => client.post(&params.url),
            "PUT" => client.put(&params.url),
            "DELETE" => client.delete(&params.url),
            _ => return Err(ActionError::InvalidAction(format!("Unsupported HTTP method: {}", params.method))),
        };

        if let Some(ref headers) = params.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        if let Some(ref body) = params.body {
            request = request.body(body.clone());
        }

        let response = request.send().await
            .map_err(|e| ActionError::ExecutionFailed(format!("HTTP request failed: {}", e)))?;

        let status = response.status();
        let body = response.text().await
            .map_err(|e| ActionError::ExecutionFailed(format!("Failed to read response: {}", e)))?;

        let result = serde_json::json!({
            "status_code": status.as_u16(),
            "body": body,
        });

        Ok(serde_json::to_vec(&result).unwrap())
    }
}
