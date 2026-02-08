//! Vision-Provider-Trait (Phase 3.1.2): Abstraktion für Bild-/Video-Analyse.

use super::types::{VisionError, VisionRequest, VisionResponse};

/// Abstraktion für Vision-Provider (Bild-Analyse, optional Video/Stream).
pub trait VisionProvider: Send + Sync {
    /// Name des verwendeten Models (z. B. gpt-4v, claude-vision).
    fn model_name(&self) -> &str;

    /// Analysiert ein Bild (und optionalen Prompt); entspricht analyze_image.
    fn process(&self, request: VisionRequest) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<VisionResponse, VisionError>> + Send + '_>>;
}
