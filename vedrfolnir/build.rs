fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build Vedrfolnir proto
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/vedrfolnir.proto"], &["proto/"])?;
    
    // Build Heimdall proto (for client)
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &[
                "proto/heimdall/authentication.proto",
                "proto/heimdall/token.proto",
            ],
            &["proto/"],
        )?;
    
    Ok(())
}
