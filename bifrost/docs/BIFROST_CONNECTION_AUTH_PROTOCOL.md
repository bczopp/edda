# Connection / Authentication Protocol (Phase 19.1.2)

Dokumentation des Connection- und Authentifizierungs-Protokolls: Challenge-Response, Token-Management, Rate-Limiting. Referenz: Phase 4 (Connection/Authentication Protocol), [BIFROST_PROTOCOL_SPECIFICATION](BIFROST_PROTOCOL_SPECIFICATION.md).

---

## 1. Challenge-Response-Mechanismus

### 1.1 Ablauf

1. **CHALLENGE_REQUEST** (Server → Client oder Initiator → Akzeptor)  
   Enthält u. a. Device-ID, Public-Key des Clients und eine Signatur über `source|target|timestamp|public_key` (Ed25519), um die Anfrage zu binden.

2. **CHALLENGE_RESPONSE** (Client → Server)  
   Antwort mit zufälligem Challenge-String, Ablaufzeit (Expiration) und Signatur des Herausgebers (z. B. Server), der die Challenge ausgestellt hat.

3. **CHALLENGE_PROOF** (Client → Server)  
   Der Client signiert den erhaltenen Challenge-String mit seinem Private-Key. Payload enthält Challenge-String und Signatur (Ed25519).

4. **Validierung**  
   Der Server prüft mit dem Public-Key des Clients die Signatur im CHALLENGE_PROOF und ob die Challenge noch nicht abgelaufen ist (`ChallengeProofValidator`). Bei Erfolg gilt die Verbindung als authentifiziert.

### 1.2 Komponenten (Bifrost)

| Komponente | Phase | Beschreibung |
|------------|-------|--------------|
| `ChallengeRequestHandler` | 4.1.1 | Erstellt CHALLENGE_REQUEST (device_id, public_key, Signatur). |
| `ChallengeResponseGenerator` | 4.1.2 | Erstellt CHALLENGE_RESPONSE (Challenge-String, Expiration, Signatur). |
| `ChallengeProofHandler` | 4.1.3 | Erstellt CHALLENGE_PROOF (Challenge signiert mit Client-Private-Key). |
| `ChallengeProofValidator` | 4.1.4 | Prüft CHALLENGE_PROOF (Signatur, Ablaufzeit). |

### 1.3 Sicherheit

- Alle relevanten Felder (z. B. source, target, timestamp, public_key bzw. Challenge-String) sind in die Signatur einbezogen (Ed25519).
- Ablaufzeit begrenzt die Gültigkeit von Challenge und Proof (Replay-Schutz).

---

## 2. Token-Management

### 2.1 Token-Typen

- **Access-Token**: Kurzlebig; Berechtigung für API/Connection-Nutzung (Subject = user_id, device_id, exp, type, jti).
- **Refresh-Token**: Langlebiger; nur zur Ausstellung neuer Access- und Refresh-Tokens (type = "refresh").

Format (Heimdall-kompatibel): `base64(payload_json).base64(ed25519_signature)`. Payload: `sub`, `device_id`, `exp`, `type`, `jti`.

### 2.2 Ablauf

1. Nach erfolgreicher Challenge-Proof-Validierung können Access- und ggf. Refresh-Token ausgestellt werden (`TokenGenerator`).
2. Eingehende Requests/Connections können mit dem Access-Token autorisiert werden (`TokenValidator`: Signatur, Ablauf, optional JTI-Revocation).
3. Vor Ablauf des Access-Tokens kann der Client mit dem Refresh-Token neue Token anfordern (`TokenRefreshManager`): Validierung des Refresh-Tokens, Ausstellung neuer Access- und Refresh-Token, optional proaktive Erneuerung anhand eines Schwellenwerts.

### 2.3 Komponenten (Bifrost)

| Komponente | Phase | Beschreibung |
|------------|-------|--------------|
| `TokenGenerator` | 4.2.1 | Erzeugt signierte Access- und Refresh-Tokens (subject, device_id, exp, jti). |
| `TokenValidator` | 4.2.2 | Prüft Signatur, Ablauf, optional JTI-Blacklist (Revocation). |
| `TokenRefreshManager` | 4.2.3 | Validiert Refresh-Token, gibt neue Access/Refresh-Token aus, proaktive Erneuerung. |

### 2.4 Revocation

- Revocation über JTI-Blacklist: Einmal verwendete oder widerrufene Token-IDs (jti) werden in einer Blacklist geführt; `TokenValidator` lehnt Tokens mit dieser jti ab.

---

## 3. Rate-Limiting

### 3.1 Verhalten

- **Sliding-Window**: Pro Schlüssel (z. B. device_id oder IP) sind maximal `max_requests` Anfragen innerhalb eines Zeitfensters `window_duration` erlaubt.
- **Token-basiert**: Jeder Aufruf von `check(key)` zählt als ein Request; bei Überschreitung wird `Err(RateLimitExceeded { retry_after })` zurückgegeben.
- **retry_after**: Gibt an, wie lange der Client warten soll, bis wieder Kapazität im Fenster frei ist.

### 3.2 Komponente (Bifrost)

| Komponente | Phase | Beschreibung |
|------------|-------|--------------|
| `RateLimiter` | 4.3.1 | Sliding-Window pro Key; `check(key)` → Ok(()) oder RateLimitExceeded mit retry_after. |

### 3.3 Einsatz

- Begrenzung von Connection-Versuchen, Authentifizierungs-Anfragen oder Message-Throughput pro Device/Connection, um Missbrauch und Überlastung zu begrenzen.

---

## 4. Referenzen

- [BIFROST_PROTOCOL_SPECIFICATION](BIFROST_PROTOCOL_SPECIFICATION.md) – Message-Types, Connection- und Authentication-Workflow (Überblick).
- IMPLEMENTATION_PLAN Phase 4.1 (Challenge-Response), Phase 4.2 (Token), Phase 4.3 (Rate Limiter).
- `src/security/challenge.rs`, `src/security/token.rs`, `src/security/rate_limiter.rs`.
