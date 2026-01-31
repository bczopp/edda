# IMPLEMENTATION_PLAN - Vedrfolnir (Connection Builder Client)

## Übersicht

Vedrfolnir ist der Connection Builder Client - er stellt Verbindungen von User-Devices zu Yggdrasil über das Ratatoskr-Protocol her.

**Programmiersprache**: Rust

## Entschiedene Konfiguration
✅ **WebSocket-Library**: tokio-tungstenite
✅ **TLS-Library**: rustls

---

## Phase 1: Projekt-Setup (8 Schritte)
- [ ] Cargo-Projekt
- [ ] Dependencies (tokio, tokio-tungstenite, rustls, serde, tracing, anyhow)
- [ ] Verzeichnisstruktur (`src/connection/`, `src/protocol/`, `src/auth/`)
- [ ] Test-Infrastruktur
  - [ ] Container-Setup für Tests (Dockerfile, Docker Compose)
  - [ ] Mock-Yggdrasil-Service in Containern
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [ ] Settings-System

## Phase 2: Ratatoskr-Protocol-Client (10 Schritte)
- [ ] Ratatoskr-Protocol-Definitions importieren
- [ ] Tests für Protocol-Client schreiben
- [ ] `RatatoskrClient` (TDD, Send-Request, Receive-Response)

## Phase 3: Connection-Builder (10 Schritte)
- [ ] Tests für Connection-Builder schreiben
- [ ] `ConnectionBuilder` (TDD, Establish-WebSocket-Connection, TLS-Handshake)
- [ ] Connection-Request/Response-Handling

## Phase 4: Authentication (8 Schritte)
- [ ] Tests für Auth-Manager schreiben
- [ ] `AuthManager` (TDD, Device-Identity, Authentication-Token)
- [ ] Heimdall-Integration

## Phase 5: Message-Sending (8 Schritte)
- [ ] Tests für Message-Sender schreiben
- [ ] `MessageSender` (TDD, Serialize-Message, Sign-Message, Send-via-WebSocket)

## Phase 6: Message-Receiving (8 Schritte)
- [ ] Tests für Message-Receiver schreiben
- [ ] `MessageReceiver` (TDD, Receive-via-WebSocket, Validate-Response, Deserialize)

## Phase 7: Connection-Management (10 Schritte)
- [ ] Tests für Connection-Manager schreiben
- [ ] `ConnectionManager` (TDD, Monitor-Connection, Heartbeat, Automatic-Reconnection)

## Phase 8: Rate-Limiting & Retry (6 Schritte)
- [ ] Tests für Rate-Limiter schreiben
- [ ] `RateLimiter` (TDD)
- [ ] Retry-Mechanism (Exponential-Backoff)

## Phase 9: Performance & Security (6 Schritte)
- [ ] Connection-Pooling (optional)
- [ ] TLS 1.3 Hardening
- [ ] Audit-Logging

## Phase 10: Documentation & Testing (6 Schritte)
- [ ] Dokumentation
- [ ] E2E-Tests (Connection → Send-Request → Receive-Response)
- [ ] Performance-Tests

---

**Schritte gesamt**: ~80
**Phasen**: 10
