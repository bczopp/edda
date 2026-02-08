//! Model-Health-Checker (Phase 6.2.1): periodische Health-Checks, Availability-Status, Uptime-Percentage.

use crate::model::ModelRegistry;
use std::collections::{HashMap, VecDeque};

/// Maximale Anzahl gespeicherter Check-Ergebnisse pro Model für Uptime-Berechnung.
const MAX_CHECK_HISTORY: usize = 100;

/// Führt einen Health-Check für ein Model durch (z. B. Ping, API-Call). In Tests mockbar.
pub trait ModelHealthProbe: Send + Sync {
    /// Gibt `true` zurück, wenn das Model verfügbar ist.
    fn check(&self, model_id: &str) -> bool;
}

/// Gespeicherter Status und Verlauf pro Model.
#[derive(Debug, Clone, Default)]
pub struct ModelHealthStatus {
    /// Letzte N Check-Ergebnisse (true = ok, false = fail) für Uptime-Berechnung.
    results: VecDeque<bool>,
}

impl ModelHealthStatus {
    /// Aktuell verfügbar (None = noch nie gecheckt).
    pub fn available(&self) -> Option<bool> {
        self.results.back().copied()
    }

    /// Uptime in Prozent (0–100), None wenn keine Checks.
    pub fn uptime_percentage(&self) -> Option<f64> {
        if self.results.is_empty() {
            return None;
        }
        let ok = self.results.iter().filter(|&&b| b).count();
        Some(ok as f64 / self.results.len() as f64 * 100.0)
    }

    fn record(&mut self, available: bool) {
        if self.results.len() >= MAX_CHECK_HISTORY {
            self.results.pop_front();
        }
        self.results.push_back(available);
    }
}

/// Führt periodische Health-Checks für alle registrierten Modelle durch und trackt Availability/Uptime.
pub struct ModelHealthChecker {
    registry: ModelRegistry,
    probe: Box<dyn ModelHealthProbe>,
    statuses: HashMap<String, ModelHealthStatus>,
}

impl ModelHealthChecker {
    /// Erstellt einen Checker mit Registry und Probe.
    pub fn new(registry: ModelRegistry, probe: Box<dyn ModelHealthProbe>) -> Self {
        Self {
            registry,
            probe,
            statuses: HashMap::new(),
        }
    }

    /// Führt einen Health-Check für alle registrierten Modelle durch.
    pub fn run_check_all(&mut self) {
        for model in self.registry.list_all() {
            let id = model.id.clone();
            let available = self.probe.check(&id);
            let status = self.statuses.entry(id).or_default();
            status.record(available);
        }
    }

    /// Gibt den aktuellen Availability-Status für das Model zurück (None = unbekannt/nicht gecheckt).
    pub fn get_availability(&self, model_id: &str) -> Option<bool> {
        self.statuses.get(model_id).and_then(|s| s.available())
    }

    /// Gibt das Uptime-Percentage für das Model zurück (None = keine Checks).
    pub fn get_uptime_percentage(&self, model_id: &str) -> Option<f64> {
        self.statuses.get(model_id).and_then(|s| s.uptime_percentage())
    }
}
