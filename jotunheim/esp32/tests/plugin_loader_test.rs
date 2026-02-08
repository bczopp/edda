// PluginLoader tests (Phase 9.2.1, TDD).

use jotunheim_esp32::streaming::{Plugin, PluginLoader};

struct TestPlugin {
    name: String,
}

impl Plugin for TestPlugin {
    fn name(&self) -> &str {
        &self.name
    }
}

#[test]
fn register_and_get_plugin() {
    let mut loader = PluginLoader::new();
    loader.register(Box::new(TestPlugin {
        name: "video".to_string(),
    }));
    let p = loader.get("video").unwrap();
    assert_eq!(p.name(), "video");
}

#[test]
fn get_returns_none_for_unknown() {
    let loader = PluginLoader::new();
    assert!(loader.get("unknown").is_none());
}

#[test]
fn list_returns_registered_plugin_names() {
    let mut loader = PluginLoader::new();
    loader.register(Box::new(TestPlugin {
        name: "audio".to_string(),
    }));
    let names = loader.list();
    assert_eq!(names, ["audio"]);
}
