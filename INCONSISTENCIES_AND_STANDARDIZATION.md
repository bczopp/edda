# Dokumentation der Unstimmigkeiten und Standardisierungs-Vorschläge

**Erstellt**: 2026-01-27  
**Grundlage**: CONSISTENCY_ANALYSIS.md, Plan „Vollständige Implementations für alle Edda-Projekte“

Dieses Dokument listet die in der Konsistenz-Analyse gefundenen Unstimmigkeiten und schlägt verbindliche Standardisierungen vor.

---

## 1. Gefundene Unstimmigkeiten

### 1.1 Gladsheim: Proto-Pfad vs. Build/Docker

- **Beschreibung**: Proto-Dateien liegen unter `src/proto/gladsheim.proto`, während `build.rs` und `Dockerfile.test` auf `proto/gladsheim.proto` (Projektroot) verweisen.
- **Auswirkung**: Build und Docker-Test-Setup können fehlschlagen oder veralten, sobald nur die eine oder andere Stelle angepasst wird.
- **Quelle**: CONSISTENCY_ANALYSIS.md, Abschnitt 2.2 und 6.2.

### 1.2 Bifrost: tonic-build ohne Nutzung

- **Beschreibung**: Im Bifrost-Cargo.toml steht `tonic-build = "0.11"` unter `[build-dependencies]`, es gibt kein `build.rs` und keinen gRPC-Code in `src/`.
- **Auswirkung**: Tote Dependency oder unklare Absicht; erschwert Verständnis und Wartung.
- **Quelle**: CONSISTENCY_ANALYSIS.md, Abschnitt 2.1 und 2.2.

### 1.3 Unterschiedliche Proto-Build-Strategien

- **Beschreibung**: Heimdall kompiliert jede Proto in separaten `tonic_build::compile()`-Aufrufen; andere Services oft ein Aufruf; Odin nutzt `Path::exists()` für optionale Service-Protos.
- **Auswirkung**: Kein einheitliches Vorgehen beim Hinzufügen neuer Protos oder beim Onboarding.
- **Quelle**: CONSISTENCY_ANALYSIS.md, Abschnitt 2.2.

### 1.4 Docker-Build-Kontext bei Path-Dependencies

- **Beschreibung**: Die meisten Projekte bauen mit `context: .` und `dockerfile: Dockerfile.test`. Nidhöggr (und ggf. Vedrfolnir) nutzen `context: ..` und `dockerfile: <projekt>/Dockerfile.test`, um vom Edda-Root aus zu bauen (z. B. Ratatoskr-Pfadabhängigkeit).
- **Auswirkung**: Ohne Dokumentation wirkt Nidhöggr „falsch“ oder willkürlich; Copy-Paste in andere Projekte mit Path-Deps kann zu Fehlern führen.
- **Quelle**: CONSISTENCY_ANALYSIS.md, Abschnitt 5.3.

### 1.5 Settings und Hot-Reload

- **Beschreibung**: Mimir (und einzelne andere) haben SettingsManager mit notify (Hot-Reload); viele Services haben nur Config-Load ohne Watcher. Es ist nicht festgelegt, wo Hot-Reload Pflicht ist.
- **Auswirkung**: Uneinheitliches Runtime-Verhalten und unterschiedliche Erwartungen in IMPLEMENTATION_PLANs.
- **Quelle**: CONSISTENCY_ANALYSIS.md, Abschnitt 4.

### 1.6 Test-Verzeichnisstruktur

- **Beschreibung**: Es gibt gemeinsame Bausteine (tests/utils, tests/mocks, tests/integration, tests/unit), aber keine verbindliche Konvention, welches Projekt welche Struktur hat.
- **Auswirkung**: Integration von Queries/Tools („wo stehen Integrations-Tests?“) und Wiederverwendung von Helpers ist weniger vorhersagbar.
- **Quelle**: CONSISTENCY_ANALYSIS.md, Abschnitt 5.2.

### 1.7 IMPLEMENTATION_PLANs vs. tatsächlicher Code

- **Beschreibung**: Plan und Konsistenz-Analyse weisen darauf hin, dass viele IMPLEMENTATION_PLANs nicht mit dem Implementationsstand aktualisiert wurden (z. B. Odin: Plan zeigt Phasen unerledigt, obwohl Prototyp funktioniert).
- **Auswirkung**: Status-Tracking und Priorisierung sind fehleranfällig; Doppelarbeit oder falsche „noch offen“-Annahmen.
- **Quelle**: CONSISTENCY_ANALYSIS.md Abschnitt 7.

---

## 2. Standardisierungs-Vorschläge

### 2.1 Proto-Dateien und build.rs (gRPC-Services)

- **Vorschlag**: Ein einheitliches Proto-Root pro Projekt.
  - **Standard A (empfohlen)**: Proto-Dateien liegen immer unter `<projektroot>/proto/`.  
    - `build.rs` ruft `tonic_build::configure().compile(&["proto/<name>.proto"], &["proto"])?` (bzw. mehrere Dateien) auf.  
    - `Dockerfile.test` kopiert mit `COPY proto ./proto`.
  - **Standard B (Alternative)**: Einheitlich `src/proto/` nutzen. Dann müssen build.rs und alle Dockerfiles auf `src/proto/` und passende Include-Pfade umgestellt werden.
- **Konkret**: Gladsheim an Standard A anpassen: Inhalt von `src/proto/` nach `proto/` verschieben, build.rs und Dockerfile.test unverändert lassen (sie folgen schon Standard A).
- **Bifrost**: Wenn kein gRPC geplant ist, `tonic-build` und ggf. `tonic`/`prost` aus Cargo.toml entfernen. Wenn gRPC später kommt: build.rs und proto/ anlegen und an Standard A halten.

### 2.2 Proto-Build-Strategie in build.rs

- **Vorschlag**: Für Services mit mehreren Protos ein einheitliches Pattern:
  - Eine zentrale Liste der Proto-Dateien (z. B. `&["proto/svc.proto", "proto/other.proto"]`).
  - Ein Aufruf `tonic_build::configure().build_server(...).build_client(...).compile(&protos, &["proto"])?`, sofern keine unterschiedlichen build_server/build_client-Profilen pro Datei nötig sind.
  - Optional vorhandene Service-Protos (wie bei Odin) weiterhin mit `Path::exists()` prüfen, aber in einem kurzen Kommentar im build.rs erklären (z. B. „optional service stubs for client-only code“).

### 2.3 Docker-Build-Kontext für Tests

- **Vorschlag**: In einer gemeinsamen Test- bzw. Build-Dokumentation (z. B. in docs/ oder in AGENTS.md/README) festhalten:
  - **Standard**: `context: .`, `dockerfile: Dockerfile.test`, Volumes relativ zum Projektroot.
  - **Ausnahme**: Projekte mit **Path-Dependencies** auf andere Edda-Crates (z. B. `path = "../ratatoskr"`) müssen von der **Edda-Repo-Root** bauen: `context: ..`, `dockerfile: <projekt>/Dockerfile.test`, und Volume-Pfade so wählen, dass das Projekt unter `/app/<projekt>/` liegt.
  - Nidhöggr und Vedrfolnir als Referenzbeispiele nennen.

### 2.4 Error-Handling und Logging

- **Vorschlag**: Unverändert als de-facto-Standard dokumentieren:
  - **Rust-Services/Platforms**: `thiserror` + `anyhow`; bei Libraries/Shared-Crates nur `thiserror` erlaubt.
  - **Logging**: `tracing` 0.1 + `tracing-subscriber` 0.3 mit mindestens Feature `env-filter`; Hauptbinary mit `json`, Mocks ohne `json` zulässig.

### 2.5 Settings und Hot-Reload

- **Vorschlag**: Klarstellung, wo Hot-Reload erwartet wird:
  - **Verpflichtend**: Alle Infrastructure-Services und Core-Services, die in IMPLEMENTATION_PLAN oder AGENTS.md „Hot-Reload“ oder „Runtime-Konfigurationsänderung“ erwähnen.
  - **Technischer Standard**: `notify` + `Arc<RwLock<Settings>>` + Validierung beim Laden; Fehlertyp für Settings-Fehler (z. B. thiserror).
  - **Optional**: Platformen und reine Protokolle (z. B. Ratatoskr); dann reines „Load once“ ausreichend.
  - In IMPLEMENTATION_PLANs und READMEs pro Projekt kurz vermerken: „Hot-Reload: ja/nein“.

### 2.6 Test-Struktur

- **Vorschlag**: Minimale Konvention, ohne bestehende Projekte umzupflastern:
  - **tests/utils/**: Gemeinsame Hilfen (mod.rs, test_helpers.rs), wo vorhanden.
  - **tests/mocks/**: Eigene Mocks für externe Services (inkl. Cargo.toml, Dockerfile.mock-service, src/main.rs), wenn das Projekt Integrationstests gegen Mock-Services ausführt.
  - **tests/integration/** bzw. **tests/unit/**: Wenn verwendet, dann Integrationstests unter integration/, reine Unit-Tests unter unit/ oder direkt unter tests/.
  - In der zentralen Test- oder Entwickler-Doku (docs/ oder AGENTS.md) einen Satz wie: „Alle Projekte verwenden, wo sinnvoll, tests/utils und bei Bedarf tests/mocks; Integration-/Unit-Trennung ist empfohlen.“

### 2.7 IMPLEMENTATION_PLANs aktuell halten

- **Vorschlag**: Keine inhaltliche Standardisierung der Pläne selbst, aber Prozess:
  - Nach Abschluss einer Phase oder eines Meilensteins: IMPLEMENTATION_PLAN des betroffenen Projekts aktualisieren (Checkboxen, „completed“-Vermerke).
  - Optional: In AGENTS.md oder in einer Checkliste festhalten: „Bei Merge/Abnahme: IMPLEMENTATION_PLAN-Status prüfen und anpassen.“

---

## 3. Priorisierung der Umsetzung

| Priorität | Unstimmigkeit / Vorschlag | Aufwand | Nutzen |
|-----------|----------------------------|---------|--------|
| P1 | Gladsheim Proto-Pfad (2.1) | Gering | Build/Docker stabil |
| P1 | Bifrost tonic-build bereinigen (2.1) | Gering | Klarheit |
| P2 | Docker-Kontext dokumentieren (2.3) | Gering | Weniger Fehler bei neuen Projekten |
| P2 | Proto-Build-Strategie dokumentieren (2.2) | Gering | Einheitlichkeit bei Änderungen |
| P3 | Settings/Hot-Reload-Standard (2.5) | Mittel | Einheitliches Runtime-Verhalten |
| P3 | Test-Struktur-Konvention (2.6) | Gering | bessere Auffindbarkeit |
| P3 | IMPLEMENTATION_PLAN-Prozess (2.7) | Prozess | zuverlässiger Status |

---

## 4. Referenzen

- **CONSISTENCY_ANALYSIS.md**: Detaillierter Vergleich und technische Befunde.
- **AGENTS.md (Edda-Root)**: TDD, Container-Tests, Konfiguration, Sicherheit.
- Plan „Vollständige Implementations für alle Edda-Projekte“: Phasen, Struktur, Konsistenz-Checks.

Mit der Umsetzung der P1-Punkte und der Dokumentation (P2/P3) sind die aus der Konsistenz-Analyse abgeleiteten Unstimmigkeiten adressiert und Standardisierungs-Vorschläge nachverfolgbar.
