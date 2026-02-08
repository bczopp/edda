//! Tests für Provider-spezifisches Token-Counting (Phase 9.1.1).

#[cfg(test)]
mod tests {
    use geri::cost::{AnthropicTokenCounter, OpenAITokenCounter};
    use geri::prompt::TokenCounter;

    #[test]
    fn openai_count_returns_positive_for_non_empty() {
        let counter = OpenAITokenCounter::new(TokenCounter::default());
        let n = counter.count("Hello world", "gpt-4");
        assert!(n > 0);
    }

    #[test]
    fn openai_count_zero_for_empty() {
        let counter = OpenAITokenCounter::new(TokenCounter::default());
        assert_eq!(counter.count("", "gpt-4"), 0);
    }

    #[test]
    fn openai_count_deterministic() {
        let counter = OpenAITokenCounter::new(TokenCounter::default());
        let t = "The quick brown fox.";
        assert_eq!(counter.count(t, "gpt-4"), counter.count(t, "gpt-4"));
    }

    #[test]
    fn anthropic_count_returns_positive_for_non_empty() {
        let counter = AnthropicTokenCounter::new(TokenCounter::default());
        let n = counter.count("Hello world", "claude-3");
        assert!(n > 0);
    }

    #[test]
    fn anthropic_count_zero_for_empty() {
        let counter = AnthropicTokenCounter::new(TokenCounter::default());
        assert_eq!(counter.count("", "claude-3"), 0);
    }

    #[test]
    fn anthropic_count_deterministic() {
        let counter = AnthropicTokenCounter::new(TokenCounter::default());
        let t = "Sample text for Claude.";
        assert_eq!(counter.count(t, "claude-3"), counter.count(t, "claude-3"));
    }
}
