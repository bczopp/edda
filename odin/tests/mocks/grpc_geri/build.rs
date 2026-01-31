fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
    // In Docker we're at /app/grpc_geri, proto is at /app/proto
    let proto_base = manifest.join("../proto");
    if !proto_base.join("einherjar.proto").exists() {
        let local = manifest.join("proto");
        if local.join("einherjar.proto").exists() {
            compile_protos(&local)?;
            return Ok(());
        }
        panic!("proto dir not found: run from odin root (Docker build copies proto to /app/proto) or copy odin/proto into tests/mocks/grpc_geri/proto");
    }
    compile_protos(&proto_base)?;
    Ok(())
}

fn compile_protos(proto_base: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                proto_base.join("einherjar.proto"),
                proto_base.join("responsibility.proto"),
                proto_base.join("services").join("geri.proto"),
            ],
            &[proto_base.to_path_buf(), proto_base.join("services")],
        )?;
    Ok(())
}
