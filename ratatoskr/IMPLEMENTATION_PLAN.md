# IMPLEMENTATION_PLAN - Ratatoskr (Business Protocol)

## Übersicht

Ratatoskr ist das Business Protocol - WebSocket-basiertes Protocol für Business-Logic-Kommunikation mit Yggdrasil (Marketplace, Payments, etc.).

**Programmiersprache**: Rust (Protocol-Definition), Implementation in Services

## Entschiedene Konfiguration
✅ **Message-Format**: Protobuf
✅ **Serialization-Library**: serde

---

## Phase 1: Projekt-Setup (8 Schritte)
- [ ] Cargo-Projekt (Protocol-Definition-Projekt)
- [ ] Dependencies (serde, prost oder rmp-serde)
- [ ] Verzeichnisstruktur (`src/protocol/`, `src/messages/`)
- [ ] Test-Infrastruktur
  - [ ] Container-Setup für Tests (Dockerfile, Docker Compose)
  - [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies auf der Entwicklungsmaschine
- [ ] Documentation-Setup

## Phase 2: Protocol-Definition (15 Schritte)
- [ ] Tests für Message-Definitions schreiben
- [ ] `RatatoskrRequest` Message (TDD)
- [ ] `RatatoskrResponse` Message (TDD)
- [ ] Message-Types definieren (MARKETPLACE_REQUEST, PAYMENT_REQUEST, PROVIDER_REGISTRATION, etc.)

## Phase 3: Message-Serialization (10 Schritte)
- [ ] Tests für Serializer schreiben
- [ ] `MessageSerializer` (TDD, Serialize/Deserialize)
- [ ] Protobuf oder MessagePack-Integration

## Phase 4: Message-Validation (10 Schritte)
- [ ] Tests für Validator schreiben
- [ ] `MessageValidator` (TDD, Signature-Validation, Nonce-Validation, Schema-Validation)

## Phase 5: Connection-Protocol (10 Schritte)
- [ ] Tests für Connection-Protocol schreiben
- [ ] `CONNECTION_REQUEST` / `CONNECTION_RESPONSE` Messages (TDD)
- [ ] Handshake-Protocol

## Phase 6: Security-Features (8 Schritte)
- [ ] Tests für Security-Features schreiben
- [ ] Message-Signature (TDD)
- [ ] Nonce-Management (TDD)
- [ ] Encryption-Support (optional)

## Phase 7: Documentation & Examples (6 Schritte)
- [ ] Protocol-Dokumentation
- [ ] Message-Format-Dokumentation
- [ ] Usage-Examples (Client/Server)

## Phase 8: Testing (6 Schritte)
- [ ] Unit-Tests (Message-Serialization, Validation)
- [ ] Integration-Tests (Full-Protocol-Flow)

---

**Schritte gesamt**: ~73
**Phasen**: 8

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
