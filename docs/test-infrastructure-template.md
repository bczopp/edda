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

Proto-Dateien und build.rs richten sich nach [docs/proto-and-build-conventions.md](proto-and-build-conventions.md) (Proto-Root: `proto/` am Projektroot).

## Dockerfile.test Template

```dockerfile
FROM rust:1.75-slim

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
5. Führe Tests aus: `docker-compose -f docker-compose.test.yml up --build`
