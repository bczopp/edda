//! Guest Mesh Cleanup Manager (Phase 12.3.1). Auto-cleanup on connection close, timeout-based cleanup, resource release.

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use super::GuestMeshManager;

struct MeshState {
    connection_count: u32,
    last_seen: Option<Instant>,
}

/// Automatically cleans up guest meshes on connection close and after idle timeout; releases resources via GuestMeshManager.
pub struct GuestMeshCleanupManager {
    guest_manager: Arc<GuestMeshManager>,
    idle_timeout: Duration,
    state: RwLock<HashMap<String, MeshState>>,
}

impl GuestMeshCleanupManager {
    pub fn new(guest_manager: Arc<GuestMeshManager>, idle_timeout: Duration) -> Self {
        Self {
            guest_manager,
            idle_timeout,
            state: RwLock::new(HashMap::new()),
        }
    }

    /// Registers a connection for the given guest mesh (increments count).
    pub fn register_connection(&self, mesh_id: &str) {
        if mesh_id == super::MAIN_MESH_ID {
            return;
        }
        let mut map = self.state.write().unwrap();
        let entry = map
            .entry(mesh_id.to_string())
            .or_insert_with(|| MeshState {
                connection_count: 0,
                last_seen: None,
            });
        entry.connection_count = entry.connection_count.saturating_add(1);
    }

    /// Called when a connection to a guest mesh is closed (decrements count; when 0, records last_seen).
    pub fn on_connection_closed(&self, mesh_id: &str) {
        if mesh_id == super::MAIN_MESH_ID {
            return;
        }
        let mut map = self.state.write().unwrap();
        let entry = match map.get_mut(mesh_id) {
            Some(e) => e,
            None => return,
        };
        entry.connection_count = entry.connection_count.saturating_sub(1);
        if entry.connection_count == 0 {
            entry.last_seen = Some(Instant::now());
        }
    }

    /// Removes guest meshes that have 0 connections and have been idle longer than idle_timeout (resource release).
    pub fn cleanup_idle(&self) {
        let now = Instant::now();
        let mut map = self.state.write().unwrap();
        let to_remove: Vec<String> = map
            .iter()
            .filter(|(_, s)| {
                s.connection_count == 0
                    && s.last_seen
                        .map(|t| now.duration_since(t) >= self.idle_timeout)
                        .unwrap_or(false)
            })
            .map(|(id, _)| id.clone())
            .collect();
        for id in &to_remove {
            self.guest_manager.remove_guest_mesh(id);
            map.remove(id);
        }
    }
}
