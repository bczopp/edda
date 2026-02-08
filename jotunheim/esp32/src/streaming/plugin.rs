//! Plugin system preparation (Phase 9.2.1, TDD).

/// Minimal plugin interface for future streaming/other plugins.
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
}

/// Registers and looks up plugins (preparation for streaming plugins).
pub struct PluginLoader {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn get(&self, name: &str) -> Option<&dyn Plugin> {
        self.plugins.iter().find(|p| p.name() == name).map(|b| b.as_ref())
    }

    pub fn list(&self) -> Vec<String> {
        self.plugins.iter().map(|p| p.name().to_string()).collect()
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}
