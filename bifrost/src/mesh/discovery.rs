//! Mesh discovery (NodeInfo, MyNodeInfo, NodeDB). Phase 11.0.1 stub.

use serde::{Deserialize, Serialize};

/// Node identity and status for discovery.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NodeInfo {
    pub num: u32,
    pub last_heard: Option<u64>,
    pub snr: Option<f32>,
    pub hops_away: Option<u32>,
}

/// Local node info (my_node_num, device_id, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MyNodeInfo {
    pub my_node_num: u32,
    pub reboot_count: u32,
    pub device_id: String,
}
