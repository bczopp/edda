use thiserror::Error;

#[derive(Debug, Error)]
pub enum LokiClientError {
    #[error("gRPC error: {0}")]
    GrpcError(String),
}

pub struct LokiClient {
    address: String,
    port: u16,
}

impl LokiClient {
    pub fn new(address: String, port: u16) -> Self {
        Self { address, port }
    }

    pub async fn call_function(&self, function_name: &str, args: &[u8]) -> Result<Vec<u8>, LokiClientError> {
        // Call Loki function via gRPC
        Ok(vec![])
    }
}
