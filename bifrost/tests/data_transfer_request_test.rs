//! Tests for Phase 12.2.1: Data Transfer Request Handler (receive request, forward to Heimdall).

use bifrost::guest::data_transfer::{
    DataTransferRequest, DataTransferRequestHandler, DataTransferResult, HeimdallConfirmationStub,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn sample_request() -> DataTransferRequest {
    DataTransferRequest {
        request_id: "req-1".to_string(),
        guest_device_id: "device-guest".to_string(),
        target_user_id: "user-1".to_string(),
        mesh_id: "main".to_string(),
    }
}

#[test]
fn handler_forwards_request_to_heimdall_stub() {
    let forwarded = Arc::new(AtomicBool::new(false));
    let stub = HeimdallConfirmationStub::new(Arc::clone(&forwarded), None);
    let handler = DataTransferRequestHandler::new(Arc::new(stub));
    let req = sample_request();
    let _ = handler.handle(req.clone());
    assert!(forwarded.load(Ordering::SeqCst));
}

#[test]
fn handler_returns_ok_when_stub_returns_allow() {
    let stub = HeimdallConfirmationStub::new(Arc::new(AtomicBool::new(false)), Some(true));
    let handler = DataTransferRequestHandler::new(Arc::new(stub));
    let req = sample_request();
    let result = handler.handle(req);
    assert!(matches!(result, Ok(DataTransferResult::Allowed(_))));
}

#[test]
fn handler_returns_denied_when_stub_returns_deny() {
    let stub = HeimdallConfirmationStub::new(Arc::new(AtomicBool::new(false)), Some(false));
    let handler = DataTransferRequestHandler::new(Arc::new(stub));
    let req = sample_request();
    let result = handler.handle(req);
    assert!(matches!(result, Ok(DataTransferResult::Denied)));
}
