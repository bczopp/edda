# IMPLEMENTATION_PLAN - Vedrfolnir (Connection Builder Client)

## Übersicht

Vedrfolnir ist der Connection Builder Client - er stellt Verbindungen von User-Devices zu Yggdrasil über das Ratatoskr-Protocol her.

**Programmiersprache**: Rust

## Entschiedene Konfiguration
✅ **WebSocket-Library**: tokio-tungstenite
✅ **TLS-Library**: rustls

---

## Phase 1: Projekt-Setup ✅ ABGESCHLOSSEN
- [x] Cargo-Projekt
- [x] Dependencies (tokio, tokio-tungstenite, rustls, serde, tracing, anyhow)
- [x] Verzeichnisstruktur (`src/connection/`, `src/protocol/`, `src/auth/`)
- [x] Test-Infrastruktur
  - [x] Container-Setup für Tests (Dockerfile, Docker Compose)
  - [x] Mock-Yggdrasil-Service in Containern
  - [x] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [x] Settings-System

## Phase 2: Ratatoskr-Protocol-Client ✅ ABGESCHLOSSEN
- [x] Ratatoskr-Protocol-Definitions importieren
- [x] Tests für Protocol-Client schreiben
- [x] `RatatoskrClient` (TDD, Send-Request, Receive-Response)

## Phase 3: Connection-Builder ✅ ABGESCHLOSSEN
- [x] Tests für Connection-Builder schreiben
- [x] `ConnectionBuilder` (TDD, Establish-WebSocket-Connection, TLS-Handshake)
- [x] Connection-Request/Response-Handling

## Phase 4: Authentication ✅ ABGESCHLOSSEN
- [x] Tests für Auth-Manager schreiben
- [x] `AuthManager` (TDD, Device-Identity, Authentication-Token)
- [x] Heimdall-Integration (gRPC-Client für AuthenticationService und TokenService)

## Phase 5: Message-Sending ⚠️ TEILWEISE
- [x] Tests für Message-Sender schreiben
- [x] `MessageSender` (TDD, Serialize-Message, Sign-Message, Send-via-WebSocket)
- [x] Message-Handler implementiert

## Phase 6: Message-Receiving ⚠️ TEILWEISE
- [x] Tests für Message-Receiver schreiben
- [x] `MessageReceiver` (TDD, Receive-via-WebSocket, Validate-Response, Deserialize)
- [x] Message-Handler implementiert

## Phase 7: Connection-Management ✅ ABGESCHLOSSEN
- [x] Tests für Connection-Manager schreiben
- [x] `ConnectionManager` (TDD, Monitor-Connection, Heartbeat, Automatic-Reconnection)

## Phase 8: Rate-Limiting & Retry ✅ ABGESCHLOSSEN
- [x] Tests für Rate-Limiter schreiben
- [x] `RateLimiter` (TDD)
- [x] Retry-Mechanism (Exponential-Backoff)

## Phase 9: Performance & Security ⚠️ TEILWEISE
- [x] Connection-Pooling (optional)
- [x] TLS 1.3 Hardening
- [x] Audit-Logging

## Phase 10: Documentation & Testing ⚠️ TEILWEISE
- [x] Dokumentation (README.md)
- [x] E2E-Tests (Connection → Send-Request → Receive-Response)
- [ ] Performance-Tests

---

**Schritte gesamt**: ~80
**Phasen**: 10
