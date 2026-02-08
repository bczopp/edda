//! Tests für Fallback-Notification-Generator (Phase 10.2.1).

#[cfg(test)]
mod tests {
    use geri::fallback::{
        FallbackNotificationGenerator, FallbackNotificationReason, NotificationSender,
    };
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn generate_text_cloud_limit_reached() {
        let gen = FallbackNotificationGenerator;
        let text = gen.generate_text(FallbackNotificationReason::CloudLimitReached);
        assert!(text.contains("lokales Modell"));
        assert!(text.contains("Cloud-Limit"));
    }

    #[test]
    fn generate_text_provider_unavailable() {
        let gen = FallbackNotificationGenerator;
        let text = gen.generate_text(FallbackNotificationReason::CloudProviderUnavailable);
        assert!(text.contains("lokales Modell"));
        assert!(text.contains("Cloud-Provider"));
    }

    #[test]
    fn generate_text_network_llm_used() {
        let gen = FallbackNotificationGenerator;
        let text = gen.generate_text(FallbackNotificationReason::NetworkLlmUsed);
        assert!(text.contains("Desktop-LLM") || text.contains("Netzwerk"));
    }

    #[test]
    fn generate_text_local_llm_used() {
        let gen = FallbackNotificationGenerator;
        let text = gen.generate_text(FallbackNotificationReason::LocalLlmUsed);
        assert!(text.contains("lokales Modell"));
    }

    struct MockSender(Arc<AtomicBool>);
    impl NotificationSender for MockSender {
        fn send(&self, _text: &str) -> Result<(), String> {
            self.0.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    #[test]
    fn generate_and_send_returns_none_when_notifications_disabled() {
        let gen = FallbackNotificationGenerator;
        let sent = Arc::new(AtomicBool::new(false));
        let sender = MockSender(Arc::clone(&sent));
        let result = gen.generate_and_send(
            FallbackNotificationReason::CloudLimitReached,
            &sender,
            false,
        );
        assert!(result.is_none());
        assert!(!sent.load(Ordering::SeqCst));
    }

    #[test]
    fn generate_and_send_returns_text_and_calls_sender_when_enabled() {
        let gen = FallbackNotificationGenerator;
        let sent = Arc::new(AtomicBool::new(false));
        let sender = MockSender(Arc::clone(&sent));
        let result = gen.generate_and_send(
            FallbackNotificationReason::CloudLimitReached,
            &sender,
            true,
        );
        assert!(result.is_some());
        assert!(result.as_ref().unwrap().contains("Cloud-Limit"));
        assert!(sent.load(Ordering::SeqCst));
    }
}
