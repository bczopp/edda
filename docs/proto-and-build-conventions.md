# Proto- und Build-Konventionen (gRPC-Services)

Dieses Dokument beschreibt verbindliche Konventionen für Protobuf-Dateien und den Aufruf von `tonic_build` in Rust-Projekten der Edda-Codebase.

---

## Proto-Platzierung

- **Standard:** Proto-Dateien liegen immer unter `<projektroot>/proto/`.
- **Nicht:** `src/proto/` – das führt zu Inkonsistenzen mit build.rs und Dockerfile.test, die typischerweise `proto/` am Projektroot erwarten.
- **Dockerfile.test:** `COPY proto ./proto` setzt voraus, dass `proto/` am Projektroot existiert.

---

## build.rs-Pattern

### Empfohlen: zentrale Liste, ein compile-Aufruf

Sofern keine unterschiedlichen `build_server`/`build_client`-Profile pro Proto-Datei nötig sind:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protos = &["proto/svc.proto", "proto/other.proto"];
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(protos, &["proto"])?;
    Ok(())
}
```

### Optionale Service-Protos (Client-only)

Wenn ein Projekt optional Proto-Dateien anderer Services einbindet (z. B. für Client-Code), ist `Path::exists()` zulässig. Im build.rs muss kurz kommentiert werden, wofür die optionale Datei dient (z. B. „optional service stubs for client-only code“):

```rust
// Optional service stubs for client-only code (z. B. wenn proto/services/ aus anderem Repo kommt)
let thor_proto = "proto/services/thor.proto";
if std::path::Path::new(thor_proto).exists() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&[thor_proto], &["proto/services"])?;
}
```

### Mehrere Aufrufe pro Proto

Einzelne Projekte (z. B. Heimdall) kompilieren jede Proto in separaten `tonic_build::compile()`-Aufrufen. Das ist funktional in Ordnung; für neue Projekte oder Refactorings ist das obige Ein-Aufruf-Pattern mit zentraler Liste bevorzugt.

---

## Referenzen

- **Test-Infrastruktur:** `docs/test-infrastructure-template.md` – Docker-Build-Kontext, Struktur
- **Technology Decisions:** `docs/TECHNOLOGY_DECISIONS.md` – Protobuf- und gRPC-Entscheidungen
