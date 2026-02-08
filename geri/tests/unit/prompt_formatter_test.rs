//! Tests für Prompt-Formatter (Phase 8.1.1).

#[cfg(test)]
mod tests {
    use geri::prompt::PromptFormatter;

    #[test]
    fn format_with_system_and_user_only() {
        let formatter = PromptFormatter::default();
        let full = formatter.format(
            "You are a helpful assistant.",
            "What is 2 + 2?",
            None,
        );
        assert!(full.contains("You are a helpful assistant."));
        assert!(full.contains("What is 2 + 2?"));
        assert!(full.find("You are").lt(&full.find("What is")));
    }

    #[test]
    fn format_with_system_context_and_user() {
        let formatter = PromptFormatter::default();
        let context = "[Document 1: doc-1]\nRAG context here.";
        let full = formatter.format(
            "You are a helpful assistant.",
            "Summarize the context.",
            Some(context),
        );
        assert!(full.contains("You are a helpful assistant."));
        assert!(full.contains("[Document 1: doc-1]"));
        assert!(full.contains("RAG context here."));
        assert!(full.contains("Summarize the context."));
        assert!(full.find("You are").lt(&full.find("doc-1")));
        assert!(full.find("doc-1").lt(&full.find("Summarize")));
    }

    #[test]
    fn format_with_empty_system_uses_default() {
        let formatter = PromptFormatter::default();
        let full = formatter.format("", "User question", None);
        assert!(!full.trim_start().is_empty());
        assert!(full.contains("User question"));
    }

    #[test]
    fn format_with_empty_context_omits_context_section() {
        let formatter = PromptFormatter::default();
        let full = formatter.format("System", "User", None);
        let full_with_empty = formatter.format("System", "User", Some(""));
        assert!(!full.contains("Context:"));
        assert_eq!(full, full_with_empty);
    }

    #[test]
    fn format_for_provider_llama_uses_instruction_format() {
        let formatter = PromptFormatter::default();
        let full = formatter.format_for_provider(
            "llama3-8b",
            "You are helpful.",
            "Tell me a joke.",
            None,
        );
        assert!(full.contains("You are helpful."));
        assert!(full.contains("Tell me a joke."));
        assert!(full.contains("[INST]") || full.contains("Instruction") || full.contains("You are helpful."));
    }

    #[test]
    fn format_for_provider_openai_uses_chat_sections() {
        let formatter = PromptFormatter::default();
        let full = formatter.format_for_provider(
            "gpt-4",
            "You are helpful.",
            "Hello",
            None,
        );
        assert!(full.contains("You are helpful."));
        assert!(full.contains("Hello"));
    }

    #[test]
    fn test_xml_protocol_injection() {
        use geri::prompt::inject_xml_protocol;
        let base_prompt = "You are a helpful assistant.";
        let injected = inject_xml_protocol(base_prompt);
        
        assert!(injected.contains(base_prompt));
        assert!(injected.contains("INTER-AGENT XML PROTOCOL"));
        assert!(injected.contains("<task>"));
        assert!(injected.contains("<call>"));
        
        // Test idempotency
        let double_injected = inject_xml_protocol(&injected);
        assert_eq!(injected, double_injected);
    }
}
