//! Security tests for Script-Execution (Phase 14.3.1): input validation, path escape.

use loki::coordination::ServiceCoordinator;

#[tokio::test(flavor = "multi_thread")]
async fn security_invalid_lua_returns_error() {
    let coord = ServiceCoordinator::new().unwrap();
    let result = coord.execute_script("invalid lua syntax {{{").await;
    assert!(result.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn security_hel_path_escape_returns_error() {
    let coord = ServiceCoordinator::new().unwrap();
    let script = "return hel:fs_read('..')";
    let result = coord.execute_script(script).await;
    assert!(result.is_err());
}
