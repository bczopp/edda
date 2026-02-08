//! Tests for JormungandrScriptAPI – Lua bindings (TDD – Phase 7.5.1).

use jormungandr::{
    HttpHandler, JormungandrScriptAPI, MqttHandler, WsHandler,
};
use mlua::Lua;
use std::sync::Arc;
use tokio::sync::Mutex;

fn api() -> JormungandrScriptAPI {
    let http = Arc::new(HttpHandler::new().unwrap());
    let ws = Arc::new(Mutex::new(None::<WsHandler>));
    let mqtt = Arc::new(Mutex::new(MqttHandler::new()));
    JormungandrScriptAPI::new(http, ws, mqtt)
}

#[tokio::test(flavor = "multi_thread")]
async fn jormungandr_script_api_register_and_http_get() {
    let mut server = mockito::Server::new();
    let _m = server.mock("GET", "/").with_body("hello from mock").create();

    let api = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();

    let url = server.url();
    let body: String = lua
        .load(format!("return jormungandr:http_get(\"{}\")", url))
        .eval()
        .unwrap();
    assert_eq!(body, "hello from mock");
}

#[tokio::test(flavor = "multi_thread")]
async fn jormungandr_script_api_http_post() {
    let mut server = mockito::Server::new();
    let _m = server
        .mock("POST", "/")
        .with_body("posted")
        .match_body("data")
        .create();

    let api = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();

    let url = server.url();
    let body: String = lua
        .load(format!(
            "return jormungandr:http_post(\"{}\", \"data\")",
            url
        ))
        .eval()
        .unwrap();
    assert_eq!(body, "posted");
}

#[tokio::test(flavor = "multi_thread")]
async fn jormungandr_script_api_http_put_and_delete() {
    let mut server = mockito::Server::new();
    let _m_put = server.mock("PUT", "/").with_body("put").create();
    let _m_del = server.mock("DELETE", "/").with_status(204).create();

    let api = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();

    let url = server.url();
    let put_body: String = lua
        .load(format!(
            "return jormungandr:http_put(\"{}\", \"payload\")",
            url
        ))
        .eval()
        .unwrap();
    assert_eq!(put_body, "put");

    lua.load(format!("jormungandr:http_delete(\"{}\")", url))
        .exec()
        .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn jormungandr_script_api_ws_not_connected_returns_error() {
    let api = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();

    let err = lua.load("return jormungandr:ws_send(\"hi\")").eval::<String>();
    assert!(err.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn jormungandr_script_api_mqtt_not_connected_returns_error() {
    let api = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();

    let err = lua
        .load("return jormungandr:mqtt_publish(\"topic\", \"payload\")")
        .eval::<String>();
    assert!(err.is_err());
}
