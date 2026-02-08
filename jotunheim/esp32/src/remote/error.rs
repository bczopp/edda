//! Remote command errors (Phase 5.1.1).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RemoteCommandError {
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Loki client error: {0}")]
    LokiClient(#[from] crate::grpc::LokiClientError),
}
