//! Multicast Manager (Phase 9.3.2). Send to device group; manage groups.

use crate::connection::ConnectionManager;
use crate::message::BifrostMessage;
use crate::routing::MessageRouter;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("group not found: {0}")]
pub struct GroupNotFoundError(pub String);

/// Multicasts message to a device group; manages group membership.
pub struct MulticastManager {
    #[allow(dead_code)] // reserved for future per-connection routing
    connection_manager: Arc<ConnectionManager>,
    router: MessageRouter,
    groups: RwLock<HashMap<String, HashSet<String>>>,
}

impl MulticastManager {
    pub fn new(
        connection_manager: Arc<ConnectionManager>,
        router: MessageRouter,
    ) -> Self {
        Self {
            connection_manager,
            router,
            groups: RwLock::new(HashMap::new()),
        }
    }

    pub fn create_group(&self, group_id: &str) {
        let mut g = self.groups.write().unwrap();
        g.entry(group_id.to_string()).or_insert_with(HashSet::new);
    }

    pub fn add_member(&self, group_id: &str, device_id: &str) {
        let mut g = self.groups.write().unwrap();
        g.entry(group_id.to_string())
            .or_insert_with(HashSet::new)
            .insert(device_id.to_string());
    }

    pub fn remove_member(&self, group_id: &str, device_id: &str) {
        let mut g = self.groups.write().unwrap();
        if let Some(members) = g.get_mut(group_id) {
            members.remove(device_id);
        }
    }

    pub fn list_members(&self, group_id: &str) -> Vec<String> {
        let g = self.groups.read().unwrap();
        g.get(group_id)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Sends message to all devices in the group (target_device_id set per member).
    pub async fn send_to_group(
        &self,
        group_id: &str,
        message: BifrostMessage,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let members: Vec<String> = {
            let g = self.groups.read().unwrap();
            g.get(group_id)
                .ok_or_else(|| GroupNotFoundError(group_id.to_string()))?
                .iter()
                .cloned()
                .collect()
        };
        for device_id in members {
            let mut msg = message.clone();
            msg.target_device_id = device_id.clone();
            let _ = self.router.route_message(msg).await;
        }
        Ok(())
    }
}
