// Test suite for OdinPlugin trait implementation
//
// TDD Cycle: Red → Green → Refactor
// This file contains all tests that must pass before implementation is complete

#![cfg_attr(not(feature = "test"), allow(dead_code))]

use frigg::plugin::{FriggPlugin, OdinPlugin};

/// ============================================================================
/// TESTS FOR NAME METHOD
/// ============================================================================

#[test]
fn test_plugin_has_expected_name() {
    let plugin = FriggPlugin::new();
    assert_eq!(plugin.name(), "frigg");
}

#[test]
fn test_plugin_name_is_static_str_not_owned_string() {
    let plugin: &FriggPlugin = &FriggPlugin::new();
    let name = plugin.name();
    assert!(name.is_empty() == false);
    // Name should be usable across lifetimes without panicking
    let _captured: &str = name;
}

/// ============================================================================
/// TESTS FOR CAPABILITIES METHOD
/// ============================================================================

#[test]
fn test_plugin_returns_expected_capabilities() {
    let plugin = FriggPlugin::new();
    let capabilities = plugin.capabilities();

    assert_eq!(capabilities.len(), 4, "Expected 4 initial capabilities");
}

#[test]
fn test_plugin_capability_contains_health_questions() {
    let plugin = FriggPlugin::new();
    let capabilities = plugin.capabilities();

    assert!(
        capabilities.contains(&"health_questions".to_string()),
        "Should include 'health_questions' capability"
    );
}

#[test]
fn test_plugin_capability_contains_mental_health() {
    let plugin = FriggPlugin::new();
    let capabilities = plugin.capabilities();

    assert!(
        capabilities.contains(&"mental_health".to_string()),
        "Should include 'mental_health' capability"
    );
}

#[test]
fn test_plugin_capability_contains_physical_health() {
    let plugin = FriggPlugin::new();
    let capabilities = plugin.capabilities();

    assert!(
        capabilities.contains(&"physical_health".to_string()),
        "Should include 'physical_health' capability"
    );
}

#[test]
fn test_plugin_capability_contains_certified_courses() {
    let plugin = FriggPlugin::new();
    let capabilities = plugin.capabilities();

    assert!(
        capabilities.contains(&"certified_courses".to_string()),
        "Should include 'certified_courses' capability"
    );
}

#[test]
fn test_plugin_capabilities_are_unique() {
    let plugin = FriggPlugin::new();
    let capabilities = plugin.capabilities();

    let unique: Vec<&str> = std::collections::HashSet::from_iter(
        capabilities.iter().map(|c| c.as_str())
    );

    assert_eq!(capabilities.len(), unique.len(), "No duplicate capabilities");
}

#[test]
fn test_plugin_can_clone_capabilities() {
    let plugin1 = FriggPlugin::new();
    let capabilities1 = plugin1.capabilities();

    let plugin2 = FriggPlugin::new();
    let capabilities2 = plugin2.capabilities();

    assert_eq!(capabilities1, capabilities2, " Cloning should produce same capabilities");
}

/// ============================================================================
/// TESTS FOR PROCESS_REQUEST METHOD
/// ============================================================================

mod process_request_tests {

    use super::*;

    /// Test 1: Mental health request parsing and handling
    #[test]
    fn test_process_request_handles_mental_health_query() {
        let plugin = FriggPlugin::new();
        let result = plugin.process_request("I'm feeling anxious").await;

        assert!(result.is_ok(), "Should handle mental health queries successfully");
        assert!(
            result.unwrap().contains("mental"),
            "Response should acknowledge mental health focus"
        );
    }

    /// Test 2: Physical health request handling
    #[test]
    fn test_process_request_handles_physical_health_query() {
        let plugin = FriggPlugin::new();
        let result = plugin.process_request("I have some back pain").await;

        assert!(result.is_ok(), "Should handle physical health queries successfully");
        assert!(
            result.unwrap().contains("physical"),
            "Response should acknowledge physical health focus"
        );
    }

    /// Test 3: Course-related request handling
    #[test]
    fn test_process_request_handles_course_query() {
        let plugin = FriggPlugin::new();
        let result = plugin.process_request("Show me certified courses").await;

        assert!(result.is_ok(), "Should handle course queries successfully");
        assert!(
            result.unwrap().contains("certified"),
            "Response should acknowledge certification focus"
        );
    }

    /// Test 4: Unknown healthcare request handling
    #[test]
    fn test_process_request_handles_generic_healthcare_request() {
        let plugin = FriggPlugin::new();
        let result = plugin.process_request("I need help with stress management").await;

        assert!(result.is_ok(), "Should handle generic requests successfully");
        let response = result.unwrap();

        // Should acknowledge the healthcare context
        assert!(!response.is_empty(), "Response should not be empty");

        #[cfg(feature = "test")]
        {
            assert!(
                response.contains("frigg"),
                "Response should identify it's from Frigg"
            );
        }
    }

    /// Test 5: Empty request handling
    #[test]
    fn test_process_request_handles_empty_request_gracefully() {
        let plugin = FriggPlugin::new();
        let result = plugin.process_request("").await;

        assert!(result.is_ok(), "Should handle empty requests gracefully");
    }
}

/// ============================================================================
/// TESTS FOR DEFAULT IMPLEMENTATION
/// ============================================================================

#[test]
fn test_frigg_plugin_can_be_created_with_default() {
    let _plugin: FriggPlugin = Default::default();
}

#[test]
fn test_default_is_same_as_new() {
    let plugin1: FriggPlugin = Default::default();
    let plugin2 = FriggPlugin::new();

    assert_eq!(plugin1.name(), plugin2.name());
    assert_eq!(plugin1.capabilities(), plugin2.capabilities());
}