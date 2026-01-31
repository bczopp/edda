use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;

pub mod jotunheim {
    tonic::include_proto!("jotunheim");
}

use jotunheim::jotunheim_service_client::JotunheimServiceClient;
use jotunheim::{DeviceCommandRequest, DeviceCommandResponse, DeviceStatusRequest, DeviceStatusResponse, ToolCallRequest, ToolCallResponse};

/// Client for Jotunheim service (IoT Platform)
pub struct JotunheimClient {
    client: JotunheimServiceClient<Channel>,
}

impl JotunheimClient {
    /// Create a new Jotunheim client
    pub async fn new(url: String) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(url)?
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = JotunheimServiceClient::new(channel);
        
        Ok(Self { client })
    }

    /// Send command to IoT device
    pub async fn send_device_command(&mut self, request: DeviceCommandRequest) -> Result<DeviceCommandResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.send_device_command(req).await?;
        Ok(response.into_inner())
    }

    /// Get device status
    pub async fn get_device_status(&mut self, request: DeviceStatusRequest) -> Result<DeviceStatusResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.get_device_status(req).await?;
        Ok(response.into_inner())
    }

    /// Call tool on IoT device
    pub async fn call_tool(&mut self, request: ToolCallRequest) -> Result<ToolCallResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.call_tool(req).await?;
        Ok(response.into_inner())
    }
}
