//! ErrorHandler (Phase 8.2.1, TDD).

use std::error::Error;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ErrorKind {
    #[error("gRPC: {0}")]
    Grpc(String),
    #[error("Network: {0}")]
    Network(String),
    #[error("Resource: {0}")]
    Resource(String),
}

/// Categorizes and optionally logs errors (gRPC, network, resource).
pub struct ErrorHandler {
    last_error: Mutex<Option<String>>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {
            last_error: Mutex::new(None),
        }
    }

    pub fn categorize(&self, err: &(dyn Error + 'static)) -> ErrorKind {
        if let Some(io) = err.downcast_ref::<std::io::Error>() {
            let msg = io.to_string();
            match io.kind() {
                std::io::ErrorKind::ConnectionRefused
                | std::io::ErrorKind::ConnectionReset
                | std::io::ErrorKind::TimedOut
                | std::io::ErrorKind::BrokenPipe => return ErrorKind::Network(msg),
                _ => return ErrorKind::Resource(msg),
            }
        }
        let msg = err.to_string();
        if msg.to_lowercase().contains("grpc") || msg.to_lowercase().contains("tonic") {
            ErrorKind::Grpc(msg)
        } else if msg.to_lowercase().contains("connection") || msg.to_lowercase().contains("refused") {
            ErrorKind::Network(msg)
        } else {
            ErrorKind::Resource(msg)
        }
    }

    pub fn is_recoverable(&self, err: &(dyn Error + 'static)) -> bool {
        matches!(self.categorize(err), ErrorKind::Network(_))
    }

    pub fn handle(&self, err: &(dyn Error + 'static)) {
        let _ = self.categorize(err);
        *self.last_error.lock().unwrap() = Some(err.to_string());
    }

    pub fn last_error(&self) -> Option<String> {
        self.last_error.lock().unwrap().clone()
    }
}

impl Default for ErrorHandler {
    fn default() -> Self {
        Self::new()
    }
}
