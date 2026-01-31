fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build Odin proto
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/odin.proto"], &["proto"])?;
    
    // Build Einherjar Protocol proto
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/einherjar.proto"], &["proto"])?;
    
    // Build Responsibility Service proto
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/responsibility.proto"], &["proto"])?;
    
    // Build service protos for clients
    let thor_proto = "proto/services/thor.proto";
    let freki_proto = "proto/services/freki.proto";
    let geri_proto = "proto/services/geri.proto";
    let skuld_proto = "proto/services/skuld.proto";
    
    if std::path::Path::new(thor_proto).exists() {
        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .compile(&[thor_proto], &["proto/services"])?;
    }
    
    if std::path::Path::new(freki_proto).exists() {
        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .compile(&[freki_proto], &["proto/services"])?;
    }
    
    if std::path::Path::new(geri_proto).exists() {
        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .compile(&[geri_proto], &["proto/services"])?;
    }
    
    if std::path::Path::new(skuld_proto).exists() {
        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .compile(&[skuld_proto], &["proto/services"])?;
    }

    let huginn_muninn_proto = "proto/services/huginn_muninn.proto";
    let loki_proto = "proto/services/loki.proto";
    let heimdall_proto = "proto/services/heimdall.proto";

    if std::path::Path::new(huginn_muninn_proto).exists() {
        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .compile(&[huginn_muninn_proto], &["proto/services"])?;
    }

    if std::path::Path::new(loki_proto).exists() {
        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .compile(&[loki_proto], &["proto/services"])?;
    }

    if std::path::Path::new(heimdall_proto).exists() {
        tonic_build::configure()
            .build_server(false)
            .build_client(true)
            .compile(&[heimdall_proto], &["proto/services"])?;
    }

    Ok(())
}
