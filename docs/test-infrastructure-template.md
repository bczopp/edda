# Test Infrastructure Template

Dieses Template beschreibt das Standard-Pattern für Container-basierte Test-Infrastruktur für alle Edda-Projekte.

## Struktur

Jedes Projekt muss folgende Dateien haben:

1. `Dockerfile.test` - Container für Test-Ausführung
2. `docker-compose.test.yml` - Docker Compose Setup mit allen Dependencies
3. `tests/` - Test-Verzeichnis mit Unit- und Integration-Tests
4. `tests/utils/` - Test-Utilities und Helpers (mod.rs, test_helpers.rs), wo vorhanden
5. `tests/mocks/` - Eigene Mocks für externe Services (inkl. Cargo.toml, Dockerfile.mock-service, src/main.rs), wenn Integrationstests gegen Mock-Services laufen
6. `tests/integration/` bzw. `tests/unit/` - Bei Nutzung: Integrationstests unter integration/, Unit-Tests unter unit/ oder direkt unter tests/
7. **Optional:** `scripts/run-tests.sh` und `scripts/run-tests.ps1` – Skripte, die ins Projektverzeichnis wechseln und `docker compose -f docker-compose.test.yml run --rm <service>-test` ausführen (ermöglicht Ausführung von Repo-Root oder Projektverzeichnis; alle CI-Services mit docker-compose.test.yml haben diese Skripte).

Proto-Dateien und build.rs richten sich nach [docs/proto-and-build-conventions.md](proto-and-build-conventions.md) (Proto-Root: `proto/` am Projektroot).

## Dockerfile.test Template

**Sprachversionen:** Es werden die aktuellsten stabilen Versionen verwendet (Rust: `rust:latest`, in CI `toolchain: stable`; Elixir: `elixir:latest`). **JavaScript/TypeScript:** Es wird **bun** verwendet (Docker: `oven/bun:latest`), nicht Node/npm – siehe [TECHNOLOGY_DECISIONS.md](TECHNOLOGY_DECISIONS.md).

```dockerfile
FROM rust:latest

WORKDIR /app

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Copy source code
COPY src ./src
COPY proto ./proto
COPY tests ./tests

# Build the project
RUN cargo build --release

# Default command for tests
CMD ["cargo", "test", "--release"]
```

## docker-compose.test.yml Template

```yaml
version: '3.8'

services:
  # Add service-specific dependencies here (databases, mock services, etc.)
  
  <project-name>-test:
    build:
      context: .
      dockerfile: Dockerfile.test
    depends_on:
      # List dependencies here
    environment:
      RUST_LOG: debug
      # Add service-specific environment variables
    volumes:
      - ./target:/app/target
      - ./tests:/app/tests
      - ./src:/app/src
    command: cargo test --release
```

### Docker-Build-Kontext

- **Standard:** `context: .`, `dockerfile: Dockerfile.test`, Volumes relativ zum Projektroot. Das Projekt liegt im Container unter `/app/`.
- **Ausnahme (Path-Dependencies):** Projekte mit **Path-Dependencies** auf andere Edda-Crates (z. B. `path = "../ratatoskr"` in der Cargo.toml) müssen von der **Edda-Repo-Root** bauen: `context: ..`, `dockerfile: <projekt>/Dockerfile.test`. Volume-Pfade so wählen, dass das Projekt unter `/app/<projekt>/` liegt (z. B. `./target:/app/<projekt>/target`, `./src:/app/<projekt>/src`).
- **Referenz:** Nidhöggr nutzt diese Ausnahme (`context: ..`, `dockerfile: nidhoggr/Dockerfile.test`) wegen der Ratatoskr-Pfadabhängigkeit; siehe `nidhoggr/docker-compose.test.yml`.

## Test-Utilities Template

Jedes Projekt sollte `tests/utils/test_helpers.rs` haben:

```rust
use std::time::Duration;
use tokio::time::sleep;

/// Wait for a service to be ready
pub async fn wait_for_service(url: &str, max_retries: u32) -> bool {
    for _ in 0..max_retries {
        if let Ok(_) = tokio::net::TcpStream::connect(url).await {
            return true;
        }
        sleep(Duration::from_millis(500)).await;
    }
    false
}

/// Get service URL from environment or use default
pub fn get_service_url(service_name: &str, default_port: u16) -> String {
    let env_var = format!("{}_URL", service_name.to_uppercase());
    std::env::var(&env_var).unwrap_or_else(|_| {
        format!("http://localhost:{}", default_port)
    })
}
```

## TDD-Prinzipien

1. **Tests zuerst**: Alle Tests werden VOR der Implementierung geschrieben
2. **Red-Green-Refactor**: 
   - Red: Tests schreiben, Tests schlagen fehl
   - Green: Minimale Implementierung, Tests bestehen
   - Refactor: Code verbessern, Tests bleiben grün
3. **Container-basiert**: Alle Tests müssen in Containern laufen
4. **Keine lokalen Dependencies**: Keine Tools oder Services auf der Entwicklungsmaschine installieren

## Service-spezifische Anpassungen

### Für Services mit Datenbanken (z.B. Mimir, Njörðr, Heidrun)
- PostgreSQL-Container hinzufügen
- Health-Checks für Datenbank
- Migrations in Container ausführen

### Für Services mit externen Dependencies (z.B. Freki mit Qdrant)
- Qdrant-Container hinzufügen
- Redis-Container falls benötigt

### Für Services mit Mock-Services (z.B. Odin)
- Mock-Service-Container hinzufügen
- Health-Checks für Mock-Services

## Verwendung

1. Kopiere `Dockerfile.test` Template in Projekt
2. Passe `docker-compose.test.yml` an (Dependencies hinzufügen)
3. Erstelle `tests/utils/test_helpers.rs`
4. Schreibe Tests nach TDD-Prinzipien
5. Führe Tests aus: `docker compose -f docker-compose.test.yml run --rm <service>-test` (oder `up --build` je nach Setup)

### Beispiel: Bifrost (Phase 20 Test Suites)

Bifrost nutzt dedizierte Test-Dateien für E2E, Performance, Security und GDPR (IMPLEMENTATION_PLAN Phase 20):

- `tests/e2e_communication_workflow_test.rs` – E2E Communication Workflows
- `tests/error_recovery_test.rs` – Error Recovery
- `tests/performance_benchmark_test.rs` – Performance Benchmarks
- `tests/security_test_suite.rs` – Security Test Suite
- `tests/gdpr_compliance_test.rs` – GDPR Compliance

Ausführung von `bifrost/`: `docker compose -f docker-compose.test.yml run --rm bifrost-test`. Siehe [bifrost/README.md](../bifrost/README.md) (Phase 20 Test Suites) und [bifrost/scripts/run-tests.sh](../bifrost/scripts/run-tests.sh).

**E2E WebSocket-Routing:** Wenn Test-Clients ohne Handshake-Device-ID verbinden (alle als „unknown“ registriert), müssen Antwort-Nachrichten im E2E-Test mit `target_device_id: "unknown"` gesendet werden, damit der Server an alle verbundenen Clients routet (z. B. gRPC-over-Bifrost E2E).

## CI/CD

Services mit GitHub Actions CI (Test im Container, Lint; bei Push/PR auf den jeweiligen Pfad):

- **Bifrost**: [.github/workflows/bifrost.yml](../.github/workflows/bifrost.yml) – `bifrost/**` (Test, Lint, Coverage, Rustdoc, cargo-audit; Test-Job-Timeout 15 min)
- **Heimdall**: [.github/workflows/heimdall.yml](../.github/workflows/heimdall.yml) – `heimdall/**` (Test, Lint, Coverage; Test-Job-Timeout 15 min)
- **Thor**: [.github/workflows/thor.yml](../.github/workflows/thor.yml) – `thor/**` (Test, Lint, Coverage; Test-Job-Timeout 15 min)
- **Odin**: [.github/workflows/odin.yml](../.github/workflows/odin.yml) – `odin/**` (Test, Lint, Coverage; Test-Job-Timeout 15 min)
- **Loki**: [.github/workflows/loki.yml](../.github/workflows/loki.yml) – `loki/**` (Test, Lint, Coverage; Test-Job-Timeout 15 min)
- **Freki**: [.github/workflows/freki.yml](../.github/workflows/freki.yml) – `freki/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Geri**: [.github/workflows/geri.yml](../.github/workflows/geri.yml) – `geri/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Skuld**: [.github/workflows/skuld.yml](../.github/workflows/skuld.yml) – `skuld/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Mimir**: [.github/workflows/mimir.yml](../.github/workflows/mimir.yml) – `mimir/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Huginn-Muninn**: [.github/workflows/huginn-muninn.yml](../.github/workflows/huginn-muninn.yml) – `huginn-muninn/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Nornen**: [.github/workflows/nornen.yml](../.github/workflows/nornen.yml) – `nornen/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Vedrfolnir**: [.github/workflows/vedrfolnir.yml](../.github/workflows/vedrfolnir.yml) – `vedrfolnir/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Valkyries**: [.github/workflows/valkyries.yml](../.github/workflows/valkyries.yml) – `valkyries/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Nidhöggr**: [.github/workflows/nidhoggr.yml](../.github/workflows/nidhoggr.yml) – `nidhoggr/**` (Test, Lint; Build-Kontext Repo-Root; Test-Job-Timeout 15 min)
- **Frigg**: [.github/workflows/frigg.yml](../.github/workflows/frigg.yml) – `frigg/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Ragnarok**: [.github/workflows/ragnarok.yml](../.github/workflows/ragnarok.yml) – `ragnarok/**` (Test, Lint, Coverage; Test-Job-Timeout 15 min)
- **Njörðr**: [.github/workflows/njordr.yml](../.github/workflows/njordr.yml) – `njordr/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Heidrun**: [.github/workflows/heidrun.yml](../.github/workflows/heidrun.yml) – `heidrun/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Ratatoskr**: [.github/workflows/ratatoskr.yml](../.github/workflows/ratatoskr.yml) – `ratatoskr/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Eikthyrnir**: [.github/workflows/eikthyrnir.yml](../.github/workflows/eikthyrnir.yml) – `eikthyrnir/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Læraðr**: [.github/workflows/laeradr.yml](../.github/workflows/laeradr.yml) – `laeradr/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Jotunheim**: [.github/workflows/jotunheim.yml](../.github/workflows/jotunheim.yml) – `jotunheim/**` (Test, Lint, Coverage; Test-Job-Timeout 15 min)
- **Asgard**: [.github/workflows/asgard.yml](../.github/workflows/asgard.yml) – `asgard/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Forseti**: [.github/workflows/forseti.yml](../.github/workflows/forseti.yml) – `forseti/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Midgard**: [.github/workflows/midgard.yml](../.github/workflows/midgard.yml) – `midgard/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Hirtir**: [.github/workflows/hirtir.yml](../.github/workflows/hirtir.yml) – `hirtir/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Gladsheim**: [.github/workflows/gladsheim.yml](../.github/workflows/gladsheim.yml) – `gladsheim/**` (Test, Lint; Test-Job-Timeout 15 min)
- **Alfheim**: [.github/workflows/alfheim.yml](../.github/workflows/alfheim.yml) – `alfheim/**` (nur Test im Container, Bun; Test-Job-Timeout 15 min)
- **Yggdrasil**: [.github/workflows/yggdrasil.yml](../.github/workflows/yggdrasil.yml) – `yggdrasil/**` (nur Test im Container, Elixir; Test-Job-Timeout 15 min)

Weitere Services können nach dem gleichen Muster (Container-Test, optional Timeout) ergänzt werden. Siehe [README#Für Entwickler](../README.md#für-entwickler).
