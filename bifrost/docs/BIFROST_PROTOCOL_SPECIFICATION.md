# Bifrost Protocol Specification (Phase 19.1.1)

Spezifikation des Bifrost-Protokolls für device-to-device Kommunikation über WebSocket. Abhängigkeiten: Phase 2.1 (Message Format), Phase 4.1 (Challenge-Response).

---

## 1. Message-Format

Alle Nachrichten sind JSON-Objekte mit folgender Grundstruktur:

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| `message_id` | string | Eindeutige ID der Nachricht (z. B. UUID). |
| `message_type` | string | Siehe Abschnitt 2 (Message-Types). |
| `source_device_id` | string | Device-ID des Senders. |
| `target_device_id` | string | Device-ID des Ziels (oder Platzhalter bei Broadcast). |
| `payload` | object | Typ-spezifische Nutzdaten (JSON). |
| `timestamp` | number | Unix-Zeitstempel (Sekunden). |
| `protocol_version` | number? | Optionale Major-Protokollversion. |

Serialisierung: UTF-8 JSON. Transport: WebSocket Text-Frames.

---

## 2. Message-Types

| `message_type` | Richtung | Beschreibung |
|----------------|----------|--------------|
| `CONNECTION_REQUEST` | Client → Server | Verbindungsanfrage (z. B. user_id, device_id). |
| `CONNECTION_RESPONSE` | Server → Client | Annahme/Ablehnung der Verbindung. |
| `MESSAGE` | bidirektional | Anwendungsnachricht (payload app-spezifisch). |
| `HEARTBEAT` | bidirektional | Keep-Alive. |
| `DISCONNECT` | bidirektional | Geplante Trennung. |
| `ERROR` | bidirektional | Fehlermeldung. |
| `VERSION_NEGOTIATION` | bidirektional | Protokollversion aushandeln (Semantic Versioning). |
| `CHALLENGE_REQUEST` | Server → Client | Challenge zur Authentifizierung. |
| `CHALLENGE_RESPONSE` | Client → Server | Challenge-Antwort (z. B. signiert). |
| `CHALLENGE_PROOF` | Client → Server | Beweis (Signatur über Challenge). |
| `AUTHENTICATION_RESULT` | Server → Client | Ergebnis der Authentifizierung. |
| `GRPC_REQUEST` | bidirektional | Getunnelter gRPC-Request (Phase 17). |
| `GRPC_RESPONSE` | bidirektional | Getunnelter gRPC-Response. |

---

## 3. Connection-Workflow

1. **Transport**: Client öffnet WebSocket-Verbindung (ws:// oder wss://) zum Bifrost-Server.
2. **Version-Negotiation** (optional): `VERSION_NEGOTIATION` mit angebotener Version; Server antwortet mit höchster gemeinsamer Version.
3. **Connection-Request**: Client sendet `CONNECTION_REQUEST` (z. B. user_id, device_id im payload).
4. **Validation**: Server validiert ggf. über Heimdall (siehe [SERVICE_INTEGRATION_PROTOCOLS.md](SERVICE_INTEGRATION_PROTOCOLS.md)).
5. **Connection-Response**: Server sendet `CONNECTION_RESPONSE` (angenommen/abgelehnt).
6. **Challenge/Authentication** (wenn aktiviert): Siehe Abschnitt 4.
7. Danach: Austausch von `MESSAGE`, `HEARTBEAT`; bei Trennung `DISCONNECT`.

---

## 4. Authentication-Workflow

1. Server sendet `CHALLENGE_REQUEST` (z. B. nonce, Ablaufzeit).
2. Client antwortet mit `CHALLENGE_RESPONSE` und/oder `CHALLENGE_PROOF` (Signatur über Challenge mit Device-Key).
3. Server prüft Signatur und sendet `AUTHENTICATION_RESULT` (erfolgreich/fehlgeschlagen).
4. Bei Erfolg: Verbindung gilt als authentifiziert; Token/Claims können im weiteren Flow genutzt werden.

Details: Phase 4.1 (Challenge-Request/Response/Proof), Phase 4.2 (Token Generator/Validator). Erweiterte Beschreibung: Phase 19.1.2 (Connection/Authentication-Protocol-Specification).

---

## 5. Protocol-Examples

### 5.1 Connection-Request

```json
{
  "message_id": "550e8400-e29b-41d4-a716-446655440000",
  "message_type": "CONNECTION_REQUEST",
  "source_device_id": "device-abc",
  "target_device_id": "server",
  "payload": { "user_id": "user-1", "device_id": "device-abc" },
  "timestamp": 1700000000,
  "protocol_version": 1
}
```

### 5.2 Connection-Response

```json
{
  "message_id": "660e8400-e29b-41d4-a716-446655440001",
  "message_type": "CONNECTION_RESPONSE",
  "source_device_id": "server",
  "target_device_id": "device-abc",
  "payload": { "accepted": true },
  "timestamp": 1700000001,
  "protocol_version": 1
}
```

### 5.3 Application Message (MESSAGE)

```json
{
  "message_id": "770e8400-e29b-41d4-a716-446655440002",
  "message_type": "MESSAGE",
  "source_device_id": "device-abc",
  "target_device_id": "device-xyz",
  "payload": { "body": "Hello", "content_type": "text/plain" },
  "timestamp": 1700000002,
  "protocol_version": 1
}
```

### 5.4 Heartbeat

```json
{
  "message_id": "880e8400-e29b-41d4-a716-446655440003",
  "message_type": "HEARTBEAT",
  "source_device_id": "device-abc",
  "target_device_id": "server",
  "payload": {},
  "timestamp": 1700000003,
  "protocol_version": 1
}
```

### 5.5 gRPC-Tunnel (GRPC_REQUEST)

```json
{
  "message_id": "990e8400-e29b-41d4-a716-446655440004",
  "message_type": "GRPC_REQUEST",
  "source_device_id": "device-abc",
  "target_device_id": "device-xyz",
  "payload": {
    "request_id": "req-uuid",
    "service": "thor.Thor",
    "method": "Execute",
    "body": "<base64-encoded-bytes>"
  },
  "timestamp": 1700000004,
  "protocol_version": 1
}
```

---

## 6. Referenzen

- [Service-Integration-Protocols](SERVICE_INTEGRATION_PROTOCOLS.md) – Heimdall, Asgard, Yggdrasil.
- [MESH_LAYER_DESIGN](MESH_LAYER_DESIGN.md) – Mesh-Paketformat und -Routing.
- IMPLEMENTATION_PLAN Phase 2.1 (Message Format), Phase 4.1 (Challenge), Phase 6/7 (WebSocket Server/Client).
- [README Phase 20 Test Suites](../README.md#phase-20-test-suites-implementation_plan) – E2E-, Performance-, Security- und GDPR-Tests für das Protokoll.
