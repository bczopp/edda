//! # Bifrost – Communication Service
//!
//! WebSocket-basierte device-to-device Kommunikation für das Edda-System.
//! Enthält Connection-Management, Message-Routing, Heimdall-Integration (Validation),
//! gRPC-Tunnel (ThorAction), Security (Challenge/Token/Rate-Limit), Mesh-Layer, Discovery.
//!
//! ## Module
//!
//! - [`connection`] – Connection-Handler, -Manager, -Cache, Status-Tracker, Validation-Cache, Blocker
//! - [`discovery`] – IP-basierter Connection-Manager, Yggdrasil Discovery
//! - [`grpc_bridge`] – gRPC-Requests/Responses über Bifrost tunneln, ThorActionRouter
//! - [`guest`] – Gast-Mesh, Isolation, Permission, User-Confirmation
//! - [`heimdall`] – Connection-Validation-Client, Request/Response-Handler, User-Identity, Cross-User-Blocking
//! - [`message`] – BifrostMessage, MessageType, MessageHandler, Validator
//! - [`mesh`] – MeshPacket, FloodRouter, Discovery, Transport, Lifecycle, Membership
//! - [`nat`] – STUNClient (NAT Discovery), STUNClientStub
//! - [`protocol`] – Version-Negotiation
//! - [`queue`] – Message-Queuing
//! - [`routing`] – MessageRouter, Broadcast, Multicast, Relay, Retry, Quality-based
//! - [`security`] – Key-Generator/Storage, Challenge, Token, Rate-Limiter, Anomaly, Intrusion
//! - [`utils`] – Audit, Config, Logging, Metrics
//! - [`websocket`] – Server, Client, Acceptor, Initiator, Heartbeat, Reconnection

pub mod connection;
pub mod discovery;
pub mod grpc_bridge;
pub mod nat;
pub mod guest;
pub mod heimdall;
pub mod message;
pub mod mesh;
pub mod protocol;
pub mod queue;
pub mod routing;
pub mod security;
pub mod utils;
pub mod websocket;
