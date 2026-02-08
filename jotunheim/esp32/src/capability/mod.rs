//! Capability management – device capabilities for Einherjar/Loki (Phase 4).

pub mod builder;
pub mod device_resolver;
pub mod device_tool_registry;
pub mod negotiator;
pub mod propagator;

pub use builder::DeviceCapabilityBuilder;
pub use device_resolver::{DeviceResolver, InMemoryDeviceResolver, ResolvedDevice};
pub use device_tool_registry::{generate_tools_from_capabilities, GeneratedParam, GeneratedToolDef};
pub use negotiator::CapabilityNegotiator;
pub use propagator::CapabilityPropagator;

/// Resolve device by ID and return generated tool definitions (with display_name as prefix).
/// Combines DeviceResolver + generate_tools_from_capabilities for use by Odin/platform.
pub fn tools_for_device<R: DeviceResolver>(
    resolver: &R,
    device_id: &str,
) -> Option<Vec<GeneratedToolDef>> {
    let r = resolver.resolve(device_id)?;
    Some(generate_tools_from_capabilities(
        &r.capabilities,
        Some(&r.display_name),
    ))
}

/// Placeholder: capability negotiation and exposure for Loki/controller.
pub struct CapabilityManager;

impl CapabilityManager {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CapabilityManager {
    fn default() -> Self {
        Self::new()
    }
}
