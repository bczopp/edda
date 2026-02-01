fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build Mimir proto (server)
    // Note: Build context is project root, so proto is at ../../proto from tests/mocks
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["../../proto/mimir/mimir.proto"], &["../../proto"])?;
    
    Ok(())
}
