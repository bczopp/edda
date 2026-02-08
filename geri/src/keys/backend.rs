//! Backend für Secure-Key-Storage (OS-spezifisch oder In-Memory für Tests).

use std::collections::HashMap;

use crate::keys::KeyStorageError;

/// Backend für sichere Speicherung von API-Keys (OS-Keychain oder In-Memory).
pub trait SecureKeyBackend: Send + Sync {
    /// Speichert Wert unter dem angegebenen Key (Provider-ID).
    fn store(&mut self, key_id: &str, value: &[u8]) -> Result<(), KeyStorageError>;
    /// Lädt Wert für den Key; None wenn nicht vorhanden.
    fn load(&self, key_id: &str) -> Result<Option<Vec<u8>>, KeyStorageError>;
    /// Entfernt den Key (z. B. für Rotation).
    fn delete(&mut self, key_id: &str) -> Result<(), KeyStorageError>;
}

/// In-Memory-Backend für Tests (keine echte Verschlüsselung).
#[derive(Debug, Clone, Default)]
pub struct InMemoryKeyBackend {
    store: HashMap<String, Vec<u8>>,
}

impl InMemoryKeyBackend {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SecureKeyBackend for InMemoryKeyBackend {
    fn store(&mut self, key_id: &str, value: &[u8]) -> Result<(), KeyStorageError> {
        self.store.insert(key_id.to_string(), value.to_vec());
        Ok(())
    }

    fn load(&self, key_id: &str) -> Result<Option<Vec<u8>>, KeyStorageError> {
        Ok(self.store.get(key_id).cloned())
    }

    fn delete(&mut self, key_id: &str) -> Result<(), KeyStorageError> {
        self.store.remove(key_id);
        Ok(())
    }
}
