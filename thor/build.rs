fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(
            &["proto/thor.proto", "proto/cross_device.proto", "proto/jotunheim.proto"],
            &["proto"],
        )?;

    Ok(())
}
