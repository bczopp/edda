//! Managed flood routing (hop_limit, rebroadcast). Phase 11.0.1 stub.

use crate::mesh::packet::MeshPacket;

/// Forwards mesh packets based on hop_limit. Stub until full implementation.
pub struct FloodRouter;

impl FloodRouter {
    pub fn new() -> Self {
        Self
    }

    /// Returns true if the packet should be forwarded (to != self, to != broadcast, hop_limit > 0).
    pub fn should_forward(&self, packet: &MeshPacket, my_node_id: u32) -> bool {
        if packet.hop_limit == 0 {
            return false;
        }
        if packet.to == my_node_id {
            return false;
        }
        if packet.to == crate::mesh::packet::BROADCAST_NODE_ID {
            return true;
        }
        true
    }
}

impl Default for FloodRouter {
    fn default() -> Self {
        Self::new()
    }
}
