# Heimdall gRPC API

Kurzbeschreibung der gRPC-Services von Heimdall (Security Service). Basis: Protobuf-Definitionen in `proto/`.

---

## 1. Authentication Service

**Package:** `heimdall.authentication`

Device-Authentifizierung per Challenge-Response; Ausstellung von Heimdall- und Session-Tokens.

| RPC | Request | Response | Kurzbeschreibung |
|-----|---------|----------|------------------|
| RequestChallenge | ChallengeRequest (device_id, public_key, signature, timestamp) | ChallengeResponse (challenge, timestamp, expires_in, signature) | Fordert eine Challenge für das Device an. |
| ProveIdentity | ProofRequest (device_id, challenge, proof, timestamp, signature) | AuthenticationTokenResponse (token, token_id, expires_at, refresh_token, …) | Beweist Identität mit signierter Challenge; liefert Token. |
| GenerateToken | TokenGenerationRequest (device_id, user_id, permissions) | AuthenticationTokenResponse | **Unimplemented** – Token-Generierung ohne Challenge (Reserviert). |

**Typische Fehler:** `InvalidArgument` (ungültige UUID/Format), `Unauthenticated` (Signatur/Proof ungültig), `Internal` (DB/Key-Fehler).

---

## 2. Authorization Service

**Package:** `heimdall.authorization`

Prüfung von Permissions und Rollen für Device/User.

| RPC | Request | Response | Kurzbeschreibung |
|-----|---------|----------|------------------|
| CheckPermission | PermissionCheckRequest (device_id, user_id, resource_type, action, resource_id?, context?) | PermissionCheckResponse (allowed, reason, granted_permissions) | Prüft, ob die Aktion erlaubt ist. |
| CheckRole | RoleCheckRequest (device_id, user_id, role_name) | RoleCheckResponse (has_role, roles) | Prüft, ob Device/User die Rolle hat; liefert alle Rollen. |

**Typische Fehler:** `InvalidArgument` (fehlende IDs), `NotFound` (Device/User unbekannt), `Internal` (DB-Fehler).

---

## 3. Token Service

**Package:** `heimdall.token`

Validierung, Erneuerung und Widerruf von Tokens.

| RPC | Request | Response | Kurzbeschreibung |
|-----|---------|----------|------------------|
| ValidateToken | ValidateTokenRequest (token, device_id?) | ValidateTokenResponse (valid, token_id, device_id, user_id, expires_at, is_revoked, permissions, reason) | Validiert Token (Format, Signatur, Ablauf, Revocation). |
| RenewToken | RenewTokenRequest (refresh_token, device_id) | RenewTokenResponse (token, token_id, expires_at, refresh_token, refresh_expires_at) | Erneuert Token mit Refresh-Token. |
| RevokeToken | RevokeTokenRequest (token_id, device_id?, reason?) | RevokeTokenResponse (revoked, message) | Widerruft Token. |

**Typische Fehler:** `InvalidArgument` (ungültiges Token/Format), `Unauthenticated` (Token ungültig/abgelaufen), `NotFound` (Token/Device nicht gefunden), `Internal` (DB-Fehler).

---

## 4. Bifrost Validation Service

**Package:** `heimdall.bifrost_validation`

Validierung von Bifrost-Verbindungen und -Nachrichten.

| RPC | Request | Response | Kurzbeschreibung |
|-----|---------|----------|------------------|
| ValidateConnection | ConnectionValidationRequest (source/target device_id, user_id, connection_type, network_id?, timestamp, signature) | ConnectionValidationResponse (allowed, reason, validation_token, expires_at, status, signature) | Prüft, ob die Verbindung source→target erlaubt ist (inkl. Guest-Netzwerk). |
| ValidateMessage | MessageValidationRequest (connection_token, message, signature, timestamp) | MessageValidationResponse (valid, reason) | Prüft Nachricht und Signatur für eine validierte Verbindung. |

**Typische Fehler:** `InvalidArgument` (fehlende/schwache Signatur), `NotFound` (Device/Connection unbekannt), `PermissionDenied` (Verbindung nicht erlaubt), `Internal` (Validierungsfehler).

---

## 5. Mesh Membership Service

**Package:** `heimdall.mesh_membership`

Mesh-Membership und Mesh-Auth-Tokens (Bifrost Device-Mesh).

| RPC | Request | Response | Kurzbeschreibung |
|-----|---------|----------|------------------|
| RegisterDevice | MeshMembershipRequest (device_id, device_name, device_type, mesh_public_key, owner_user_id, timestamp, signature) | MeshMembershipResponse (registered, requires_approval, message, mesh_device_id) | Registriert Device im Mesh; neue Devices benötigen Owner-Freigabe. |
| GenerateMeshAuthToken | MeshAuthTokenRequest (device_id, mesh_device_id, timestamp, signature) | MeshAuthTokenResponse (mesh_token, role, expires_at, signature) | Erzeugt Mesh-Auth-Token für aktives, freigegebenes Device. |

**Typische Fehler:** `InvalidArgument` (ungültige UUID/leeres mesh_public_key), `NotFound` (Device nicht in Mesh/Mesh-Device nicht gefunden), `PermissionDenied` (Device nicht aktiv), `Internal` (Registrierung/Token-Generierung fehlgeschlagen).

---

## Allgemeines

- **Transport:** gRPC (tonic); Port konfigurierbar über Settings (`grpc_port`).
- **Health:** tonic-health für Liveness/Readiness.
- **Fehler:** gRPC-Status-Codes (z. B. `InvalidArgument`, `NotFound`, `PermissionDenied`, `Internal`); Details oft in Status-Message.
- **Sicherheit:** Alle RPCs erfordern gültige Geräte-Identität/Signatur wo im Proto vorgesehen; Tokens signiert (Heimdall Key).
