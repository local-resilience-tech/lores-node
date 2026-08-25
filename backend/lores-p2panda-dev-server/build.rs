fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["proto/panda.proto"], &["proto"])
        .unwrap();
}
