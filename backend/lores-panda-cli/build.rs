fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../lores-p2panda-server/proto/panda.proto")?;
    Ok(())
}
