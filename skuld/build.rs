fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Skuld: Server; Eikthyrnir: Client (Quality-Daten). Beide Protos in einem Durchlauf.
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&["proto/skuld.proto", "proto/eikthyrnir.proto"], &["proto"])?;
    Ok(())
}
