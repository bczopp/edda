//! Fallback-Manager (Phase 10.1.1): Cloud-Limit-Erkennung, automatischer Fallback zu lokalem LLM.

use crate::model::ModelInfo;
use crate::selection::{EfficiencyInput, ModelSelector, SelectionOptions};

use async_trait::async_trait;

/// Erkennt, ob ein Cloud-Limit erreicht ist (z. B. Budget, Rate-Limit). In Tests mockbar.
#[async_trait]
pub trait CloudLimitDetector: Send + Sync {
    /// Gibt true zurück, wenn auf lokales LLM gewechselt werden soll.
    async fn is_cloud_limit(&self) -> bool;
}

/// Steuert den Fallback von Cloud-LLM zu lokalem LLM bei Limit-Erkennung.
#[derive(Debug, Clone)]
pub struct FallbackManager {
    selector: ModelSelector,
}

impl FallbackManager {
    /// Erstellt einen Fallback-Manager mit dem angegebenen Model-Selector (für beste lokale Auswahl).
    pub fn new(selector: ModelSelector) -> Self {
        Self { selector }
    }

    /// Liefert das beste lokale Model als Fallback, wenn ein Cloud-Limit erkannt wurde; sonst None.
    /// `local_candidates`: nur lokale Models mit zugehörigem EfficiencyInput (Multi-Faktor-Bewertung).
    pub async fn get_fallback_model(
        &self,
        local_candidates: &[(ModelInfo, EfficiencyInput)],
        detector: &dyn CloudLimitDetector,
    ) -> Option<ModelInfo> {
        if !detector.is_cloud_limit().await {
            return None;
        }
        self.selector
            .select(local_candidates, &SelectionOptions::default())
    }
}

pub struct BudgetCloudLimitDetector {
    budget_tracker: Arc<crate::cost::BudgetTracker>,
}

impl BudgetCloudLimitDetector {
    pub fn new(budget_tracker: Arc<crate::cost::BudgetTracker>) -> Self {
        Self { budget_tracker }
    }
}

#[async_trait]
impl CloudLimitDetector for BudgetCloudLimitDetector {
    async fn is_cloud_limit(&self) -> bool {
        self.budget_tracker.is_over_limit().await
    }
}

// Re-defining the trait to be async if needed, but let's check first.
