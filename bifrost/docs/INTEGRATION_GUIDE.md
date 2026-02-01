# Integration Guide für Platforms (Phase 19.3.1)

Anleitung für Platform-Entwickler (Midgard, Alfheim, Asgard, Ragnarok, Jotunheim): Wie Bifrost genutzt wird, Connection-Establishment und Message-Routing mit konkreten Beispielen. Referenz: [BIFROST_PROTOCOL_SPECIFICATION](BIFROST_PROTOCOL_SPECIFICATION.md), [ARCHITECTURE](ARCHITECTURE.md).

---

## 1. Wie Platforms Bifrost nutzen

**Rolle der Platform:** Die Platform (Desktop, Mobile, Homeserver, Terminal, IoT) fungiert als **Client** gegenüber Bifrost. Sie hält eine oder mehrere WebSocket-Verbindungen zu Bifrost (lokal oder über Relay) und tauscht **BifrostMessages** (JSON) aus.

**Typischer Ablauf:**

1. **Verbindung aufbauen**: WebSocket zu Bifrost-URL (z. B. `ws://localhost:8080` oder `wss://asgard.example.com/bifrost`).
2. **Registrieren**: CONNECTION_REQUEST mit `user_id` und `device_id` senden; CONNECTION_RESPONSE abwarten.
3. **Nachrichten senden/empfangen**: MESSAGE mit `source_device_id`/`target_device_id` und `payload`; eingehende MESSAGEs vom Server verarbeiten.
4. **Keep-Alive**: HEARTBEAT in konfigurierbaren Abständen senden/empfangen.
5. **Trennung**: DISCONNECT senden und WebSocket schließen.

**Wichtige Punkte:**

- Jede Nachricht ist ein JSON-Objekt mit `message_id`, `message_type`, `source_device_id`, `target_device_id`, `payload`, `timestamp`, optional `protocol_version`.
- Bifrost routet MESSAGEs an das Ziel-Device (direkt oder über Relay); die Platform muss nur an die eigene Connection senden und von dort empfangen.
- Für Cross-Device-Actions (z. B. Thor): gRPC-Requests werden als GRPC_REQUEST/GRPC_RESPONSE getunnelt (siehe Phase 17).

---

## 2. Connection-Establishment-Examples

### 2.1 WebSocket öffnen und CONNECTION_REQUEST senden

Nach dem Öffnen der WebSocket-Verbindung sendet die Platform eine CONNECTION_REQUEST (als JSON-Text-Frame):

```json
{
  "message_id": "550e8400-e29b-41d4-a716-446655440000",
  "message_type": "CONNECTION_REQUEST",
  "source_device_id": "my-device-id",
  "target_device_id": "server",
  "payload": {
    "user_id": "user-123",
    "device_id": "my-device-id"
  },
  "timestamp": 1700000000,
  "protocol_version": 1
}
```

- `message_id`: Eindeutig (z. B. UUID).
- `source_device_id`: Eure Device-ID (konsistent für diese Platform-Instanz).
- `target_device_id`: Für die erste Registrierung typischerweise `"server"`.
- `payload.user_id`, `payload.device_id`: Für Heimdall-Validation.

### 2.2 CONNECTION_RESPONSE empfangen

Der Server antwortet mit CONNECTION_RESPONSE:

```json
{
  "message_id": "660e8400-e29b-41d4-a716-446655440001",
  "message_type": "CONNECTION_RESPONSE",
  "source_device_id": "server",
  "target_device_id": "my-device-id",
  "payload": { "accepted": true },
  "timestamp": 1700000001,
  "protocol_version": 1
}
```

- Bei `accepted: true`: Verbindung ist angenommen; danach MESSAGE/HEARTBEAT nutzen.
- Bei `accepted: false`: Verbindung abgelehnt (z. B. nach Heimdall DENY); ggf. Fehlermeldung im payload.

### 2.3 Optional: Version-Negotiation vor CONNECTION_REQUEST

Falls Version-Negotiation genutzt wird, zuerst VERSION_NEGOTIATION senden (payload mit angebotener Version), Antwort des Servers mit vereinbarter Version abwarten, dann CONNECTION_REQUEST mit derselben Semantik wie oben.

---

## 3. Message-Routing-Examples

### 3.1 Nachricht an ein anderes Device senden (Direct)

Um eine Anwendungsnachricht an ein anderes Device zu senden, MESSAGE mit Ziel-Device-ID senden:

```json
{
  "message_id": "770e8400-e29b-41d4-a716-446655440002",
  "message_type": "MESSAGE",
  "source_device_id": "my-device-id",
  "target_device_id": "other-device-id",
  "payload": {
    "body": "Hello from platform",
    "content_type": "text/plain"
  },
  "timestamp": 1700000002,
  "protocol_version": 1
}
```

Bifrost leitet die Nachricht an die Connection von `other-device-id` weiter (direkt, wenn verbunden, sonst über Relay).

### 3.2 Nachricht empfangen

Die Platform empfängt auf derselben WebSocket-Verbindung Text-Frames mit JSON. Bei `message_type === "MESSAGE"` und `target_device_id === "my-device-id"` ist es eine für diese Instanz bestimmte Nachricht; `payload` und `source_device_id` auswerten.

Beispiel eingehende MESSAGE:

```json
{
  "message_id": "880e8400-e29b-41d4-a716-446655440003",
  "message_type": "MESSAGE",
  "source_device_id": "other-device-id",
  "target_device_id": "my-device-id",
  "payload": { "body": "Reply", "content_type": "text/plain" },
  "timestamp": 1700000003,
  "protocol_version": 1
}
```

### 3.3 HEARTBEAT senden

Regelmäßig HEARTBEAT senden (z. B. alle 30 s), um die Verbindung am Leben zu halten:

```json
{
  "message_id": "990e8400-e29b-41d4-a716-446655440004",
  "message_type": "HEARTBEAT",
  "source_device_id": "my-device-id",
  "target_device_id": "server",
  "payload": {},
  "timestamp": 1700000004,
  "protocol_version": 1
}
```

### 3.4 ThorAction (gRPC over Bifrost)

Um eine Aktion an ein anderes Device zu senden (z. B. Thor ausführen), nutzt die Platform den gRPC-Tunnel: Bifrost erwartet GRPC_REQUEST/GRPC_RESPONSE. Die eigentliche Serialisierung (z. B. Protobuf) liegt in den `body`-Bytes (base64 im JSON). Die Platform kann dazu die Bifrost-Bibliothek (GrpcBridge/ThorActionRouter) nutzen oder GRPC_REQUEST/GRPC_RESPONSE gemäß [BIFROST_PROTOCOL_SPECIFICATION](BIFROST_PROTOCOL_SPECIFICATION.md) §5.5 selbst bauen.

---

## 4. Übersicht für Platform-Entwickler

| Schritt | Aktion |
|--------|--------|
| 1 | WebSocket zu Bifrost-URL öffnen |
| 2 | CONNECTION_REQUEST (user_id, device_id) senden |
| 3 | CONNECTION_RESPONSE auswerten (accepted?) |
| 4 | MESSAGE mit target_device_id senden für Nachrichten an andere Devices |
| 5 | Eingehende Frames parsen; bei message_type MESSAGE → payload/source_device_id nutzen |
| 6 | HEARTBEAT periodisch senden |
| 7 | Bei Trennung DISCONNECT senden, WebSocket schließen |

### 4.1 Testing (Bifrost bei Integration prüfen)

Zum Abgleich mit dem Bifrost-Protokoll können die Bifrost-Tests im Container ausgeführt werden (siehe [README Phase 20 Test Suites](../README.md#phase-20-test-suites-implementation_plan)):

- **Alle Tests:** `docker compose -f docker-compose.test.yml run --rm bifrost-test` (von `bifrost/` aus).
- **E2E-Workflows:** `tests/e2e_communication_workflow_test.rs` (Discovery→Connection→Message, Direct/Relay, gRPC over Bifrost).
- **Security:** `tests/security_test_suite.rs`; Details in [SECURITY](SECURITY.md).

---

## 5. Referenzen

- [BIFROST_PROTOCOL_SPECIFICATION](BIFROST_PROTOCOL_SPECIFICATION.md) – Message-Format, Message-Types, Connection-/Auth-Workflow, JSON-Beispiele.
- [ARCHITECTURE](ARCHITECTURE.md) – Komponenten, Connection-Establishment- und Message-Routing-Sequenzen.
- [SERVICE_INTEGRATION_PROTOCOLS](SERVICE_INTEGRATION_PROTOCOLS.md) – Heimdall, Asgard, Yggdrasil (für Konfiguration/Relay-URLs).
