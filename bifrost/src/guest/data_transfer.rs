//! Data Transfer Request Handler (Phase 12.2.1). Receives request from guest device, forwards to Heimdall for user confirmation.

use std::sync::Arc;

/// Data transfer request from a guest device (explicit access to main mesh or user data).
#[derive(Debug, Clone)]
pub struct DataTransferRequest {
    pub request_id: String,
    pub guest_device_id: String,
    pub target_user_id: String,
    pub mesh_id: String,
}

/// Result of forwarding a data transfer request to Heimdall (user confirmation).
#[derive(Debug, PartialEq)]
pub enum DataTransferResult {
    Allowed(String),
    Denied,
}

/// Provider that forwards the request to Heimdall (gRPC) for user confirmation; stub for tests.
pub trait HeimdallConfirmationProvider: Send + Sync {
    fn request_confirmation(&self, req: &DataTransferRequest) -> Result<DataTransferResult, Box<dyn std::error::Error + Send + Sync>>;
}

/// Stub for tests: records that request was forwarded and returns configurable Allow/Deny.
pub struct HeimdallConfirmationStub {
    forwarded: Arc<std::sync::atomic::AtomicBool>,
    allow: Option<bool>,
}

impl HeimdallConfirmationStub {
    pub fn new(
        forwarded: Arc<std::sync::atomic::AtomicBool>,
        allow: Option<bool>,
    ) -> Self {
        Self { forwarded, allow }
    }
}

impl HeimdallConfirmationProvider for HeimdallConfirmationStub {
    fn request_confirmation(&self, _req: &DataTransferRequest) -> Result<DataTransferResult, Box<dyn std::error::Error + Send + Sync>> {
        self.forwarded.store(true, std::sync::atomic::Ordering::SeqCst);
        match self.allow {
            Some(true) => Ok(DataTransferResult::Allowed("stub-token".to_string())),
            Some(false) => Ok(DataTransferResult::Denied),
            None => Ok(DataTransferResult::Denied),
        }
    }
}

/// Receives data transfer request from guest device and forwards it to Heimdall for user confirmation.
pub struct DataTransferRequestHandler {
    heimdall: Arc<dyn HeimdallConfirmationProvider>,
}

impl DataTransferRequestHandler {
    pub fn new(heimdall: Arc<dyn HeimdallConfirmationProvider>) -> Self {
        Self { heimdall }
    }

    pub fn handle(
        &self,
        req: DataTransferRequest,
    ) -> Result<DataTransferResult, Box<dyn std::error::Error + Send + Sync>> {
        self.heimdall.request_confirmation(&req)
    }
}
