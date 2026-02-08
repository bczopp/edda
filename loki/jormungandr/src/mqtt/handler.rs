//! MQTT handler – connect, publish, subscribe (Phase 7.4.1).

use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug, Error)]
pub enum MqttError {
    #[error("MQTT error: {0}")]
    Mqtt(#[from] rumqttc::ClientError),
    #[error("Not connected")]
    NotConnected,
    #[error("Connection error: {0}")]
    Connect(String),
}

pub type Result<T> = std::result::Result<T, MqttError>;

/// MQTT handler: connect, publish, subscribe.
pub struct MQTTHandler {
    client: Option<AsyncClient>,
    _recv: Option<mpsc::UnboundedReceiver<Vec<u8>>>,
}

impl MQTTHandler {
    pub fn new() -> Self {
        Self {
            client: None,
            _recv: None,
        }
    }

    /// Connect to broker (host, port).
    pub async fn connect(&mut self, host: &str, port: u16) -> Result<()> {
        let mut options = MqttOptions::new("jormungandr-mqtt", host, port);
        options.set_keep_alive(std::time::Duration::from_secs(30));

        let (client, mut eventloop) = AsyncClient::new(options, 10);

        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Ok(event) = eventloop.poll().await {
                if let Event::Incoming(Packet::Publish(p)) = event {
                    let _ = tx.send(p.payload.to_vec());
                }
            }
        });

        self.client = Some(client);
        self._recv = Some(rx);
        Ok(())
    }

    /// Publish to topic.
    pub async fn publish(&self, topic: &str, payload: &[u8]) -> Result<()> {
        let client = self.client.as_ref().ok_or(MqttError::NotConnected)?;
        client.publish(topic, QoS::AtMostOnce, false, payload).await?;
        Ok(())
    }

    /// Subscribe to topic.
    pub async fn subscribe(&self, topic: &str) -> Result<()> {
        let client = self.client.as_ref().ok_or(MqttError::NotConnected)?;
        client.subscribe(topic, QoS::AtMostOnce).await?;
        Ok(())
    }
}

impl Default for MQTTHandler {
    fn default() -> Self {
        Self::new()
    }
}
