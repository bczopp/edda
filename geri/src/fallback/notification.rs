//! Fallback-Notification-Generator (Phase 10.2.1): Benachrichtigungs-Text, An Odin senden, User-Einstellungen.

/// Grund für die Fallback-Benachrichtigung (für TTS via Muninn).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FallbackNotificationReason {
    /// Cloud-Limit erreicht (Budget/Rate-Limit).
    CloudLimitReached,
    /// Cloud-Provider nicht verfügbar.
    CloudProviderUnavailable,
    /// Netzwerk-LLM (z. B. Desktop) wird verwendet.
    NetworkLlmUsed,
    /// Lokales LLM wird verwendet.
    LocalLlmUsed,
}

/// Sendet Benachrichtigungstext an Odin (für TTS via Muninn). In Tests mockbar.
pub trait NotificationSender: Send + Sync {
    /// Sendet den Text an Odin; Fehler werden als String zurückgegeben.
    fn send(&self, text: &str) -> Result<(), String>;
}

/// Generiert Fallback-Benachrichtigungstexte und sendet sie optional an Odin.
#[derive(Debug, Clone, Copy, Default)]
pub struct FallbackNotificationGenerator;

impl FallbackNotificationGenerator {
    /// Erzeugt den Benachrichtigungstext für den angegebenen Grund (siehe README).
    pub fn generate_text(&self, reason: FallbackNotificationReason) -> String {
        match reason {
            FallbackNotificationReason::CloudLimitReached => {
                "Ich nutze jetzt lokales Modell, da Cloud-Limit erreicht".to_string()
            }
            FallbackNotificationReason::CloudProviderUnavailable => {
                "Ich nutze jetzt lokales Modell, da Cloud-Provider nicht verfügbar".to_string()
            }
            FallbackNotificationReason::NetworkLlmUsed => {
                "Ich nutze jetzt Desktop-LLM für bessere Qualität".to_string()
            }
            FallbackNotificationReason::LocalLlmUsed => {
                "Ich nutze jetzt lokales Modell".to_string()
            }
        }
    }

    /// Generiert Text und sendet ihn an Odin, wenn Benachrichtigungen aktiviert sind.
    /// Gibt den generierten Text zurück bei Erfolg, sonst None (z. B. wenn deaktiviert).
    pub fn generate_and_send(
        &self,
        reason: FallbackNotificationReason,
        sender: &impl NotificationSender,
        notifications_enabled: bool,
    ) -> Option<String> {
        if !notifications_enabled {
            return None;
        }
        let text = self.generate_text(reason);
        sender.send(&text).ok()?;
        Some(text)
    }
}
