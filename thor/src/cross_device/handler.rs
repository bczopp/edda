use crate::actions::{ActionExecutor, ActionContext};
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;

pub mod cross_device {
    tonic::include_proto!("cross_device");
}

use cross_device::cross_device_service_client::CrossDeviceServiceClient;
use cross_device::{CrossDeviceActionRequest, CrossDeviceActionResult};

/// Handler for cross-device actions
/// Receives actions via gRPC from other devices, executes them, and sends results back
pub struct CrossDeviceActionHandler {
    // gRPC client for sending results back to source device
    clients: Arc<RwLock<std::collections::HashMap<String, CrossDeviceServiceClient<Channel>>>>,
    local_executor: Arc<dyn ActionExecutor>,
}

impl CrossDeviceActionHandler {
    pub fn new(local_executor: Arc<dyn ActionExecutor>) -> Self {
        Self {
            clients: Arc::new(RwLock::new(std::collections::HashMap::new())),
            local_executor,
        }
    }

    /// Handle incoming cross-device action
    pub async fn handle_action(
        &self,
        request: CrossDeviceActionRequest,
    ) -> Result<CrossDeviceActionResult> {
        // Create action context
        let context = ActionContext {
            device_id: request.target_device_id.clone(),
            user_id: request.user_id.clone(),
            action_id: request.action_id.clone(),
        };

        // Execute action locally
        let result = self.local_executor
            .execute(&context, &request.action_data)
            .await
            .map_err(|e| anyhow::anyhow!("Action execution failed: {}", e))?;

        // Create result (assuming success if no error)
        Ok(CrossDeviceActionResult {
            action_id: request.action_id,
            success: true,
            result_data: result,
            error_message: String::new(),
        })
    }

    /// Send action to another device
    pub async fn send_action(
        &self,
        target_device_url: String,
        request: CrossDeviceActionRequest,
    ) -> Result<CrossDeviceActionResult> {
        // Check if client exists, if not create it
        let client = {
            let clients = self.clients.read().await;
            clients.get(&target_device_url).cloned()
        };
        
        let mut client = if let Some(c) = client {
            c
        } else {
            // Create new client
            let endpoint = tonic::transport::Endpoint::from_shared(target_device_url.clone())?
                .timeout(Duration::from_secs(30))
                .connect_timeout(Duration::from_secs(5));
            
            let channel = endpoint.connect().await?;
            let new_client = CrossDeviceServiceClient::new(channel);
            
            // Store client
            let mut clients = self.clients.write().await;
            clients.insert(target_device_url.clone(), new_client.clone());
            new_client
        };

        // Send action
        let req = tonic::Request::new(request);
        let response = client.execute_cross_device_action(req).await?;
        Ok(response.into_inner())
    }

    /// Connect to a device for cross-device communication
    pub async fn connect_device(&self, device_url: String) -> Result<()> {
        let endpoint = tonic::transport::Endpoint::from_shared(device_url.clone())?
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = CrossDeviceServiceClient::new(channel);
        
        let mut clients = self.clients.write().await;
        clients.insert(device_url, client);
        
        Ok(())
    }
}
