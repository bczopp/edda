//! Key-Rotation-Manager (Phase 14.2.1): Alte Keys entfernen, neue hinzufügen, Rotation-Workflow.

use crate::keys::{KeyStorageError, SecureKeyStorage};

/// Führt den Rotation-Workflow für API-Keys durch (alt entfernen, neu speichern).
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyRotationManager;

impl KeyRotationManager {
    /// Rotiert den API-Key für den Provider: entfernt den alten Key (falls vorhanden), speichert den neuen.
    pub fn rotate(
        self,
        storage: &mut SecureKeyStorage,
        provider_id: &str,
        new_api_key: &str,
    ) -> Result<(), KeyStorageError> {
        let _ = storage.remove_key(provider_id);
        storage.store_key(provider_id, new_api_key)
    }
}
