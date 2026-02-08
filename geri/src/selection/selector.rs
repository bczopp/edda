//! Model-Selection-Engine (Phase 7.2.1): automatische Auswahl nach Efficiency-Score, User-Override, Constraints.

use crate::model::ModelInfo;
use crate::selection::{EfficiencyInput, EfficiencyScoreCalculator};

/// Optionen für die Model-Auswahl (User-Präferenz, Latency-/Cost-Constraints).
#[derive(Debug, Clone, Default)]
pub struct SelectionOptions {
    /// Wenn gesetzt und in Kandidaten: dieses Model wird gewählt (übersteuert automatische Auswahl).
    pub user_preferred_model_id: Option<String>,
    /// Max. akzeptable Latency in ms; Kandidaten mit höherem ping_ms werden ausgeschlossen.
    pub max_latency_ms: Option<u32>,
    /// Max. akzeptable Kosten pro Token; Kandidaten mit höherem cost_per_token werden ausgeschlossen.
    pub max_cost_per_token: Option<f64>,
}

/// Wählt aus Kandidaten (Model + EfficiencyInput) ein Model: User-Override, dann Constraints, dann bester Score.
#[derive(Debug, Clone)]
pub struct ModelSelector {
    calculator: EfficiencyScoreCalculator,
}

impl ModelSelector {
    /// Erstellt einen Selector mit dem angegebenen Efficiency-Score-Calculator.
    pub fn new(calculator: EfficiencyScoreCalculator) -> Self {
        Self { calculator }
    }

    /// Wählt ein Model: zuerst User-Präferenz (falls gesetzt und vorhanden), sonst Filter nach Constraints, dann höchster Efficiency-Score.
    pub fn select(
        &self,
        candidates: &[(ModelInfo, EfficiencyInput)],
        options: &SelectionOptions,
    ) -> Option<ModelInfo> {
        if candidates.is_empty() {
            return None;
        }
        if let Some(ref preferred) = options.user_preferred_model_id {
            if let Some((info, _)) = candidates.iter().find(|(m, _)| m.id == *preferred) {
                return Some(info.clone());
            }
            return None;
        }
        let filtered: Vec<_> = candidates
            .iter()
            .filter(|(_, input)| self.satisfies_constraints(input, options))
            .collect();
        if filtered.is_empty() {
            return None;
        }
        let best = filtered
            .iter()
            .max_by(|(_, a), (_, b)| {
                self.calculator
                    .calculate(a)
                    .partial_cmp(&self.calculator.calculate(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;
        Some(best.0.clone())
    }

    fn satisfies_constraints(&self, input: &EfficiencyInput, options: &SelectionOptions) -> bool {
        if let Some(max_lat) = options.max_latency_ms {
            if let Some(ping) = input.ping_ms {
                if ping > max_lat {
                    return false;
                }
            }
        }
        if let Some(max_cost) = options.max_cost_per_token {
            if let Some(cost) = input.cost_per_token {
                if cost > max_cost {
                    return false;
                }
            }
        }
        true
    }
}
