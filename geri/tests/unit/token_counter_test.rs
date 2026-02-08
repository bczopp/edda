//! Tests für Token-Counter (Phase 8.3.1).

#[cfg(test)]
mod tests {
    use geri::prompt::TokenCounter;

    #[test]
    fn count_empty_returns_zero() {
        let counter = TokenCounter::default();
        assert_eq!(counter.count(""), 0);
        assert_eq!(counter.count("   "), 0);
    }

    #[test]
    fn count_returns_positive_for_non_empty() {
        let counter = TokenCounter::default();
        assert!(counter.count("Hello world") > 0);
        assert!(counter.count("One token") > 0);
    }

    #[test]
    fn count_deterministic() {
        let counter = TokenCounter::default();
        let t = "The quick brown fox jumps over the lazy dog.";
        assert_eq!(counter.count(t), counter.count(t));
    }

    #[test]
    fn count_longer_text_more_tokens() {
        let counter = TokenCounter::default();
        let short = "Hi";
        let long = "This is a much longer piece of text with many more words.";
        assert!(counter.count(long) > counter.count(short));
    }

    #[test]
    fn count_for_model_llama() {
        let counter = TokenCounter::default();
        let n = counter.count_for_model("Hello world", "llama3-8b");
        assert!(n > 0);
    }

    #[test]
    fn count_for_model_gpt() {
        let counter = TokenCounter::default();
        let n = counter.count_for_model("Hello world", "gpt-4");
        assert!(n > 0);
    }

    #[test]
    fn count_for_model_empty_text_zero() {
        let counter = TokenCounter::default();
        assert_eq!(counter.count_for_model("", "gpt-4"), 0);
    }
}
