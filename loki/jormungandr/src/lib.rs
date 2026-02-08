//! Jörmungandr – Network/Communication Service (Loki sub-service).
//! HTTP, WebSocket, MQTT; script-accessible via Loki.

pub mod http;
pub mod mqtt;
pub mod script;
pub mod websocket;

pub use http::{HttpError, HTTPRequestHandler as HttpHandler, Result as HttpResult};
pub use mqtt::{MqttError, MQTTHandler as MqttHandler, Result as MqttResult};
pub use script::JormungandrScriptAPI;
pub use websocket::{WsError, WebSocketHandler as WsHandler, Result as WsResult};
