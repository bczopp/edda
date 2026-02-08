//! Odin service integration (Phase 4): Chat/Process via Odin gRPC client.

use crate::grpc_client::odin_client::odin::ProcessRequest;
use crate::grpc_client::{OdinClient, OdinClientError};
use uuid::Uuid;

/// Odin service integration: wraps OdinClient for chat/process flow.
#[derive(Clone)]
pub struct OdinServiceIntegration {
    client: OdinClient,
}

impl OdinServiceIntegration {
    pub async fn new(port: u16) -> Result<Self, OdinClientError> {
        let client = OdinClient::new(port).await?;
        Ok(Self { client })
    }

    /// Send chat message to Odin and return full response.
    pub async fn send_chat(&mut self, message: &str) -> Result<crate::grpc_client::odin_client::odin::ProcessResponse, OdinClientError> {
        let request = ProcessRequest {
            request_id: Uuid::new_v4().to_string(),
            user_id: String::new(),
            device_id: String::new(),
            input: message.to_string(),
            input_type: "text".to_string(),
        };
        let response = self.client.process_request(request).await?;
        Ok(response)
    }
}
