fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build Nornen proto (server)
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["proto/nornen.proto"], &["proto"])?;
    
    // Build Mimir proto (client)
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/mimir/mimir.proto"], &["proto"])?;
    
    Ok(())
}
