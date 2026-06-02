fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("../lores-p2panda-client/proto/panda.proto")?;
    Ok(())
}
