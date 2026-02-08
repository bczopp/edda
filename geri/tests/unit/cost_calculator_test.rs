//! Tests für Cost-Calculator (Phase 9.2.1).

#[cfg(test)]
mod tests {
    use geri::cost::CostCalculator;

    #[test]
    fn calculate_total_cost_openai() {
        let calc = CostCalculator::default();
        let cost = calc.total_cost(100, 50, "openai", "gpt-4");
        assert!(cost >= 0.0);
        assert!(cost < 1.0);
    }

    #[test]
    fn calculate_total_cost_anthropic() {
        let calc = CostCalculator::default();
        let cost = calc.total_cost(100, 50, "anthropic", "claude-3");
        assert!(cost >= 0.0);
    }

    #[test]
    fn calculate_total_cost_zero_tokens_zero_cost() {
        let calc = CostCalculator::default();
        assert_eq!(calc.total_cost(0, 0, "openai", "gpt-4"), 0.0);
    }

    #[test]
    fn cost_is_deterministic() {
        let calc = CostCalculator::default();
        let a = calc.total_cost(1000, 200, "openai", "gpt-4");
        let b = calc.total_cost(1000, 200, "openai", "gpt-4");
        assert_eq!(a, b);
    }

    #[test]
    fn unknown_provider_returns_zero_or_default() {
        let calc = CostCalculator::default();
        let cost = calc.total_cost(100, 50, "unknown", "unknown");
        assert!(cost >= 0.0);
    }
}
