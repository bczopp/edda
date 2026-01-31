//! Connection Error Handler tests (Phase 10.2.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::connection::ConnectionErrorAction;
use bifrost::connection::ConnectionErrorCategory;
use bifrost::connection::ConnectionErrorHandler;
use std::io;

#[test]
fn io_timeout_categorizes_as_timeout_and_suggests_retry() {
    let handler = ConnectionErrorHandler;
    let err = io::Error::new(io::ErrorKind::TimedOut, "connection timed out");
    let category = handler.categorize(&err);
    assert_eq!(category, ConnectionErrorCategory::Timeout);
    let action = handler.suggest_action(category);
    assert_eq!(action, ConnectionErrorAction::Retry);
}

#[test]
fn io_connection_refused_categorizes_as_network_and_suggests_retry() {
    let handler = ConnectionErrorHandler;
    let err = io::Error::new(io::ErrorKind::ConnectionRefused, "refused");
    let category = handler.categorize(&err);
    assert_eq!(category, ConnectionErrorCategory::Network);
    let action = handler.suggest_action(category);
    assert_eq!(action, ConnectionErrorAction::Retry);
}

#[test]
fn io_not_found_categorizes_as_permanent_and_suggests_fallback() {
    let handler = ConnectionErrorHandler;
    let err = io::Error::new(io::ErrorKind::NotFound, "target not connected");
    let category = handler.categorize(&err);
    assert_eq!(category, ConnectionErrorCategory::Permanent);
    let action = handler.suggest_action(category);
    assert_eq!(action, ConnectionErrorAction::Fallback);
}

#[test]
fn auth_like_message_categorizes_as_auth_and_suggests_notify_user() {
    let handler = ConnectionErrorHandler;
    let err: Box<dyn std::error::Error + Send + Sync> =
        Box::new(io::Error::new(io::ErrorKind::PermissionDenied, "auth failed"));
    let category = handler.categorize(err.as_ref());
    assert_eq!(category, ConnectionErrorCategory::Auth);
    let action = handler.suggest_action(category);
    assert_eq!(action, ConnectionErrorAction::NotifyUser);
}

#[test]
fn critical_category_suggests_notify_user() {
    let handler = ConnectionErrorHandler;
    let action = handler.suggest_action(ConnectionErrorCategory::Critical);
    assert_eq!(action, ConnectionErrorAction::NotifyUser);
}

#[test]
fn transient_category_suggests_retry() {
    let handler = ConnectionErrorHandler;
    let action = handler.suggest_action(ConnectionErrorCategory::Transient);
    assert_eq!(action, ConnectionErrorAction::Retry);
}
