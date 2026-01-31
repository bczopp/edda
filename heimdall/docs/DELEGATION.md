# Heimdall – Team-Delegation

**Rolle**: Du bist Manager. Du delegierst die offenen Aufgaben an Teams. Jedes Team erledigt seine Aufgaben einfach und überschaubar. Danach wird alles geprüft.

**Nutzung**:
- **Als Manager**: Dieses Dokument an die Teams weitergeben; nach Abschluss den Abschnitt **Manager-Check** ausführen.
- **Als Team**: Den eigenen Team-Abschnitt (z. B. „Team Bifrost“) lesen, Auftrag 1:1 umsetzen (TDD), dann **Abnahme** selbst prüfen und IMPLEMENTATION_PLAN/IMPLEMENTATION_STATUS aktualisieren.

---

## Übersicht

| Team | Fokus | Aufwand |
|------|--------|--------|
| **Team Bifrost** | Message-Validator + Validation-Cache | 2 Komponenten |
| **Team Caching** | Token-Cache + Permission-Cache | 2 Komponenten |
| **Team Mesh** | Mesh-Token-Generator-Tests + optional Email-Hinweis | 1–2 Punkte |
| **Team Infra** | Logging-Setup + optional Metrics-Stub | 1–2 Punkte |
| **Team Docs** | API-Dokumentation (gRPC) | 1 Deliverable |
| **Team QA** | E2E-/Security-/Performance-Test-Skelette | 3 kleine Suites |

**Nach allen Teams**: Manager-Check (siehe unten).

---

## Team Bifrost

**Ziel**: Phase 9.5.1 und 9.6.1 abarbeiten (TDD).

**Auftrag**:
1. **9.5.1 Message-Validator**
   - Tests zuerst: `tests/unit/message_validator_test.rs` (Signatur prüfen, ungültige Message verwerfen).
   - Implementierung: `src/bifrost/message_validator.rs` – `MessageValidator` mit `verify_message(public_key, message, signature) -> Result<bool, Error>`, nutzt `SignatureManager::verify`.
   - IMPLEMENTATION_PLAN und IMPLEMENTATION_STATUS für 9.5.1 abhaken.

2. **9.6.1 Validation-Cache-Manager**
   - Tests zuerst: `tests/unit/validation_cache_test.rs` (cache set/get, TTL, invalidate).
   - Implementierung: `src/bifrost/validation_cache.rs` – Connection-Validierung cachen (z. B. Key `source:target`, TTL z. B. 5 Min), Invalidierung bei Bedarf (z. B. `invalidate_device(device_id)`).
   - IMPLEMENTATION_PLAN und IMPLEMENTATION_STATUS für 9.6.1 abhaken.

**Abnahme**: Beide Testdateien laufen (Container: `docker compose -f docker-compose.test.yml run --rm heimdall-test`), IMPLEMENTATION_PLAN-Checkboxen für 9.5.1 und 9.6.1 sind [x].

---

## Team Caching

**Ziel**: Phase 13.1.1 und 13.2.1 (Token- und Permission-Cache) – TDD.

**Auftrag**:
1. **13.1.1 Token-Cache-Manager**
   - Tests: `tests/unit/token_cache_test.rs` (Token-Validierung cachen, TTL, Invalidierung bei Revocation).
   - Implementierung: `src/token/cache.rs` (oder unter `utils/cache.rs` erweitern) – `TokenCacheManager` mit get/set/invalidate, TTL z. B. 5 Min.
   - IMPLEMENTATION_PLAN 13.1.1 abhaken.

2. **13.2.1 Permission-Cache-Manager**
   - Bereits `PermissionCheckCache` in `utils/cache.rs` – prüfen, ob als „Permission-Cache-Manager“ abgedeckt; wenn ja, nur IMPLEMENTATION_PLAN 13.2.1 abhaken und ggf. kurze Tests ergänzen. Sonst: kleine Wrapper-Komponente + Tests, dann abhaken.

**Abnahme**: Tests grün, IMPLEMENTATION_PLAN 13.1.1 und 13.2.1 [x].

---

## Team Mesh

**Ziel**: Lücken in Phase 10 schließen (nur einfache, klare Punkte).

**Auftrag**:
1. **10.3.1 Mesh-Token-Generator – Tests**
   - Dedizierte Tests für Mesh-Token-Generierung in `tests/unit/mesh_token_generator_test.rs` (oder in bestehendem Mesh-Test): Token-Format, Role, Ablauf.
   - IMPLEMENTATION_PLAN 10.3.1 „Tests für Mesh-Token-Generator schreiben“ und „Tests ausführen und bestehen“ abhaken.

2. **10.2.1 Email-Benachrichtigung**
   - Nur als Stub/Platzhalter: z. B. Funktion `notify_owner_new_device(owner_id, device_id)` die derzeit nur loggt (oder unimplemented! zurückgibt), damit der Plan-Punkt „Email-Benachrichtigung an Owner“ als „Stub implementiert“ abgehakt werden kann. Kein echter E-Mail-Versand nötig.

**Abnahme**: Neue/angepasste Tests grün, IMPLEMENTATION_PLAN 10.2.1/10.3.1 entsprechend aktualisiert.

---

## Team Infra

**Ziel**: Phase 16.1 (Logging) und optional 16.2 (Metrics) – minimal.

**Auftrag**:
1. **16.1.1 Logging**
   - Structured Logging mit `tracing` bestätigen/ergänzen (z. B. in `main.rs` oder Bootstrap), Security-relevante Log-Level dokumentieren (z. B. in README oder IMPLEMENTATION_PLAN).
   - IMPLEMENTATION_PLAN 16.1.1 Checkboxen abhaken (Structured-Logging, Log-Levels, ggf. Log-Rotation als „konfigurierbar“).

2. **16.2.1 Metrics (optional, Stub)**
   - Wenn leicht machbar: kleiner `MetricsCollector`-Stub (z. B. Zähler für Token-Validierungen), sonst nur 16.1 abhaken und 16.2 als „optional, später“ vermerken.

**Abnahme**: Logging läuft, IMPLEMENTATION_PLAN 16.1 (und ggf. 16.2) aktualisiert.

---

## Team Docs

**Ziel**: Phase 17.1 API-Dokumentation.

**Auftrag**:
1. **17.1.1 gRPC-Service-Documentation**
   - Eine Datei (z. B. `docs/api-grpc.md`) mit kurzer Beschreibung der gRPC-Services: Authentication, Authorization, Token, Bifrost-Validation, Mesh-Membership (Endpoints, typische Request/Response, Fehler).
   - IMPLEMENTATION_PLAN 17.1.1 Checkboxen für „gRPC-Service-Documentation“ und die fünf Services abhaken.

**Abnahme**: `docs/api-grpc.md` vorhanden und in IMPLEMENTATION_PLAN vermerkt.

---

## Team QA

**Ziel**: Phase 18 – Test-Skelette für E2E, Security, Performance (ohne vollständige Implementierung).

**Auftrag**:
1. **18.1.1 E2E-Test-Skelett**
   - Eine Datei `tests/e2e/security_workflows_test.rs` (oder unter `tests/integration/`) mit mindestens einem E2E-Skelett (z. B. „Challenge-Response → Token“), der mit TestDatabase/Container läuft; kann zunächst nur einen Happy-Path abdecken.

2. **18.2.1 Security-Test-Skelett**
   - Eine Datei `tests/security/auth_bypass_test.rs` (oder ähnlich) mit 1–2 Tests (z. B. ungültiges Token wird abgelehnt, unbekanntes Device wird abgelehnt).

3. **18.3.1 Performance-Test-Skelett**
   - Eine Datei `tests/performance/token_validation_bench_test.rs` (oder in `benches/`) mit einem einfachen Benchmark/Test für Token-Validierung; Ziel „< 10ms“ in README oder Kommentar festhalten.

**Abnahme**: Drei Test-/Bench-Dateien vorhanden, in IMPLEMENTATION_PLAN 18.1/18.2/18.3 als „Skelette erstellt“ abgehakt, Container-Tests laufen weiter.

---

## Manager-Check (nach allen Teams)

**Der Manager (oder ein abschließender Agent) führt aus**:

1. **Build & Tests**
   - `docker compose -f heimdall/docker-compose.test.yml run --rm heimdall-test` (oder äquivalent) – alle Tests müssen grün sein.

2. **Linting**
   - `cargo fmt --check` und `cargo clippy` im Heimdall-Verzeichnis (oder im CI-Workflow prüfen).

3. **IMPLEMENTATION_PLAN**
   - Alle von den Teams abgehakten Punkte sind mit [x] markiert und die Beschreibungen passen (keine leeren [x] ohne Inhalt).

4. **IMPLEMENTATION_STATUS.md**
   - Status für die bearbeiteten Phasen (9.5, 9.6, 10, 13, 16, 17, 18) ist aktualisiert.

5. **Kurz-Report**
   - Eine kurze Liste: welche Teams was geliefert haben, welche Dateien neu/geändert sind, und ob es offene Punkte gibt (z. B. „16.2 Metrics nur Stub“, „18.x nur Skelette“).

---

## Reihenfolge (Empfehlung)

1. Team Bifrost  
2. Team Caching  
3. Team Mesh  
4. Team Infra  
5. Team Docs  
6. Team QA  
7. Manager-Check  

Teams können parallel arbeiten, sofern sie unterschiedliche Dateien/Module anfassen. Bei Konflikten (z. B. gleiche `mod.rs`) nacheinander oder Absprache.

---

## Hinweise für Agent/Teams

- Immer **TDD**: Tests zuerst, dann Implementierung.
- **Container-Tests**: Tests so anlegen, dass sie mit `DATABASE_URL` und `docker compose -f docker-compose.test.yml run --rm heimdall-test` laufen.
- **AGENTS.md / README**: Keine unnötigen Änderungen; nur wenn neue Module oder Konventionen dokumentiert werden müssen.
- **KISS**: Kleine, klare Lösungen; keine Überkomplexität.
