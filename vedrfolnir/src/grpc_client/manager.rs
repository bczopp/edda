use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GrpcClientError {
    #[error("gRPC client error: {0}")]
    ClientError(String),
}

pub struct GrpcClientManager {
    channel: Option<Channel>,
}

impl GrpcClientManager {
    pub fn new() -> Self {
        Self { channel: None }
    }

    pub async fn connect(&mut self, url: &str) -> Result<(), GrpcClientError> {
        let channel = Channel::from_shared(url.to_string())
            .map_err(|e| GrpcClientError::ClientError(format!("{}", e)))?
            .connect()
            .await
            .map_err(|e| GrpcClientError::ClientError(format!("{}", e)))?;
        self.channel = Some(channel);
        Ok(())
    }
}
