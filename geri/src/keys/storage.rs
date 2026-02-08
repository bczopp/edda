//! Secure-Key-Storage (Phase 14.1.1): API-Keys speichern/laden über Backend.

use std::fmt;
use crate::keys::{KeyStorageError, SecureKeyBackend};

/// Speichert und lädt API-Keys über ein konfigurierbares Backend (OS-Secure-Storage oder In-Memory).
pub struct SecureKeyStorage {
    backend: Box<dyn SecureKeyBackend>,
}

impl fmt::Debug for SecureKeyStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecureKeyStorage").finish_non_exhaustive()
    }
}

impl SecureKeyStorage {
    /// Erstellt eine Storage mit dem angegebenen Backend (z. B. OS-Keychain oder In-Memory für Tests).
    pub fn new(backend: Box<dyn SecureKeyBackend>) -> Self {
        Self { backend }
    }

    /// Speichert einen API-Key für den angegebenen Provider (z. B. "openai", "anthropic").
    pub fn store_key(&mut self, provider_id: &str, api_key: &str) -> Result<(), KeyStorageError> {
        self.backend.store(provider_id, api_key.as_bytes())
    }

    /// Lädt den API-Key für den Provider; None wenn nicht gesetzt.
    pub fn load_key(&self, provider_id: &str) -> Result<Option<String>, KeyStorageError> {
        let bytes = self.backend.load(provider_id)?;
        match bytes {
            None => Ok(None),
            Some(b) => {
                String::from_utf8(b).map(Some).map_err(|_| KeyStorageError::InvalidUtf8)
            }
        }
    }

    /// Entfernt den API-Key für den Provider (z. B. für Rotation).
    pub fn remove_key(&mut self, provider_id: &str) -> Result<(), KeyStorageError> {
        self.backend.delete(provider_id)
    }
}
