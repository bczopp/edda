//! gRPC Bridge (Phase 17.1.1). Tunnel gRPC requests/responses over Bifrost WebSocket.

use crate::message::{BifrostMessage, MessageType};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tokio::sync::oneshot;

#[derive(Error, Debug)]
pub enum GrpcBridgeError {
    #[error("remote error: {0}")]
    RemoteError(String),
    #[error("timeout")]
    Timeout,
    #[error("parse error: {0}")]
    Parse(String),
    #[error("response channel closed")]
    ChannelClosed(#[from] oneshot::error::RecvError),
}

/// Parsed gRPC response payload from a Bifrost message.
#[derive(Debug)]
pub struct GrpcResponsePayload {
    pub request_id: String,
    pub body: Vec<u8>,
    pub ok: bool,
}

/// Tunnels gRPC requests over Bifrost; matches responses to pending requests.
pub struct GrpcBridge {
    router: Arc<crate::routing::MessageRouter>,
    timeout: std::time::Duration,
    pending: Arc<RwLock<HashMap<String, oneshot::Sender<Result<Vec<u8>, GrpcBridgeError>>>>>,
}

impl GrpcBridge {
    pub fn new(
        router: Arc<crate::routing::MessageRouter>,
        timeout: std::time::Duration,
    ) -> Self {
        Self {
            router,
            timeout,
            pending: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Builds a BifrostMessage of type GrpcRequest with payload (request_id, service, method, body base64). Returns (request_id, message).
    pub fn build_request(
        &self,
        source_device_id: &str,
        target_device_id: &str,
        service: &str,
        method: &str,
        body: &[u8],
    ) -> Result<(String, BifrostMessage), GrpcBridgeError> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "request_id": request_id,
            "service": service,
            "method": method,
            "body": BASE64.encode(body),
        });
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let msg = BifrostMessage {
            message_id: uuid::Uuid::new_v4().to_string(),
            message_type: MessageType::GrpcRequest,
            source_device_id: source_device_id.to_string(),
            target_device_id: target_device_id.to_string(),
            payload,
            timestamp,
            protocol_version: None,
        };
        Ok((request_id, msg))
    }

    /// Sends a gRPC request over Bifrost and waits for the response with the configured timeout.
    pub async fn send_request_and_wait(
        &self,
        source_device_id: &str,
        target_device_id: &str,
        service: &str,
        method: &str,
        body: &[u8],
    ) -> Result<Vec<u8>, GrpcBridgeError> {
        let (request_id, msg) = self.build_request(
            source_device_id,
            target_device_id,
            service,
            method,
            body,
        )?;
        let rx = self.register_pending(&request_id)?;
        self.router
            .route_message(msg)
            .await
            .map_err(|e| GrpcBridgeError::Parse(e.to_string()))?;
        let result = tokio::time::timeout(self.timeout, rx)
            .await
            .map_err(|_| GrpcBridgeError::Timeout)?;
        result?
    }

    /// Registers a pending request; returns a receiver that completes when on_grpc_response is called for this request_id.
    pub fn register_pending(
        &self,
        request_id: &str,
    ) -> Result<oneshot::Receiver<Result<Vec<u8>, GrpcBridgeError>>, GrpcBridgeError> {
        let (tx, rx) = oneshot::channel();
        self.pending
            .write()
            .unwrap()
            .insert(request_id.to_string(), tx);
        Ok(rx)
    }

    /// Completes the pending request with the given response. Call when a GrpcResponse message is received.
    pub fn on_grpc_response(&self, request_id: &str, body: &[u8], ok: bool) {
        if let Some(tx) = self.pending.write().unwrap().remove(request_id) {
            let result = if ok {
                Ok(body.to_vec())
            } else {
                Err(GrpcBridgeError::RemoteError(
                    String::from_utf8_lossy(body).to_string(),
                ))
            };
            let _ = tx.send(result);
        }
    }

    /// Parses a BifrostMessage of type GrpcResponse into request_id, body, ok.
    pub fn parse_grpc_response_payload(
        msg: &BifrostMessage,
    ) -> Result<GrpcResponsePayload, GrpcBridgeError> {
        if msg.message_type != MessageType::GrpcResponse {
            return Err(GrpcBridgeError::Parse("not a GrpcResponse message".to_string()));
        }
        let request_id = msg
            .payload
            .get("request_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GrpcBridgeError::Parse("missing request_id".to_string()))?
            .to_string();
        let body_b64 = msg
            .payload
            .get("body")
            .and_then(|v| v.as_str())
            .ok_or_else(|| GrpcBridgeError::Parse("missing body".to_string()))?;
        let body = BASE64
            .decode(body_b64)
            .map_err(|e| GrpcBridgeError::Parse(e.to_string()))?;
        let ok = msg.payload.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
        Ok(GrpcResponsePayload {
            request_id,
            body,
            ok,
        })
    }
}

/// Module-level helper for tests (parse_grpc_response_payload).
pub fn parse_grpc_response_payload(
    msg: &BifrostMessage,
) -> Result<GrpcResponsePayload, GrpcBridgeError> {
    GrpcBridge::parse_grpc_response_payload(msg)
}

// --- Phase 17.1.2: ThorAction Routing ---

/// Default Thor gRPC service and method for action execution.
pub const THOR_SERVICE: &str = "thor.Thor";
pub const THOR_METHOD: &str = "Execute";

#[derive(Error, Debug)]
pub enum ThorActionRouterError {
    #[error("action timeout")]
    Timeout,
    #[error("bridge error: {0}")]
    Bridge(#[from] GrpcBridgeError),
}

/// Routes ThorAction to remote device via gRPC over Bifrost; returns ThorResult with timeout handling.
pub struct ThorActionRouter {
    bridge: Arc<GrpcBridge>,
}

impl ThorActionRouter {
    pub fn new(bridge: Arc<GrpcBridge>) -> Self {
        Self { bridge }
    }

    /// Builds a ThorAction request message (service thor.Thor, method Execute). For tests or manual routing.
    pub fn build_action_request(
        &self,
        source_device_id: &str,
        target_device_id: &str,
        action_body: &[u8],
    ) -> Result<(String, BifrostMessage), GrpcBridgeError> {
        self.bridge.build_request(
            source_device_id,
            target_device_id,
            THOR_SERVICE,
            THOR_METHOD,
            action_body,
        )
    }

    /// Sends ThorAction to target device and waits for ThorResult with the bridge's timeout.
    pub async fn send_action(
        &self,
        source_device_id: &str,
        target_device_id: &str,
        action_body: &[u8],
    ) -> Result<Vec<u8>, ThorActionRouterError> {
        self.bridge
            .send_request_and_wait(
                source_device_id,
                target_device_id,
                THOR_SERVICE,
                THOR_METHOD,
                action_body,
            )
            .await
            .map_err(|e| match e {
                GrpcBridgeError::Timeout => ThorActionRouterError::Timeout,
                e => ThorActionRouterError::Bridge(e),
            })
    }
}
