// Code-Generierungs-Pipeline: Protobuf → Rust (Phase 2).
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = std::path::Path::new("proto");
    if !proto_dir.exists() {
        return Ok(());
    }
    // Jotunheim capability protocol (client-only)
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/jotunheim_capability.proto"], &["proto"])?;
    // Loki service (client-only for Jotunheim)
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/loki.proto"], &["proto"])?;
    Ok(())
}
