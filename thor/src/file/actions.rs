use crate::actions::{ActionExecutor, ActionContext, ActionError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReadParams {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileWriteParams {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDeleteParams {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMoveParams {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation")]
pub enum FileOperation {
    Read(FileReadParams),
    Write(FileWriteParams),
    Delete(FileDeleteParams),
    Move(FileMoveParams),
    Copy { from: String, to: String },
}

pub struct FileActionExecutor;

#[async_trait]
impl ActionExecutor for FileActionExecutor {
    fn action_type(&self) -> &str {
        "FILE_OPERATION"
    }

    async fn execute(
        &self,
        _context: &ActionContext,
        action_data: &[u8],
    ) -> Result<Vec<u8>, ActionError> {
        let operation: FileOperation = serde_json::from_slice(action_data)
            .map_err(|e| ActionError::InvalidAction(format!("Invalid file operation: {}", e)))?;

        match operation {
            FileOperation::Read(params) => {
                let content = fs::read_to_string(&params.path).await
                    .map_err(|e| ActionError::ExecutionFailed(format!("Failed to read file: {}", e)))?;
                Ok(serde_json::to_vec(&serde_json::json!({ "content": content })).unwrap())
            }
            FileOperation::Write(params) => {
                fs::write(&params.path, params.content).await
                    .map_err(|e| ActionError::ExecutionFailed(format!("Failed to write file: {}", e)))?;
                Ok(serde_json::to_vec(&serde_json::json!({ "success": true })).unwrap())
            }
            FileOperation::Delete(params) => {
                fs::remove_file(&params.path).await
                    .map_err(|e| ActionError::ExecutionFailed(format!("Failed to delete file: {}", e)))?;
                Ok(serde_json::to_vec(&serde_json::json!({ "success": true })).unwrap())
            }
            FileOperation::Move(params) => {
                fs::rename(&params.from, &params.to).await
                    .map_err(|e| ActionError::ExecutionFailed(format!("Failed to move file: {}", e)))?;
                Ok(serde_json::to_vec(&serde_json::json!({ "success": true })).unwrap())
            }
            FileOperation::Copy { from, to } => {
                fs::copy(&from, &to).await
                    .map_err(|e| ActionError::ExecutionFailed(format!("Failed to copy file: {}", e)))?;
                Ok(serde_json::to_vec(&serde_json::json!({ "success": true })).unwrap())
            }
        }
    }
}
