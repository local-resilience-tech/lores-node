fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=../lores-p2panda-client/proto/panda.proto");
    tonic_prost_build::compile_protos("../lores-p2panda-client/proto/panda.proto")?;
    Ok(())
}
