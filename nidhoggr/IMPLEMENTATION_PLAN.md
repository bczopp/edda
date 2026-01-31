# IMPLEMENTATION_PLAN - Nidhöggr (Connection Endpoint & Message Receiver)

## Übersicht

Nidhöggr ist der Server-Side Connection Endpoint bei Yggdrasil. Er empfängt Verbindungen von Vedrfolnir-Clients über das Ratatoskr-Protocol und leitet Nachrichten an entsprechende Services weiter.

**Programmiersprache**: Rust

## Entschiedene Konfiguration

✅ **Protobuf-Rust-Tool**: prost + tonic
✅ **WebSocket-Library**: tokio-tungstenite
✅ **TLS-Library**: rustls

**Begründung**: Beste Performance, async-native, pure Rust, robuste Security

---

## Phase 1: Projekt-Setup (10 Schritte)
- [ ] Cargo-Projekt erstellen
- [ ] Dependencies (tokio, tonic, tokio-tungsten ite, rustls, serde, tracing, anyhow)
- [ ] Verzeichnisstruktur (`src/connection/`, `src/routing/`, `src/validation/`)
- [ ] Test-Infrastruktur
  - [ ] Container-Setup für Tests (Dockerfile, Docker Compose)
  - [ ] Mock-Services in Containern (Nornen, Heidrun, Mimir)
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [ ] CI/CD-Pipeline
- [ ] Settings-System (JSON, Hot-Reload)

## Phase 2: Protobuf & Ratatoskr Protocol (10 Schritte)
- [ ] Shared Protobuf-Projekt verwenden
- [ ] `RatatoskrProtocol.proto` importieren (RatatoskrRequest, RatatoskrResponse)
- [ ] gRPC-Clients für Services (Nornen, Heidrun, Mimir)

## Phase 3: Connection Management (15 Schritte)
- [ ] Tests für Connection-Manager schreiben
- [ ] `ConnectionManager` (TDD, Connection-Empfang, Validation, Monitoring, Termination)
- [ ] Connection-Pooling
- [ ] Heartbeat-System

## Phase 4: WebSocket-Server (10 Schritte)
- [ ] Tests für WebSocket-Server schreiben
- [ ] `WebSocketServer` (TDD, TLS 1.3)
- [ ] Connection-Establishment-Flow
- [ ] Connection-Termination-Flow

## Phase 5: Message-Receiving (15 Schritte)
- [ ] Tests für Message-Receiver schreiben
- [ ] `MessageReceiver` (TDD, Message-Validation, Signature-Check, Nonce-Validation, Rate-Limiting, Audit-Logging)

## Phase 6: Message-Routing (15 Schritte)
- [ ] Tests für Message-Router schreiben
- [ ] `MessageRouter` (TDD, Service-Discovery, Message-Type → Service Mapping, Load-Balancing)

## Phase 7: Rate-Limiting & Security (10 Schritte)
- [ ] Tests für Rate-Limiter schreiben
- [ ] `RateLimiter` (TDD, Per-Device/User Limits)
- [ ] Security-Monitoring
- [ ] Audit-Logger

## Phase 8: Performance-Optimization (10 Schritte)
- [ ] Connection-Pooling optimieren
- [ ] Message-Batching
- [ ] Async-Processing
- [ ] Benchmarks (< 100ms Connection-Latenz, < 10ms Message-Routing, 1000+ Messages/s)

## Phase 9: Monitoring & Logging (5 Schritte)
- [ ] Structured-Logging (tracing)
- [ ] Performance-Monitor

## Phase 10: Documentation & Testing (10 Schritte)
- [ ] Service-Dokumentation
- [ ] E2E-Tests (Connection → Message → Routing → Response)
- [ ] Performance-Tests
- [ ] Security-Tests

---

**Schritte gesamt**: ~100
**Phasen**: 10
