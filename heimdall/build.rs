fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["proto/authentication.proto"],
            &["proto"],
        )?;
    
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["proto/authorization.proto"],
            &["proto"],
        )?;
    
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["proto/token.proto"],
            &["proto"],
        )?;
    
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["proto/bifrost_validation.proto"],
            &["proto"],
        )?;
    
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["proto/mesh_membership.proto"],
            &["proto"],
        )?;
    
    Ok(())
}
