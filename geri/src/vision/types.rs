//! Gemeinsame Typen für Vision (Request, Response, Error).

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionRequest {
    pub image_data: Vec<u8>,
    pub prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionResponse {
    pub description: String,
    pub analysis: serde_json::Value,
}

#[derive(Debug, Error)]
pub enum VisionError {
    #[error("Vision processing failed: {0}")]
    ProcessingFailed(String),
}
