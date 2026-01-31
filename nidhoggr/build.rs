fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build proto files from other services
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &[
                "../nornen/proto/nornen.proto",
                "../heidrun/proto/heidrun.proto",
                "../mimir/proto/mimir.proto",
            ],
            &["../nornen/proto", "../heidrun/proto", "../mimir/proto"],
        )?;
    
    // Build Nidhöggr's own proto
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&["proto/nidhoggr.proto"], &["proto"])?;
    
    Ok(())
}
