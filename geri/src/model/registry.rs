//! Model-Registry (Phase 6.1.2): Register/Unregister, Get/List/Filter.

use std::collections::HashMap;
use crate::model::{ModelInfo, ModelType};

/// In-Memory-Registry für Modelle (Register, Unregister, Get, List, Filter).
#[derive(Debug, Clone, Default)]
pub struct ModelRegistry {
    models: HashMap<String, ModelInfo>,
}

impl ModelRegistry {
    /// Erstellt ein leeres Registry (wie Default).
    pub fn new() -> Self {
        Self::default()
    }

    /// Registriert ein Model (überschreibt bei gleicher ID).
    pub fn register(self, model: ModelInfo) -> Self {
        let mut models = self.models;
        models.insert(model.id.clone(), model);
        Self { models }
    }

    /// Entfernt ein Model anhand der ID.
    pub fn unregister(self, id: &str) -> Self {
        let mut models = self.models;
        models.remove(id);
        Self { models }
    }

    /// Liefert das Model mit der angegebenen ID.
    pub fn get_by_id(&self, id: &str) -> Option<&ModelInfo> {
        self.models.get(id)
    }

    /// Liefert alle registrierten Modelle.
    pub fn list_all(&self) -> Vec<&ModelInfo> {
        self.models.values().collect()
    }

    /// Filtert nach Modell-Typ.
    pub fn filter_by_type(&self, model_type: ModelType) -> Vec<&ModelInfo> {
        self.models
            .values()
            .filter(|m| m.model_type == model_type)
            .collect()
    }

    /// Filtert nach Provider.
    pub fn filter_by_provider(&self, provider: &str) -> Vec<&ModelInfo> {
        let p = provider.to_lowercase();
        self.models
            .values()
            .filter(|m| m.provider.to_lowercase() == p)
            .collect()
    }
}
