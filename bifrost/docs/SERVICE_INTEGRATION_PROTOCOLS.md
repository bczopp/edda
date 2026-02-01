# Service-Integration-Protocols (Phase 2.1.4)

Dieses Dokument definiert die Protokolle, über die Bifrost mit externen Services (Heimdall, Asgard, Yggdrasil) integriert wird.

---

## 1. Heimdall: gRPC (Connection Validation)

**Zweck**: Verbindungsvalidierung vor dem Aufbau bzw. der Annahme von WebSocket-Connections.

**Protokoll**: gRPC (Request/Response).

**Bifrost-Seite**:
- **Phase 5.1.1**: Heimdall gRPC Client (`HeimdallClient`, `HeimdallConnectionValidator`).
- **Phase 5.2.1**: Connection Validation Request Handler (`ConnectionValidationHandler`) – baut signierte `ConnectionValidationRequest`, sendet an Heimdall.
- **Phase 5.2.2**: Validation Response Handler (`ValidationResponseHandler`) – wertet `ConnectionValidationResponse` (ALLOW/DENY, Validation-Token) aus.

**Nachrichten**:
- Request: `ConnectionValidationRequest` (user_id, device_id, timestamp, request_signature).
- Response: `ConnectionValidationResponse` (allowed, validation_token).

**Referenz**: IMPLEMENTATION_PLAN Phase 5 (Connection Validation).

---

## 2. Asgard: WebSocket-Relay (Device-Relay)

**Zweck**: Nachrichten über den Asgard-Homeserver an andere Devices routen (Relay bei nicht erreichbarem Direktpfad).

**Protokoll**: WebSocket (persistente Verbindung Bifrost ↔ Asgard).

**Bifrost-Seite**:
- **Phase 9.2.1**: Relay Manager (`RelayManager`) – wählt Relay (Asgard/Yggdrasil), Failover.
- **Phase 9.2.2**: Asgard Relay Client (`AsgardRelayClient`) – Connection zu Asgard-Relay, Message-Routing über Asgard.

**Eigenschaften**:
- Asgard fungiert als WebSocket-Relay; Bifrost sendet/empfängt Messages über dieselbe Verbindung.
- Relay-Auswahl und Fallback über `FallbackRoutingManager` (Phase 9.2).

**Referenz**: IMPLEMENTATION_PLAN Phase 9.2 (Relay Routing), Abschnitt 9.2.2.

---

## 3. Yggdrasil Ratatoskr: WebSocket (Business-Logic)

**Zweck**: Business-Logik mit Yggdrasil (Marketplace, Payments, persistente Sessions).

**Protokoll**: Ratatoskr-Protokoll über WebSocket (persistente Verbindung).

**Bifrost-Seite**:
- **Phase 9.2.3**: Yggdrasil Relay Client (`YggdrasilRelayClient`) – persistente Connection zu Yggdrasil, Message-Routing, Event-Notifications.

**Eigenschaften**:
- Ratatoskr nutzt WebSocket für langlebige Sessions.
- Yggdrasil kann Events an Bifrost pushen; Bifrost sendet Requests (z. B. Marketplace, Payments) über dieselbe Verbindung.

**Referenz**: IMPLEMENTATION_PLAN Phase 9.2.3 (Yggdrasil Relay Client).

---

## 4. Yggdrasil API: gRPC (Device-Registry, User-Management)

**Zweck**: Device-Registry und User-Management (z. B. Device-Liste pro User, Registrierung).

**Protokoll**: gRPC (Request/Response).

**Bifrost-Seite**:
- **Phase 8.3.1**: Yggdrasil Discovery Client (`YggdrasilDiscoveryClient`) – ruft Device-Liste für einen User ab (über `YggdrasilDiscoveryProvider`).
- Discovery-Request/Response können über gRPC oder einen Stub abgebildet werden.

**Typische Operationen**:
- Device-Liste für user_id abrufen (`fetch_devices(user_id)`).
- (Später) Device-Registrierung, User-Management.

**Referenz**: IMPLEMENTATION_PLAN Phase 8.3 (Global Discovery via Yggdrasil), Phase 9.2.3.

---

## Übersicht

| Service        | Protokoll     | Verwendung in Bifrost                          |
|----------------|---------------|-------------------------------------------------|
| Heimdall       | gRPC          | Connection Validation (Phase 5)                |
| Asgard         | WebSocket     | Device-Relay (Phase 9.2.2)                     |
| Yggdrasil      | WebSocket     | Ratatoskr Business-Logic (Phase 9.2.3)         |
| Yggdrasil API  | gRPC          | Device-Registry, Discovery (Phase 8.3, 9.2.3)  |
