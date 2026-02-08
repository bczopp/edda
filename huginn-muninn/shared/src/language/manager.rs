//! Language Manager for managing supported languages and preferences

use std::collections::HashMap;
use tracing::{debug, info};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Language Manager for managing supported languages and preferences
pub struct LanguageManager {
    supported_languages: Arc<RwLock<Vec<String>>>,
    user_preferences: Arc<RwLock<HashMap<String, String>>>, // user_id -> language
    default_language: String,
}

impl LanguageManager {
    /// Create a new Language Manager
    pub fn new() -> Self {
        info!("Creating LanguageManager");
        let mut supported = vec![
            "en-US".to_string(),
            "de-DE".to_string(),
            "fr-FR".to_string(),
            "es-ES".to_string(),
            "it-IT".to_string(),
            "pt-BR".to_string(),
            "ru-RU".to_string(),
            "zh-CN".to_string(),
            "ja-JP".to_string(),
            "ko-KR".to_string(),
        ];
        
        Self {
            supported_languages: Arc::new(RwLock::new(supported)),
            user_preferences: Arc::new(RwLock::new(HashMap::new())),
            default_language: "en-US".to_string(),
        }
    }
    
    /// Get supported languages
    pub async fn get_supported_languages(&self) -> Vec<String> {
        let languages = self.supported_languages.read().await;
        languages.clone()
    }
    
    /// Check if language is supported
    pub async fn is_supported(&self, language: &str) -> bool {
        let languages = self.supported_languages.read().await;
        languages.contains(&language.to_string())
    }
    
    /// Add supported language
    pub async fn add_language(&self, language: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut languages = self.supported_languages.write().await;
        if !languages.contains(&language) {
            languages.push(language.clone());
            info!("Added supported language: {}", language);
        }
        Ok(())
    }
    
    /// Remove supported language
    pub async fn remove_language(&self, language: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut languages = self.supported_languages.write().await;
        languages.retain(|l| l != language);
        info!("Removed supported language: {}", language);
        Ok(())
    }
    
    /// Get user language preference
    pub async fn get_user_language(&self, user_id: &str) -> String {
        let preferences = self.user_preferences.read().await;
        preferences.get(user_id)
            .cloned()
            .unwrap_or_else(|| self.default_language.clone())
    }
    
    /// Set user language preference
    pub async fn set_user_language(&self, user_id: &str, language: String) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_supported(&language).await {
            return Err(format!("Language {} is not supported", language).into());
        }
        
        let mut preferences = self.user_preferences.write().await;
        preferences.insert(user_id.to_string(), language.clone());
        debug!("Set language preference for user {}: {}", user_id, language);
        Ok(())
    }
    
    /// Get default language
    pub fn default_language(&self) -> &str {
        &self.default_language
    }
    
    /// Set default language
    pub fn set_default_language(&mut self, language: String) {
        self.default_language = language;
    }
}

impl Default for LanguageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_language_manager_new() {
        let manager = LanguageManager::new();
        let languages = manager.get_supported_languages().await;
        assert!(!languages.is_empty());
        assert!(languages.contains(&"en-US".to_string()));
    }
    
    #[tokio::test]
    async fn test_language_manager_is_supported() {
        let manager = LanguageManager::new();
        assert!(manager.is_supported("en-US").await);
        assert!(manager.is_supported("de-DE").await);
        assert!(!manager.is_supported("xx-XX").await);
    }
    
    #[tokio::test]
    async fn test_language_manager_user_preferences() {
        let manager = LanguageManager::new();
        
        // Get default language
        let lang = manager.get_user_language("user1").await;
        assert_eq!(lang, "en-US");
        
        // Set user language
        manager.set_user_language("user1", "de-DE".to_string()).await.unwrap();
        let lang = manager.get_user_language("user1").await;
        assert_eq!(lang, "de-DE");
    }
    
    #[tokio::test]
    async fn test_language_manager_add_remove() {
        let manager = LanguageManager::new();
        
        // Add new language
        manager.add_language("pl-PL".to_string()).await.unwrap();
        assert!(manager.is_supported("pl-PL").await);
        
        // Remove language
        manager.remove_language("pl-PL").await.unwrap();
        assert!(!manager.is_supported("pl-PL").await);
    }
}
