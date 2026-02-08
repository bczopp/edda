//! Tests for MQTTHandler (TDD – Phase 7.4.1).
//! No real broker: connect to invalid host fails, publish/subscribe before connect fail.

use jormungandr::mqtt::MQTTHandler;

#[test]
fn mqtt_handler_new() {
    let _ = MQTTHandler::new();
}

#[tokio::test]
async fn mqtt_handler_publish_before_connect_fails() {
    let handler = MQTTHandler::new();
    let res = handler.publish("topic", b"payload").await;
    assert!(res.is_err());
}

#[tokio::test]
async fn mqtt_handler_subscribe_before_connect_fails() {
    let handler = MQTTHandler::new();
    let res = handler.subscribe("topic").await;
    assert!(res.is_err());
}

// Integration test with real broker: connect to 127.0.0.1:1883 and publish/subscribe (optional)
