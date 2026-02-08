//! WebSocket handler – connect, send, receive, reconnect (Phase 7.3.1).

use futures_util::{SinkExt, StreamExt};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

#[derive(Debug, Error)]
pub enum WsError {
    #[error("WebSocket error: {0}")]
    Ws(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("Connect error: {0}")]
    Connect(String),
}

pub type Result<T> = std::result::Result<T, WsError>;

/// WebSocket handler: connect, send, receive, reconnect.
pub struct WebSocketHandler {
    url: String,
    stream: Option<WsStream>,
}

impl WebSocketHandler {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            stream: None,
        }
    }

    /// Establish WebSocket connection.
    pub async fn connect(&mut self) -> Result<()> {
        let (stream, _) = connect_async(&self.url).await.map_err(|e| WsError::Connect(e.to_string()))?;
        self.stream = Some(stream);
        Ok(())
    }

    /// Send text frame.
    pub async fn send(&mut self, text: &str) -> Result<()> {
        let s = self.stream.as_mut().ok_or_else(|| WsError::Connect("not connected".into()))?;
        s.send(tokio_tungstenite::tungstenite::Message::Text(text.into())).await?;
        Ok(())
    }

    /// Receive next text frame (None if closed or non-text).
    pub async fn receive(&mut self) -> Result<Option<String>> {
        let s = self.stream.as_mut().ok_or_else(|| WsError::Connect("not connected".into()))?;
        match s.next().await {
            Some(Ok(msg)) => match msg {
                tokio_tungstenite::tungstenite::Message::Text(t) => Ok(Some(t)),
                tokio_tungstenite::tungstenite::Message::Close(_) => {
                    self.stream = None;
                    Ok(None)
                }
                _ => Ok(None),
            },
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    /// Reconnect (close current if any, then connect).
    pub async fn reconnect(&mut self) -> Result<()> {
        self.stream = None;
        self.connect().await
    }
}
