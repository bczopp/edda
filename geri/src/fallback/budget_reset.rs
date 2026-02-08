//! Budget-Reset-Listener (Phase 10.3.1): Yggdrasil Budget-Reset-Events, automatische Rückkehr zu Cloud-LLM.

/// Wird aufgerufen, wenn Yggdrasil ein Budget-Reset signalisiert (automatische Rückkehr zu Cloud-LLM).
pub trait BudgetResetHandler: Send + Sync {
    /// Wird von `BudgetResetListener::notify_reset()` aufgerufen; Implementierung kann z. B. Provider auf Cloud umschalten.
    fn on_budget_reset(&self);
}

/// Hört auf Budget-Reset-Events (z. B. von Yggdrasil) und löst die Rückkehr zu Cloud-LLM aus.
pub struct BudgetResetListener {
    handler: Box<dyn BudgetResetHandler>,
}

impl std::fmt::Debug for BudgetResetListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BudgetResetListener").finish_non_exhaustive()
    }
}

impl BudgetResetListener {
    /// Erstellt einen Listener mit dem angegebenen Handler (wird bei jedem Reset aufgerufen).
    pub fn new(handler: Box<dyn BudgetResetHandler>) -> Self {
        Self { handler }
    }

    /// Wird von der Yggdrasil-Integration aufgerufen, wenn ein Budget-Reset erfolgt ist; ruft den Handler auf.
    pub fn notify_reset(&self) {
        self.handler.on_budget_reset();
    }
}
