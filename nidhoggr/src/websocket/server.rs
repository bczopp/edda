use tokio_tungstenite::{accept_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsAcceptor, TlsStream};
use rustls::{ServerConfig, pki_types::{CertificateDer, PrivateKeyDer}};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tracing::{info, error, warn};
use std::sync::Arc;
use std::net::SocketAddr;
use std::io::BufReader;
use crate::connection::ConnectionManager;
use crate::routing::MessageRouter;
use crate::ratelimiter::RateLimiter;
use crate::security::{SecurityMonitor, SecurityEventType, SecuritySeverity};
use crate::security::audit::{AuditLogger, AuditEventType};
use ratatoskr::protocol::{MessageSerializer, MessageValidator, ConnectionProtocol};
use ratatoskr::messages::{RatatoskrRequest, RatatoskrResponse};
use ratatoskr::proto::ratatoskr::{MessageType, ConnectionRequestPayload};
use chrono::{Utc, Duration as ChronoDuration};
use uuid::Uuid;

pub struct WebSocketServer {
    port: u16,
    connection_manager: Arc<ConnectionManager>,
    router: Arc<MessageRouter>,
    rate_limiter: Arc<RateLimiter>,
    validator: MessageValidator,
    connection_protocol: ConnectionProtocol,
    tls_cert_path: Option<String>,
    tls_key_path: Option<String>,
    tls_acceptor: Option<Arc<TlsAcceptor>>,
    security_monitor: Arc<SecurityMonitor>,
    audit_logger: Arc<AuditLogger>,
}

impl WebSocketServer {

    pub fn new_with_tls(
        port: u16,
        connection_manager: Arc<ConnectionManager>,
        router: Arc<MessageRouter>,
        rate_limiter: Arc<RateLimiter>,
        tls_cert_path: Option<String>,
        tls_key_path: Option<String>,
        security_monitor: Arc<SecurityMonitor>,
        audit_logger: Arc<AuditLogger>,
    ) -> Self {
        let tls_acceptor = if let (Some(cert_path), Some(key_path)) = (&tls_cert_path, &tls_key_path) {
            Self::create_tls_acceptor(cert_path, key_path)
        } else {
            None
        };

        Self {
            port,
            connection_manager,
            router,
            rate_limiter,
            validator: MessageValidator::new(),
            connection_protocol: ConnectionProtocol::new(),
            tls_cert_path,
            tls_key_path,
            tls_acceptor,
            security_monitor,
            audit_logger,
        }
    }

    pub fn new_with_deps(
        port: u16,
        connection_manager: Arc<ConnectionManager>,
        router: Arc<MessageRouter>,
        rate_limiter: Arc<RateLimiter>,
        security_monitor: Arc<SecurityMonitor>,
        audit_logger: Arc<AuditLogger>,
    ) -> Self {
        Self::new_with_tls(port, connection_manager, router, rate_limiter, None, None, security_monitor, audit_logger)
    }

    fn create_tls_acceptor(cert_path: &str, key_path: &str) -> Option<Arc<TlsAcceptor>> {
        // Load certificate
        let cert_file = std::fs::File::open(cert_path).ok()?;
        let mut cert_reader = BufReader::new(cert_file);
        let certs: Vec<CertificateDer> = certs(&mut cert_reader)
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        // Load private key
        let key_file = std::fs::File::open(key_path).ok()?;
        let mut key_reader = BufReader::new(key_file);
        let mut keys = pkcs8_private_keys(&mut key_reader)
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        if keys.is_empty() {
            return None;
        }

        let key = PrivateKeyDer::Pkcs8(keys.remove(0));

        // Configure TLS 1.3 only
        let mut config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .ok()?;

        // Force TLS 1.3 only
        config.versions = vec![rustls::ProtocolVersion::TLSv1_3];

        Some(Arc::new(TlsAcceptor::from(Arc::new(config))))
    }

    pub async fn start(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;
        let local_addr = listener.local_addr()?;
        info!("Nidhöggr WebSocket server listening on {}", local_addr);

        // Start cleanup task
        let connection_manager = self.connection_manager.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                connection_manager.cleanup_expired().await;
            }
        });

        // Spawn server loop in background
        let server = Arc::new(self.clone());
        tokio::spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        let server_clone = server.clone();
                        tokio::spawn(async move {
                            if let Err(e) = server_clone.handle_connection(stream, addr).await {
                                error!("Connection error from {}: {}", addr, e);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);
                    }
                }
            }
        });
        
        Ok(local_addr)
    }

    async fn handle_connection(
        &self,
        stream: TcpStream,
        addr: SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Apply TLS if configured, then accept WebSocket connection
        let ws_stream = if let Some(acceptor) = &self.tls_acceptor {
            let tls_stream: TlsStream<TcpStream> = acceptor.accept(stream).await?;
            // Convert TlsStream to a type that tokio-tungstenite can accept
            // tokio-tungstenite with rustls-tls-webpki-roots feature can accept TlsStream directly
            accept_async(tls_stream).await?
        } else {
            accept_async(stream).await?
        };
        info!("New WebSocket connection from {} (TLS: {})", addr, self.tls_acceptor.is_some());
        
        self.audit_logger.log(
            AuditEventType::ConnectionEstablished,
            None,
            None,
            None,
            None,
            format!("Connection from {}", addr),
            true,
        ).await;

        let (mut write, mut read) = ws_stream.split();
        let mut session_id: Option<String> = None;

        while let Some(msg) = read.next().await {
            match msg {
                Ok(Message::Binary(data)) => {
                    match self.handle_ratatoskr_message(&data, &mut session_id).await {
                        Ok(Some(response)) => {
                            let serializer = MessageSerializer::new();
                            let serialized = serializer.serialize_response(&response)?;
                            write.send(Message::Binary(serialized)).await?;
                        }
                        Ok(None) => {
                            // No response needed (e.g., heartbeat)
                        }
                        Err(e) => {
                            error!("Error handling message: {}", e);
                            let error_response = RatatoskrResponse::new_error(
                                MessageType::Error,
                                Uuid::new_v4().to_string(),
                                "ERROR".to_string(),
                                e.to_string(),
                            );
                            let serializer = MessageSerializer::new();
                            let serialized = serializer.serialize_response(&error_response)?;
                            write.send(Message::Binary(serialized)).await?;
                        }
                    }
                }
                Ok(Message::Text(_)) => {
                    warn!("Received text message, expected binary Ratatoskr message");
                    let error_response = RatatoskrResponse::new_error(
                        MessageType::Error,
                        Uuid::new_v4().to_string(),
                        "INVALID_FORMAT".to_string(),
                        "Expected binary Ratatoskr message".to_string(),
                    );
                    let serializer = MessageSerializer::new();
                    let serialized = serializer.serialize_response(&error_response)?;
                    write.send(Message::Binary(serialized)).await?;
                }
                Ok(Message::Close(_)) => {
                    info!("Connection closed from {}", addr);
                    if let Some(sid) = &session_id {
                        self.connection_manager.remove_connection_by_session(sid).await;
                        self.audit_logger.log(
                            AuditEventType::ConnectionClosed,
                            None,
                            None,
                            Some(sid.clone()),
                            None,
                            format!("Connection closed from {}", addr),
                            true,
                        ).await;
                    }
                    break;
                }
                Ok(Message::Ping(data)) => {
                    write.send(Message::Pong(data)).await?;
                }
                Ok(Message::Pong(_)) => {}
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }

    async fn handle_ratatoskr_message(
        &self,
        data: &[u8],
        session_id: &mut Option<String>,
    ) -> Result<Option<RatatoskrResponse>, Box<dyn std::error::Error>> {
        // Deserialize request
        let serializer = MessageSerializer::new();
        let request = serializer.deserialize_request(data)?;

        // Validate request
        if let Err(e) = self.validator.validate_request(&request) {
            self.security_monitor.record_event(
                SecurityEventType::InvalidSignature,
                request.device_id.clone(),
                request.user_id.clone(),
                format!("Validation failed: {}", e),
                SecuritySeverity::Medium,
            ).await;
            self.audit_logger.log(
                AuditEventType::Error,
                Some(request.device_id.clone()),
                Some(request.user_id.clone()),
                None,
                Some(format!("{:?}", request.message_type)),
                format!("Validation failed: {}", e),
                false,
            ).await;
            return Err(e.into());
        }

        // Check rate limit
        if let Err(e) = self.rate_limiter.check_rate_limit(&request.device_id, &request.user_id).await {
            self.security_monitor.record_event(
                SecurityEventType::RateLimitExceeded,
                request.device_id.clone(),
                request.user_id.clone(),
                e.to_string(),
                SecuritySeverity::High,
            ).await;
            self.audit_logger.log(
                AuditEventType::RateLimitHit,
                Some(request.device_id.clone()),
                Some(request.user_id.clone()),
                None,
                None,
                e.to_string(),
                false,
            ).await;
            return Err(anyhow::anyhow!("Rate limit error: {}", e));
        }

        // Log message received
        self.audit_logger.log(
            AuditEventType::MessageReceived,
            Some(request.device_id.clone()),
            Some(request.user_id.clone()),
            session_id.clone(),
            Some(format!("{:?}", request.message_type)),
            "Message received".to_string(),
            true,
        ).await;

        // Handle based on message type
        match request.message_type {
            x if x == MessageType::ConnectionRequest as i32 => {
                self.handle_connection_request(request, session_id).await
            }
            x if x == MessageType::BusinessRequest as i32 => {
                self.handle_business_request(request).await
            }
            x if x == MessageType::Heartbeat as i32 => {
                // Heartbeat - update last heartbeat time
                if let Some(ref sid) = session_id {
                    self.connection_manager.update_heartbeat(sid).await;
                }
                Ok(None)
            }
            x if x == MessageType::Disconnect as i32 => {
                if let Some(sid) = session_id.take() {
                    self.connection_manager.remove_connection_by_session(&sid).await;
                }
                Ok(None)
            }
            _ => Err(anyhow::anyhow!("Unknown message type: {}", request.message_type)),
        }
    }

    async fn handle_connection_request(
        &self,
        request: RatatoskrRequest,
        session_id: &mut Option<String>,
    ) -> Result<Option<RatatoskrResponse>, Box<dyn std::error::Error>> {
        // Parse connection request payload
        let payload = self.connection_protocol.parse_connection_request_payload(&request)?;

        // In a real implementation, we would:
        // 1. Validate device_identity with Heimdall
        // 2. Validate authentication_token with Heimdall
        // 3. Create session

        // For now, accept all connection requests
        let expires_at = Utc::now() + ChronoDuration::hours(24);
        let (connection_id, sid) = self.connection_manager.register_connection(
            &request.device_id,
            &request.user_id,
            expires_at,
        ).await;

        *session_id = Some(sid.clone());

        let response = self.connection_protocol.create_connection_response(
            &request,
            true,
            sid,
            expires_at.timestamp(),
            "1.0".to_string(),
        )?;

        Ok(Some(response))
    }

    async fn handle_business_request(
        &self,
        request: RatatoskrRequest,
    ) -> Result<Option<RatatoskrResponse>, Box<dyn std::error::Error>> {
        // Route message to appropriate service
        let service_response = self.router.route_message(request.clone()).await
            .map_err(|e| anyhow::anyhow!("Routing error: {}", e))?;

        // Create RatatoskrResponse with service response
        let response_payload = service_response.unwrap_or_else(|| {
            serde_json::json!({"status": "success"}).to_string().into_bytes()
        });
        
        let response = RatatoskrResponse::new_success(
            MessageType::BusinessRequest,
            request.request_id,
            response_payload,
        );

        Ok(Some(response))
    }
}

impl Clone for WebSocketServer {
    fn clone(&self) -> Self {
        Self {
            port: self.port,
            connection_manager: self.connection_manager.clone(),
            router: self.router.clone(),
            rate_limiter: self.rate_limiter.clone(),
            validator: MessageValidator::new(),
            connection_protocol: ConnectionProtocol::new(),
            tls_cert_path: self.tls_cert_path.clone(),
            tls_key_path: self.tls_key_path.clone(),
            tls_acceptor: self.tls_acceptor.clone(),
            security_monitor: self.security_monitor.clone(),
            audit_logger: self.audit_logger.clone(),
        }
    }
}
