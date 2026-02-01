# Security (Phase 15.3)

Überblick über Security-Maßnahmen und Umgang mit Security-Findings. Referenz: IMPLEMENTATION_PLAN Phase 15 (Security).

---

## 1. Vulnerability-Scanning (cargo-audit)

- **CI**: Der Job „Security (cargo-audit)“ in [.github/workflows/bifrost.yml](../.github/workflows/bifrost.yml) führt `cargo audit` gegen die RustSec Advisory Database aus.
- **Lokal**: `cargo install cargo-audit` und im Projekt `cargo audit` (bzw. `cargo audit --manifest-path bifrost/Cargo.toml` von Repo-Root).
- **Bei Findings**: Siehe Abschnitt 3 (Security-Findings dokumentieren und beheben).

---

## 2. Security-Audit und Penetration-Testing

- **Security-Audit**: Periodisch oder vor Releases; manueller Review von Konfiguration, Authentifizierung, Autorisierung, Rate-Limiting, Audit-Logging.
- **Penetration-Testing**: Optional; z. B. gegen WebSocket-Endpoints, Heimdall-Integration, Relay-Pfade.
- **Ergebnisse**: In Findings dokumentieren (Abschnitt 3) und priorisieren.

---

## 3. Security-Findings dokumentieren und beheben

### 3.1 Dokumentation

- **Quelle**: cargo-audit, manueller Audit, Penetration-Test, Bug-Bounty oder Meldung.
- **Pro Finding** (z. B. in einem internen Tracking oder in `docs/SECURITY_FINDINGS.md`):
  - **ID/Title**: Kurze Kennung.
  - **Quelle**: z. B. cargo-audit, CVE-XXXX, manueller Test.
  - **Beschreibung**: Was ist betroffen (Komponente, Endpoint, Abhängigkeit)?
  - **Risiko**: Kritisch / Hoch / Mittel / Niedrig.
  - **Status**: Offen / In Bearbeitung / Behoben / Risiko akzeptiert.
  - **Maßnahme**: Geplante oder durchgeführte Änderung (Patch, Upgrade, Konfiguration, Abschaltung).
  - **Datum**: Erfassung, ggf. Behebung.

### 3.2 Behebung

- **Abhängigkeiten (cargo-audit)**: Upgrade auf sichere Version der Crate; falls kein Fix: Risiko bewerten, Mitigation dokumentieren oder temporär `[patch]` / Fork prüfen.
- **Eigenentwicklung**: Fix nach Priorität; Test (inkl. [websocket_security_test](../tests/websocket_security_test.rs)) ergänzen; Review.
- **Konfiguration**: Sichere Defaults, keine Secrets in Repo; Dokumentation anpassen.

### 3.3 Beispiel-Eintrag (Template)

```markdown
## Finding-XXX: Kurztitel
- **Quelle**: cargo-audit / CVE-XXXX / manuell
- **Beschreibung**: Betroffene Komponente und Kurzbeschreibung.
- **Risiko**: Hoch
- **Status**: Offen
- **Maßnahme**: Geplante Änderung (z. B. Upgrade crate x auf Version y).
- **Erfasst**: YYYY-MM-DD
```

---

## 4. Security Test Suite (Phase 20.3.1)

- **[tests/security_test_suite.rs](../tests/security_test_suite.rs)** – Zentrale Security-Suite: WebSocket-Security (Validation DENY/ALLOW), Unauthorized-Access-Prevention (Cross-User-Block, Threat-Block + Revoke), Connection-Authentication (Challenge-Request), Message-Validation (Invalid Format, PayloadTooLarge, Sanitize).
- Ausführen: `docker compose -f docker-compose.test.yml run --rm bifrost-test` (alle Tests inkl. Security).

## 5. Referenzen

- [IMPLEMENTATION_PLAN](../IMPLEMENTATION_PLAN.md) Phase 15.3 (Security Audit, WebSocket Security Tests), Phase 20.3.1 (Security Test Suite).
- [tests/websocket_security_test.rs](../tests/websocket_security_test.rs) – Unauthorized-Access-Prevention.
- [BIFROST_CONNECTION_AUTH_PROTOCOL](BIFROST_CONNECTION_AUTH_PROTOCOL.md) – Challenge/Token/Rate-Limit.
