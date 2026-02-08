//! CapabilityPropagator (Phase 4.3.1, TDD).
//! Propagates capabilities on connect and sends CapabilityUpdateEvent.

use crate::grpc::proto::jotunheim_capability::{
    CapabilityUpdateEvent, JotunheimCapabilities,
};

/// Propagates capabilities when device couples/connects and sends update events.
pub struct CapabilityPropagator {
    capabilities: JotunheimCapabilities,
    on_event: Option<Box<dyn Fn(CapabilityUpdateEvent) + Send + Sync>>,
}

impl CapabilityPropagator {
    pub fn new(capabilities: JotunheimCapabilities) -> Self {
        Self {
            capabilities,
            on_event: None,
        }
    }

    pub fn with_event_callback<F>(mut self, f: F) -> Self
    where
        F: Fn(CapabilityUpdateEvent) + Send + Sync + 'static,
    {
        self.on_event = Some(Box::new(f));
        self
    }

    /// Propagate capabilities on connection (e.g. send "connected" event).
    pub async fn on_connect(&self) {
        self.emit_event("connected");
    }

    /// Emit a capability update event.
    pub fn emit_event(&self, event_type: &str) {
        let ev = CapabilityUpdateEvent {
            event_type: event_type.to_string(),
            capabilities: Some(self.capabilities.clone()),
        };
        if let Some(ref f) = self.on_event {
            f(ev);
        }
    }

    /// Update capabilities and optionally emit "updated" event.
    pub fn update_capabilities(&mut self, capabilities: JotunheimCapabilities) {
        self.capabilities = capabilities;
        self.emit_event("updated");
    }
}
