//! CapabilityNegotiator (Phase 4.2.1, TDD).

use crate::grpc::proto::jotunheim_capability::{CapabilityResponse, JotunheimCapabilities};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::time::timeout;

#[derive(Debug, Error)]
pub enum NegotiationError {
    #[error("Negotiation timeout")]
    Timeout,
}

/// Sends device capabilities when a CAPABILITY_REQUEST is received.
pub struct CapabilityNegotiator {
    capabilities: JotunheimCapabilities,
    responder: Arc<dyn Fn(CapabilityResponse) + Send + Sync>,
    negotiation_timeout: Option<Duration>,
}

impl CapabilityNegotiator {
    pub fn new<F>(capabilities: JotunheimCapabilities, responder: F) -> Self
    where
        F: Fn(CapabilityResponse) + Send + Sync + 'static,
    {
        Self {
            capabilities,
            responder: Arc::new(responder),
            negotiation_timeout: None,
        }
    }

    pub fn with_timeout<F>(
        capabilities: JotunheimCapabilities,
        responder: F,
        d: Duration,
    ) -> Self
    where
        F: Fn(CapabilityResponse) + Send + Sync + 'static,
    {
        Self {
            capabilities,
            responder: Arc::new(responder),
            negotiation_timeout: Some(d),
        }
    }

    pub async fn on_capability_request(&self) {
        let resp = CapabilityResponse {
            capabilities: Some(self.capabilities.clone()),
        };
        (self.responder)(resp);
    }

    pub async fn negotiate_with_timeout(&self) -> Result<(), NegotiationError> {
        match self.negotiation_timeout {
            Some(d) => {
                timeout(d, self.on_capability_request())
                    .await
                    .map_err(|_| NegotiationError::Timeout)?;
            }
            None => {
                self.on_capability_request().await;
            }
        }
        Ok(())
    }
}
