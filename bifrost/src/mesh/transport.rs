//! IP transport for MeshPackets (WebSocket). Phase 11.0.1.

use crate::mesh::packet::MeshPacket;
use serde_json;

/// Encode MeshPacket for WebSocket text frame.
pub fn encode_mesh_packet(packet: &MeshPacket) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    serde_json::to_string(packet).map_err(Into::into)
}

/// Decode MeshPacket from WebSocket text frame. Fails if JSON is not a valid MeshPacket.
pub fn decode_mesh_packet(s: &str) -> Result<MeshPacket, Box<dyn std::error::Error + Send + Sync>> {
    serde_json::from_str(s).map_err(Into::into)
}

/// Sends/receives MeshPackets over WebSocket (codec + optional send path).
pub struct MeshTransport;

impl MeshTransport {
    pub fn new() -> Self {
        Self
    }

    /// Encode and return bytes to send; caller sends over WebSocket.
    pub fn encode(&self, packet: &MeshPacket) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        encode_mesh_packet(packet)
    }

    /// Decode from WebSocket text frame.
    pub fn decode(&self, s: &str) -> Result<MeshPacket, Box<dyn std::error::Error + Send + Sync>> {
        decode_mesh_packet(s)
    }
}

impl Default for MeshTransport {
    fn default() -> Self {
        Self::new()
    }
}
