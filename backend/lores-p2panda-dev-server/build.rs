fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["../lores-p2panda-client/proto/panda.proto"], &["../lores-p2panda-client/proto"])
        .unwrap();
}
