# Heimdall Security Guide

## 1. Security-Best-Practices

### Token-Management
- **Heimdall-Token**: 24h Gültigkeit (konfigurierbar). Kurze Laufzeit reduziert Missbrauchsrisiko.
- **Session-Token**: 1h Gültigkeit. Für aktive Sitzungen; Refresh-Token für Verlängerung.
- **Refresh-Token**: 30 Tage. Nur zur Token-Erneuerung; sicher und rotierbar speichern.
- **Keine Secrets im Code**: Alle Secrets (Signing-Keys, DB-Credentials) über Umgebungsvariablen oder sichere Key-Storage.
- **Token-Revocation**: Revozierte Token werden in der Datenbank geführt; Validierung prüft Revocation-Liste.

### Schlüsselverwaltung
- **Ed25519** für Token-Signatur (ring). Keine schwachen Algorithmen.
- **Key-Storage**: Signing-Keys in konfigurierbarem Verzeichnis; Berechtigungen restriktiv setzen (nur Service-User).
- **Key-Rotation**: Unterstützt; alte Keys können für Grace-Period validiert werden.

### Datenbank
- **PostgreSQL** mit TLS für Verbindungen (empfohlen).
- **Passwörter/Secrets**: Nur über `DATABASE_URL` oder Secret-Manager, nie in Config-Dateien committen.
- **Migrations**: Schema-Änderungen nur über Migrations; keine manuellen Änderungen in Production.

### gRPC & TLS
- **TLS 1.3** für alle Service-zu-Service-Verbindungen (security/tls).
- **Certificate Validation**: Server- und Client-Zertifikate validieren; keine Self-Signed in Production ohne explizite Trust-Store-Pflege.

### Bifrost-Validierung
- **Connection-Validation**: Jede Bifrost-Verbindung wird gegen Heimdall validiert (Challenge-Response, Token).
- **Rate-Limiting**: Brute-Force-Schutz; zu viele Fehlversuche führen zu temporärem Block.

### Allgemein
- **Fail-Safe**: Bei Fehlern oder fehlenden Daten: Deny statt Allow.
- **Audit-Logging**: Security-relevante Ereignisse (Token-Erstellung, Revocation, Validierungsfehler) loggen.
- **Least Privilege**: Services und User nur mit minimal nötigen Rechten konfigurieren.

---

## 2. Threat-Models

### 2.1 Angreifer-Modelle

| Bedrohung | Beschreibung | Mitigation |
|-----------|--------------|------------|
| **Token-Diebstahl** | Angreifer erlangt gültigen Token (z.B. MitM, Log-Leak). | Kurze Token-Laufzeit; TLS; Revocation-Liste; Audit-Log bei Nutzung. |
| **Replay-Attacken** | Wiederverwendung abgefangener Requests. | Nonces/Challenges bei Bifrost-Validation; kurze Token-Gültigkeit. |
| **Brute-Force** | Rate von Login/Validierungsversuchen. | Rate-Limiting; Backoff; Account-Lockout (konfigurierbar). |
| **Privilege Escalation** | Nutzer erlangt höhere Rechte. | RBAC; Permission-Check bei jeder Aktion; keine Rechte-Anreicherung ohne explizite Prüfung. |
| **Key-Compromise** | Signing-Key oder DB-Credentials kompromittiert. | Key-Rotation; Credential-Rotation; Audit bei Key-Nutzung. |
| **Insider** | Berechtigter Nutzer nutzt Daten missbräuchlich. | Audit-Logging; Zugriff nur auf notwendige Ressourcen; Review von Berechtigungen. |

### 2.2 Vertrauensgrenzen

- **Heimdall** vertraut: Konfiguration, Key-Storage, Datenbank, TLS-Zertifikaten.
- **Heimdall** vertraut nicht: Client-Requests ohne gültigen Token; unverschlüsselte Kanäle; unbekannte Devices ohne Attestation.
- **Vertrauensannahme**: Netzwerk zwischen Heimdall und Aufrufern ist durch TLS geschützt; Admins konfigurieren Keys und DB sicher.

### 2.3 Angriffsflächen

- **gRPC-API**: Authentifizierung und Autorisierung bei jedem Endpoint; Input-Validierung; Rate-Limiting.
- **Datenbank**: Zugriff nur über Heimdall; keine direkte DB-Exposition; sichere Credentials.
- **Key-Storage**: Dateisystem-Berechtigungen; optional OS-Keyring für Production.
- **Logs**: Keine Tokens/Secrets in Logs; nur Referenzen (z.B. token_id) und Security-Events.

---

## 3. Security-Workflows

### 3.1 Token-Lifecycle
1. **Erstellung**: Client/Device authentifiziert sich (z.B. Challenge-Response, OAuth); Heimdall prüft und stellt Token aus (Signatur mit Signing-Key).
2. **Validierung**: Aufrufer sendet Token; Heimdall prüft Signatur, Ablaufzeit, Revocation-Liste; bei Cache: konsistent mit DB.
3. **Erneuerung**: Refresh-Token an Token-Service; neuer Access/Session-Token; optional Refresh-Token-Rotation.
4. **Revocation**: Admin oder automatisch (z.B. Logout); Eintrag in Revocation-Liste; alle nachfolgenden Validierungen schlagen fehl.

### 3.2 Bifrost-Connection-Validation
1. **Challenge-Request**: Bifrost fragt bei Heimdall eine Challenge für Device/User an.
2. **Challenge-Response**: Client löst Challenge (z.B. Signatur mit Device-Key); Heimdall prüft und gibt Connection-Token aus.
3. **Connection-Validation**: Bei jeder relevanten Nachricht oder periodisch: Token an Heimdall; Heimdall bestätigt Gültigkeit und Berechtigung.
4. **Bei Fehler**: Verbindung ablehnen oder trennen; Event loggen; Rate-Limit berücksichtigen.

### 3.3 Permission-Check
1. **Anfrage**: Aufrufer sendet device_id, user_id, resource_type, action.
2. **Cache**: Optional Permission-Check-Cache (TTL); bei Hit: zurückgeben.
3. **DB-Lookup**: Device und zugehörige Permissions laden; Prüfung resource_type/action; Ergebnis cachen.
4. **Antwort**: Allow/Deny; bei Deny: Grund nicht an Client zurückgeben (nur intern loggen).

### 3.4 Mesh-Membership / Device-Attestation
1. **Registrierung**: Device meldet sich mit Identität und optional Attestation; Heimdall prüft und registriert im Mesh.
2. **Mesh-Token**: Bei Erfolg: Mesh-spezifisches Token für Teilnahme am Mesh.
3. **Updates**: Status- oder Capability-Updates; erneute Prüfung bei kritischen Änderungen.
4. **Abmeldung**: Device oder Admin entfernt Device; Token ungültig.

### 3.5 Incident-Response (empfohlen)
- **Token-Compromise**: Betroffene Token sofort revozieren; ggf. Signing-Key rotieren und alle Tokens invalidiert erklären.
- **Key-Compromise**: Key-Rotation durchführen; Audit-Log prüfen; Credentials wechseln.
- **Verdächtige Aktivität**: Rate-Limits und Audit-Logs auswerten; betroffene Accounts/Devices sperren; Nachanalyse.

---

Dieser Guide deckt **Phase 17.2.1 Security-Guide** (Security-Best-Practices, Threat-Models, Security-Workflows) ab.
