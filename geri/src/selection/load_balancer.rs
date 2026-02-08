//! Load-Balancer (Phase 7.3.1): gewichtete Auswahl nach Efficiency-Score, Request-Counting, 80%-Threshold.

use std::collections::HashMap;

use crate::model::ModelInfo;
use crate::selection::{EfficiencyInput, EfficiencyScoreCalculator};

/// Faktor für effektiven Score, wenn Provider über Load-Threshold liegt (weniger gewichtet).
const OVERLOAD_PENALTY: f64 = 0.2;

/// Gewichtete Provider-Auswahl basierend auf Efficiency-Score; Request-Counting und 80%-Threshold.
#[derive(Debug, Clone)]
pub struct LoadBalancer {
    request_counts: HashMap<String, u64>,
    load_threshold: f64,
    capacity_per_provider: u64,
}

impl LoadBalancer {
    /// Erstellt einen Load-Balancer mit Schwellwert (z. B. 0.8 = 80 %) und Kapazität pro Provider.
    pub fn new(load_threshold: f64, capacity_per_provider: u64) -> Self {
        Self {
            request_counts: HashMap::new(),
            load_threshold: load_threshold.clamp(0.0, 1.0),
            capacity_per_provider: capacity_per_provider.max(1),
        }
    }

    /// Erfasst einen Request für den angegebenen Provider.
    pub fn record_request(&mut self, provider_id: &str) {
        *self.request_counts.entry(provider_id.to_string()).or_insert(0) += 1;
    }

    /// Liefert die aktuelle Auslastung des Providers (0.0–1.0+), 0.0 wenn unbekannt.
    pub fn get_load(&self, provider_id: &str) -> f64 {
        let count = self.request_counts.get(provider_id).copied().unwrap_or(0);
        if self.capacity_per_provider == 0 {
            return 0.0;
        }
        count as f64 / self.capacity_per_provider as f64
    }

    /// Wählt aus Kandidaten das Model mit höchstem effektiven Efficiency-Score (überlastete Provider werden abgewertet).
    pub fn next(
        &self,
        candidates: &[(ModelInfo, EfficiencyInput)],
        calculator: &EfficiencyScoreCalculator,
    ) -> Option<ModelInfo> {
        if candidates.is_empty() {
            return None;
        }
        let with_effective: Vec<_> = candidates
            .iter()
            .map(|(info, input)| {
                let score = calculator.calculate(input);
                let load = self.get_load(&info.provider);
                let effective = if load >= self.load_threshold {
                    score * OVERLOAD_PENALTY
                } else {
                    score
                };
                (info, effective)
            })
            .collect();
        let best = with_effective
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))?;
        Some(best.0.clone())
    }
}
