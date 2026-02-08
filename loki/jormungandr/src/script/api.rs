//! Jörmungandr script API – Lua bindings for HTTP, WebSocket, MQTT (Phase 7.5.1).

use std::sync::Arc;

use mlua::{UserData, UserDataMethods};

use crate::http::HTTPRequestHandler;
use crate::mqtt::MQTTHandler;
use crate::websocket::WebSocketHandler;

/// Script API exposed to Lua: http_*, ws_*, mqtt_*.
pub struct JormungandrScriptAPI {
    http: Arc<HTTPRequestHandler>,
    ws: Arc<tokio::sync::Mutex<Option<WebSocketHandler>>>,
    mqtt: Arc<tokio::sync::Mutex<MQTTHandler>>,
}

impl JormungandrScriptAPI {
    pub fn new(
        http: Arc<HTTPRequestHandler>,
        ws: Arc<tokio::sync::Mutex<Option<WebSocketHandler>>>,
        mqtt: Arc<tokio::sync::Mutex<MQTTHandler>>,
    ) -> Self {
        Self { http, ws, mqtt }
    }

    /// Register this API as global "jormungandr" in the given Lua state.
    pub fn register_into(&self, lua: &mlua::Lua) -> mlua::Result<()> {
        lua.globals()
            .set("jormungandr", self.clone_for_lua())
    }

    fn clone_for_lua(&self) -> JormungandrScriptAPI {
        JormungandrScriptAPI {
            http: Arc::clone(&self.http),
            ws: Arc::clone(&self.ws),
            mqtt: Arc::clone(&self.mqtt),
        }
    }

    fn block_on<F, T>(future: F) -> T
    where
        F: std::future::Future<Output = T>,
    {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(future)
        })
    }
}

impl Clone for JormungandrScriptAPI {
    fn clone(&self) -> Self {
        Self {
            http: Arc::clone(&self.http),
            ws: Arc::clone(&self.ws),
            mqtt: Arc::clone(&self.mqtt),
        }
    }
}

impl UserData for JormungandrScriptAPI {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("http_get", |_, this, url: String| {
            let body = Self::block_on(this.http.get(&url))
                .map_err(|e| mlua::Error::external(e))?;
            Ok(body)
        });
        methods.add_method("http_post", |_, this, (url, body): (String, String)| {
            let resp = Self::block_on(this.http.post(&url, &body))
                .map_err(|e| mlua::Error::external(e))?;
            Ok(resp)
        });
        methods.add_method("http_put", |_, this, (url, body): (String, String)| {
            let resp = Self::block_on(this.http.put(&url, &body))
                .map_err(|e| mlua::Error::external(e))?;
            Ok(resp)
        });
        methods.add_method("http_delete", |_, this, url: String| {
            Self::block_on(this.http.delete(&url))
                .map_err(|e| mlua::Error::external(e))?;
            Ok(())
        });

        methods.add_method("ws_connect", |_, this, url: String| {
            let mut guard = Self::block_on(this.ws.lock());
            let mut handler = WebSocketHandler::new(url);
            Self::block_on(handler.connect())
                .map_err(|e| mlua::Error::external(e))?;
            *guard = Some(handler);
            Ok(())
        });
        methods.add_method("ws_send", |_, this, text: String| {
            let mut guard = Self::block_on(this.ws.lock());
            let handler = guard.as_mut().ok_or_else(|| {
                mlua::Error::external(crate::websocket::WsError::Connect(
                    "not connected".into(),
                ))
            })?;
            Self::block_on(handler.send(&text)).map_err(|e| mlua::Error::external(e))?;
            Ok(())
        });
        methods.add_method("ws_receive", |_, this, ()| {
            let mut guard = Self::block_on(this.ws.lock());
            let handler = guard.as_mut().ok_or_else(|| {
                mlua::Error::external(crate::websocket::WsError::Connect(
                    "not connected".into(),
                ))
            })?;
            let opt = Self::block_on(handler.receive())
                .map_err(|e| mlua::Error::external(e))?;
            if opt.is_none() {
                *guard = None;
            }
            Ok(opt)
        });
        methods.add_method("ws_reconnect", |_, this, ()| {
            let mut guard = Self::block_on(this.ws.lock());
            let handler = guard.as_mut().ok_or_else(|| {
                mlua::Error::external(crate::websocket::WsError::Connect(
                    "not connected".into(),
                ))
            })?;
            Self::block_on(handler.reconnect()).map_err(|e| mlua::Error::external(e))?;
            Ok(())
        });

        methods.add_method("mqtt_connect", |_, this, (host, port): (String, u16)| {
            let mut guard = Self::block_on(this.mqtt.lock());
            Self::block_on(guard.connect(&host, port))
                .map_err(|e| mlua::Error::external(e))?;
            Ok(())
        });
        methods.add_method("mqtt_publish", |_, this, (topic, payload): (String, String)| {
            let guard = Self::block_on(this.mqtt.lock());
            Self::block_on(guard.publish(&topic, payload.as_bytes()))
                .map_err(|e| mlua::Error::external(e))?;
            Ok(())
        });
        methods.add_method("mqtt_subscribe", |_, this, topic: String| {
            let guard = Self::block_on(this.mqtt.lock());
            Self::block_on(guard.subscribe(&topic))
                .map_err(|e| mlua::Error::external(e))?;
            Ok(())
        });
    }
}
