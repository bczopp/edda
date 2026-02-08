//! Tests for HTTPRequestHandler (TDD – Phase 7.2.1).

use jormungandr::http::HTTPRequestHandler;

#[tokio::test]
async fn http_handler_get_returns_body() {
    let mut server = mockito::Server::new();
    let _m = server.mock("GET", "/").with_body("hello").create();

    let handler = HTTPRequestHandler::new().unwrap();
    let url = server.url();
    let body = handler.get(url).await.unwrap();
    assert_eq!(body, "hello");
}

#[tokio::test]
async fn http_handler_post_returns_body() {
    let mut server = mockito::Server::new();
    let _m = server.mock("POST", "/").with_body("posted").create();

    let handler = HTTPRequestHandler::new().unwrap();
    let url = server.url();
    let body = handler.post(url, "data").await.unwrap();
    assert_eq!(body, "posted");
}

#[tokio::test]
async fn http_handler_put_returns_body() {
    let mut server = mockito::Server::new();
    let _m = server.mock("PUT", "/").with_body("put").create();

    let handler = HTTPRequestHandler::new().unwrap();
    let url = server.url();
    let body = handler.put(url, "payload").await.unwrap();
    assert_eq!(body, "put");
}

#[tokio::test]
async fn http_handler_delete_ok() {
    let mut server = mockito::Server::new();
    let _m = server.mock("DELETE", "/").with_status(204).create();

    let handler = HTTPRequestHandler::new().unwrap();
    let url = server.url();
    handler.delete(url).await.unwrap();
}
