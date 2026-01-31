# Konsistenz-Analyse: Vergleich aller Edda-Implementations

**Erstellt**: 2026-01-27  
**Basis**: Plan "Vollständige Implementations für alle Edda-Projekte", IMPLEMENTATION_COMPLETION_ANALYSIS.md, Codebase-Scan

## 1. Übersicht der untersuchten Bereiche

- gRPC/Protobuf-Konsistenz
- Error-Handling und Logging
- Konfiguration und Settings
- Test-Infrastruktur (Container, Struktur)
- Docker/Container-Setup
- Projektstruktur (proto, src, tests)

---

## 2. gRPC- und Protobuf-Konsistenz

### 2.1 Dependencies (tonic, prost)

| Aspekt | Befund |
|--------|--------|
| **tonic** | Einheitlich **0.11** in allen Rust-Services und -Platforms, die gRPC nutzen |
| **prost** | Einheitlich **0.12** |
| **tonic-build** | **0.11** bei Services mit gRPC; **prost-build 0.12** nur bei Ratatoskr (kein tonic) |
| **prost-types** | Thor, Heimdall, Bifrost, Odin (teilweise) – uneinheitlich, wo explizit als Dependency geführt |

**Unstimmigkeit**: Bifrost führt `tonic-build = "0.11"` in `[build-dependencies]`, hat aber **kein build.rs** und keinen gRPC-Code in `src/`. Entweder tote Dependency oder geplante spätere Nutzung.

### 2.2 Proto-Dateien und build.rs

| Projekt | Proto-Pfad | build.rs-Verhalten |
|---------|------------|--------------------|
| Heimdall | `proto/` (Root) | 5 separate `tonic_build::compile()`-Aufrufe pro Proto |
| Odin | `proto/`, `proto/services/` | Mehrere Aufrufe, teilweise mit `Path::exists()` |
| Freki, Geri, Skuld, Mimir, Nornen, Njörðr, Heidrun, Eikthyrnir, Forseti, Nidhöggr, Vedrfolnir, Læraðr, Hirtir | `proto/` (Root) | Ein Aufruf pro Projekt |
| **Gladsheim** | **src/proto/** | build.rs verweist auf **proto/gladsheim.proto** (Root) – **Inkonsistenz**: Datei liegt unter `src/proto/`, Build erwartet `proto/` am Projektroot |
| Loki, Huginn-Muninn | Workspace: jeweiliges Crate-Verzeichnis mit `proto/` | Crate-spezifisch |
| Bifrost | – | Kein build.rs, kein Proto – reines WebSocket-Service |
| Ratatoskr | `proto/` | Nutzt **prost-build** (kein tonic), nur Serialization |

**Unstimmigkeiten**:

1. **Gladsheim**: `build.rs` und `Dockerfile.test` erwarten `proto/` am Projektroot; tatsächlich liegt nur `src/proto/gladsheim.proto` vor. Build und Docker-Copy sind fehleranfällig bzw. falsch.
2. **Proto-Lage**: Fast überall `proto/` am Root; Gladsheim weicht mit `src/proto/` ab.
3. **Build-Pattern**: Heimdall kompiliert jede Proto einzeln; andere Projekte oft ein Aufruf – funktional ok, aber unterschiedlich.

### 2.3 Service- und Methoden-Namen

- Kein einheitliches Schema für Package-Namen (z. B. `package heimdall.v1;` vs. `package gladsheim.v1;`) in den gescannten Protos; Benennung wirkt konventionell, aber nicht zentral dokumentiert.

---

## 3. Error-Handling und Logging

### 3.1 Error-Libraries

| Bibliothek | Verwendung |
|------------|------------|
| **thiserror** | Durchgängig in Haupt-Crates (Services, Platforms, Plugins) |
| **anyhow** | In fast allen Haupt-Crates zusammen mit thiserror; in Test-Mocks oft nur anyhow |
| **Jotunheim** | Nur thiserror, kein anyhow (IoT/ressourcenschonend – nachvollziehbar) |
| **Læraðr/Hirtir** | Workspace: Shared-/Sub-Crates haben oft nur thiserror; Root-Crate anyhow + thiserror |

**Befund**: Konsistent für „Service/App“-Ebene (thiserror + anyhow). Leichte Abweichung bei Jotunheim und bei Workspace-Shared-Crates.

### 3.2 Logging (tracing)

| Aspekt | Befund |
|--------|--------|
| **tracing** | Durchgängig **0.1** |
| **tracing-subscriber** | **0.3**, Feature **env-filter** überall |
| **Feature "json"** | In Haupt-Crates meist gesetzt; in Test-Mocks häufig nur **env-filter** (weniger Dependencies) |

**Befund**: Einheitliches tracing-Setup; bewusste Reduktion in Mocks.

---

## 4. Konfiguration und Settings

### 4.1 Config-/Settings-Pfade und Formate

- **JSON** wird durchgängig für Konfigurationsdateien verwendet (z. B. `*.json.example` in config/ oder Projektroot).
- Viele Services haben `src/utils/config.rs` oder `src/utils/mod.rs` mit Config-Structs und Laden aus Datei.
- **Mimir** hat ein ausgeprägtes **SettingsManager**-Pattern mit:
  - `Arc<RwLock<MimirSettings>>`
  - **notify** für File-Watcher (Hot-Reload)
  - Validierung und eigene `SettingsError` (thiserror).

### 4.2 Hot-Reload und File-Watcher

- **notify** wird in mehreren Projekten genutzt (u. a. Mimir, Bifrost, Nidhöggr, Vedrfolnir).
- Nicht in allen Services, die „Settings“ oder „Hot-Reload“ im Plan haben, ist ein Watcher sichtbar implementiert – d. h. Unterschied zwischen „geplant“ und „vorhanden“.

**Unstimmigkeit**: Kein dokumentierter Standard, wann Hot-Reload verpflichtend ist; Umsetzung ist von Projekt zu Projekt unterschiedlich.

### 4.3 Naming und Struktur

- Bezeichnungen wie `Settings`, `Config`, `SettingsManager`, `config_path` kommen vor; keine zentrale Nomenklatur (z. B. immer „Settings“ vs. „Config“).

---

## 5. Test-Infrastruktur

### 5.1 Container-Basis

- **docker-compose.test.yml** und **Dockerfile.test** existieren in fast allen Rust-Projekten, die im Plan als „mit Container-Tests“ beschrieben sind.
- **Alfheim** hat `docker-compose.test.yml`; Yggdrasil nutzt eigenes Docker-Setup (Elixir).

### 5.2 Test-Verzeichnisstruktur

| Typ | Beispiele |
|-----|-----------|
| **tests/utils/** | mod.rs, test_helpers.rs – sehr verbreitet |
| **tests/mocks/** | Viele Services (Odin, Skuld, Mimir, Bifrost, Geri, Freki, Huginn-Muninn, Loki, Nidhöggr, Platforms, Valkyries, Frigg) mit Cargo.toml + Dockerfile.mock-service + src/main.rs |
| **tests/integration/** | Odin, Thor, Nidhöggr, Vedrfolnir, Ratatoskr, … |
| **tests/unit/** | Thor, Heimdall, Mimir, Nornen, … |

**Befund**: Ein klares, gemeinsames Pattern (utils, mocks, integration, unit) gibt es nicht projektübergreifend; die Bausteine (utils, mocks, integration, unit) sind aber weit verbreitet.

### 5.3 Docker-Kontext und -Pfade

| Projekt | docker-compose.test.yml – Build-Kontext |
|---------|----------------------------------------|
| Thor, Odin, Freki, Geri, … | `context: .`, `dockerfile: Dockerfile.test` |
| **Nidhöggr** | `context: ..`, `dockerfile: nidhoggr/Dockerfile.test` – Build aus Edda-Root (wegen Ratatoskr-Pfadabhängigkeit) |
| **Nidhöggr** Volumes | `./target`, `./tests`, `./src` werden unter `/app/nidhoggr/...` gemountet |

**Unstimmigkeit**: Nidhöggr weicht bei Kontext und Pfaden bewusst ab (Ratatoskr); andere Projekte nutzen „alles lokal unter Projektroot“. Das kann zu Missverständnissen führen („immer context: . “ vs. „bei Path-Deps context: .. “).

---

## 6. Projektstruktur

### 6.1 Erwartete Basis-Struktur (laut Plan)

- `src/main.rs`, `src/lib.rs`
- `src/proto/` oder `proto/` für .proto
- `src/grpc/`, `src/service/` bzw. domänenspezifische Module
- `src/utils/` oder `config.rs`
- `tests/` mit integration/ und unit/

### 6.2 Abweichungen

| Projekt | Abweichung |
|---------|------------|
| **Gladsheim** | Proto unter `src/proto/`, build.rs und Dockerfile verweisen auf `proto/` am Root |
| **Bifrost** | Kein build.rs, keine Proto; WebSocket-Fokus – strukturiert anders als typischer gRPC-Service |
| **Workspaces (Læraðr, Hirtir, Huginn-Muninn, Loki, Jotunheim)** | Mehrere Crates, eigene proto/ pro Crate oder shared – strukturell anders als Einzel-Service |
| **Ratatoskr** | Protokoll-Bibliothek, kein klassischer Service: `protocol/`, `proto/`, kaum grpc/service |

**Befund**: Die meisten Abweichungen sind domänenbedingt (WebSocket, Workspace, Library). Einzig Gladsheim hat einen klaren Fehler (Proto-Pfad vs. build/Docker).

---

## 7. Identifizierte Unstimmigkeiten – Kurzliste

1. **Gladsheim Proto-Pfad**: Datei in `src/proto/`, build.rs und Dockerfile nutzen `proto/` – Build/Docker passen nicht zur tatsächlichen Struktur.
2. **Bifrost**: tonic-build als build-dependency ohne build.rs und ohne gRPC in src – unklar ob beabsichtigt.
3. **Proto-Build-Pattern**: Verschiedene Strategien (ein Aufruf vs. viele, mit/ohne exists()-Prüfung) – technisch ok, aber nicht einheitlich.
4. **Docker-Build-Kontext**: Üblich `context: .`; Nidhöggr (und ggf. Vedrfolnir) mit `context: ..` – klare Ausnahme, die dokumentiert werden sollte.
5. **Settings/Hot-Reload**: Kein gemeinsamer Standard, wo Hot-Reload verpflichtend ist; Nutzung von notify uneinheitlich.
6. **Test-Struktur**: tests/utils, tests/mocks, tests/integration, tests/unit sind „typisch“, aber keine verbindliche Konvention.
7. **IMPLEMENTATION_PLANs vs. Code**: Der Plan und frühere Analysen weisen darauf hin, dass IMPLEMENTATION_PLANs nicht immer mit dem Implementationsstand übereinstimmen – Status in Plänen kann veraltet sein.

---

## 8. Empfehlungen für die Konsistenz-Analyse (Folgearbeit)

- Gladsheim: Entweder `src/proto/` nach `proto/` verschieben und bestehendes build/Docker beibehalten, oder build.rs und Dockerfile auf `src/proto/` umstellen und einheitlich „proto unter src“ einführen.
- Bifrost: Entweder build.rs und Proto einführen, falls gRPC geplant ist, oder tonic-build aus den build-dependencies entfernen.
- Ein gemeinsames, kurzes Dokument zu „Proto-Platzierung (proto/ vs. src/proto/)“ und „Docker-Kontext bei Path-Dependencies“ würde zukünftige Implementierungen vereinheitlichen.
- Optional: Ein Minimal-Standard für „Settings mit Hot-Reload“ (z. B. „Alle Infra-Services nutzen notify + Arc<RwLock<…>>“) würde die Lücke zwischen Plan und Code schließen.

Diese Analyse dient als Grundlage für die **Dokumentation der Unstimmigkeiten und Standardisierungs-Vorschläge** (separates Dokument).
