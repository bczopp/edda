use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ForwardingError {
    #[error("Forwarding failed: {0}")]
    ForwardingFailed(String),
    #[error("gRPC error: {0}")]
    GrpcError(String),
}

pub struct DataForwarder {
    odin_url: String,
    client: Option<tonic::client::Grpc<Channel>>,
}

impl DataForwarder {
    pub fn new(odin_url: String) -> Self {
        Self {
            odin_url,
            client: None,
        }
    }

    async fn get_client(&mut self) -> Result<tonic::client::Grpc<Channel>, ForwardingError> {
        if let Some(ref client) = self.client {
            // Client exists, but we can't easily clone it
            // For now, create new client each time (in production would use connection pool)
        }
        
        let endpoint = tonic::transport::Endpoint::from_shared(self.odin_url.clone())
            .map_err(|e| ForwardingError::GrpcError(format!("Invalid URL: {}", e)))?
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await
            .map_err(|e| ForwardingError::GrpcError(format!("Connection failed: {}", e)))?;
        
        Ok(tonic::client::Grpc::new(channel))
    }

    pub async fn forward_to_odin(&mut self, data: &[u8], data_type: &str) -> Result<(), ForwardingError> {
        // Forward to Odin via gRPC
        // In a real implementation, this would:
        // 1. Create gRPC client to Odin
        // 2. Create ForwardDataRequest with data and type
        // 3. Send request via gRPC
        // 4. Handle response
        
        tracing::info!("Forwarding {} data ({} bytes) to Odin", data_type, data.len());
        
        // For now, log the forwarding
        // In production, would use actual gRPC call to Odin's Huginn Data Service
        let _client = self.get_client().await?;
        
        // TODO: Implement actual gRPC call when Odin's Huginn Data Service is available
        // let request = odin::ForwardDataRequest {
        //     data: data.to_vec(),
        //     data_type: data_type.to_string(),
        //     ...
        // };
        // client.forward_data(request).await?;
        
        Ok(())
    }

    pub async fn forward_text(&mut self, text: &str) -> Result<(), ForwardingError> {
        self.forward_to_odin(text.as_bytes(), "text").await
    }

    pub async fn forward_image(&mut self, image_data: &[u8]) -> Result<(), ForwardingError> {
        self.forward_to_odin(image_data, "image").await
    }

    pub async fn forward_video(&mut self, video_data: &[u8]) -> Result<(), ForwardingError> {
        self.forward_to_odin(video_data, "video").await
    }

    pub async fn forward_audio(&mut self, audio_data: &[u8]) -> Result<(), ForwardingError> {
        self.forward_to_odin(audio_data, "audio").await
    }
}
