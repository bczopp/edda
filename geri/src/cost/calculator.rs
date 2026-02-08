//! Cost-Calculator (Phase 9.2.1): Cost pro Token pro Provider, Input + Output, Total-Cost.

/// Berechnet Gesamtkosten aus Input- und Output-Tokens pro Provider/Model.
#[derive(Debug, Clone, Default)]
pub struct CostCalculator;

impl CostCalculator {
    /// Berechnet die Gesamtkosten (in Dollar) für die angegebenen Input- und Output-Tokens.
    /// Unbekannte Provider/Model liefern 0.0.
    pub fn total_cost(
        &self,
        input_tokens: u32,
        output_tokens: u32,
        provider: &str,
        model: &str,
    ) -> f64 {
        let (in_per_1k, out_per_1k) = self.cost_per_1k_tokens(provider, model);
        let in_cost = (input_tokens as f64 / 1000.0) * in_per_1k;
        let out_cost = (output_tokens as f64 / 1000.0) * out_per_1k;
        in_cost + out_cost
    }

    /// Gibt (Input-$/1K Tokens, Output-$/1K Tokens) für Provider/Model zurück.
    fn cost_per_1k_tokens(&self, provider: &str, model: &str) -> (f64, f64) {
        let p = provider.to_lowercase();
        let m = model.to_lowercase();
        if p.contains("openai") {
            if m.contains("gpt-4") {
                return (0.03, 0.06);
            }
            if m.contains("gpt-3") {
                return (0.0015, 0.002);
            }
            return (0.03, 0.06);
        }
        if p.contains("anthropic") {
            if m.contains("claude") {
                return (0.025, 0.075);
            }
            return (0.025, 0.075);
        }
        (0.0, 0.0)
    }
}
