//! NAT Traversal (Phase 13): STUN/TURN/ICE abstractions.
//!
//! STUN and TURN clients are implemented as trait + stub for container/CI compatibility.
//! Real implementations can be plugged in via provider traits.

mod ice_manager;
mod port_forwarding;
mod stun_client;
mod turn_client;

pub use ice_manager::{
    IceCandidate, IceCandidateKind, ICEManager, ICEManagerProvider, ICEManagerStub, SelectedPath,
};
pub use port_forwarding::{
    PortForwardingConfigurator, PortForwardingConfiguratorProvider, PortForwardingConfiguratorStub,
};
pub use stun_client::{NatType, STUNClient, STUNClientProvider, STUNClientStub};
pub use turn_client::{RelayAllocation, TURNClient, TURNClientProvider, TURNClientStub};
