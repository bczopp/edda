//! Mesh layer (Meshtastic-inspired): MeshPacket, FloodRouter, Discovery, Transport.
//! See docs/MESH_LAYER_DESIGN.md and IMPLEMENTATION_PLAN Phase 11.

pub mod discovery;
pub mod enforcer;
pub mod flood_router;
pub mod lifecycle;
pub mod membership;
pub mod packet;
pub mod status_monitor;
pub mod transport;

pub use discovery::{MyNodeInfo, NodeInfo};
pub use enforcer::{MeshConnectionEnforcer, MeshEnforcerError};
pub use flood_router::FloodRouter;
pub use lifecycle::{LifecycleAction, MeshConnectionLifecycleManager};
pub use membership::{MeshMembershipChecker, MeshMembershipProvider, MeshMembershipStub};
pub use packet::{Data, MeshPacket, BROADCAST_NODE_ID};
pub use status_monitor::{MeshStatusMonitor, MeshStatusSnapshot};
pub use transport::MeshTransport;
