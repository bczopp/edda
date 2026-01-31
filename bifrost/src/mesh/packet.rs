//! MeshPacket and Data types (Phase 11.0.1). See docs/MESH_LAYER_DESIGN.md.

use serde::{Deserialize, Serialize};

/// Broadcast node ID (0xFFFFFFFF).
pub const BROADCAST_NODE_ID: u32 = 0xFFFF_FFFF;

/// Mesh packet envelope (Meshtastic-inspired). Carries payload and hop metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeshPacket {
    pub from: u32,
    pub to: u32,
    pub hop_limit: u32,
    pub hop_start: u32,
    pub want_ack: bool,
    pub id: u32,
    pub channel: u32,
    pub payload: Vec<u8>,
}

/// Data sub-packet (port, payload, dest/source, request/reply).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data {
    pub portnum: u32,
    pub payload: Vec<u8>,
    pub dest: Option<u32>,
    pub source: Option<u32>,
    pub request_id: Option<u32>,
    pub reply_id: Option<u32>,
    pub want_response: bool,
}
