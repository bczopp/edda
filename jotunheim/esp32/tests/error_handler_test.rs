// ErrorHandler tests (Phase 8.2.1, TDD).

use jotunheim_esp32::resilience::{ErrorHandler, ErrorKind};
use std::error::Error;

#[test]
fn categorize_connection_refused_as_network() {
    let h = ErrorHandler::new();
    let err: Box<dyn Error> = Box::new(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "refused",
    ));
    let kind = h.categorize(err.as_ref());
    assert!(matches!(kind, ErrorKind::Network(_)));
}

#[test]
fn is_recoverable_network_error() {
    let h = ErrorHandler::new();
    let err: Box<dyn Error> = Box::new(std::io::Error::new(
        std::io::ErrorKind::ConnectionRefused,
        "refused",
    ));
    assert!(h.is_recoverable(err.as_ref()));
}

#[test]
fn handle_records_error() {
    let h = ErrorHandler::new();
    let err: Box<dyn Error> = Box::new(std::io::Error::new(std::io::ErrorKind::Other, "test"));
    h.handle(err.as_ref());
    assert!(h.last_error().is_some());
}
