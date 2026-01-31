fn main() {
    prost_build::Config::new()
        .compile_protos(&["proto/ratatoskr.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
}
