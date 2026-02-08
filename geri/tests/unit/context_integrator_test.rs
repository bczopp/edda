//! Tests für Context-Integrator (Phase 8.2.2).

#[cfg(test)]
mod tests {
    use geri::prompt::{ContextDocument, ContextFormatter, ContextIntegrator, PromptFormatter};

    fn doc(id: &str, content: &str, score: f32) -> ContextDocument {
        ContextDocument {
            id: id.to_string(),
            content: content.to_string(),
            score,
            metadata: None,
        }
    }

    #[test]
    fn integrate_without_context_omits_context_section() {
        let integrator = ContextIntegrator::new(
            PromptFormatter::default(),
            ContextFormatter::default(),
        );
        let full = integrator.integrate("System.", "User question.", &[]);
        assert!(full.contains("System."));
        assert!(full.contains("User question."));
        assert!(!full.contains("Context:"));
    }

    #[test]
    fn integrate_with_documents_inserts_context_between_system_and_user() {
        let integrator = ContextIntegrator::new(
            PromptFormatter::default(),
            ContextFormatter::default(),
        );
        let docs = vec![doc("d1", "RAG content.", 0.8)];
        let full = integrator.integrate("System.", "User.", &docs);
        assert!(full.contains("System."));
        assert!(full.contains("RAG content."));
        assert!(full.contains("User."));
        let pos_system = full.find("System.").unwrap();
        let pos_context = full.find("RAG content.").unwrap();
        let pos_user = full.find("User.").unwrap();
        assert!(pos_system < pos_context);
        assert!(pos_context < pos_user);
    }

    #[test]
    fn integrate_uses_prompt_template_order() {
        let integrator = ContextIntegrator::new(
            PromptFormatter::default(),
            ContextFormatter::default(),
        );
        let docs = vec![doc("a", "A.", 0.9)];
        let full = integrator.integrate("You are helpful.", "What is 2+2?", &docs);
        assert!(full.starts_with("You are helpful.") || full.contains("You are helpful."));
        assert!(full.contains("A."));
        assert!(full.contains("What is 2+2?"));
    }

    #[test]
    fn integrate_with_empty_documents_same_as_no_context() {
        let integrator = ContextIntegrator::new(
            PromptFormatter::default(),
            ContextFormatter::default(),
        );
        let with_empty = integrator.integrate("S", "U", &[]);
        assert!(with_empty.contains("S"));
        assert!(with_empty.contains("U"));
        assert!(!with_empty.contains("Context:"));
    }

    #[test]
    fn integrate_with_max_chars_truncates_context_section() {
        let integrator = ContextIntegrator::new(
            PromptFormatter::default(),
            ContextFormatter::default(),
        );
        let docs = vec![
            doc("a", "Short.", 0.9),
            doc("b", "Also short.", 0.8),
        ];
        let full = integrator.integrate_with_max_chars("S", "U", &docs, 80);
        assert!(full.contains("S"));
        assert!(full.contains("U"));
        assert!(full.len() <= 80 + 20);
    }
}
