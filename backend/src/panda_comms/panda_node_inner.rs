use super::panda_node::PandaNodeError;

pub struct PandaNodeInner {}

impl PandaNodeInner {
    pub async fn new() -> Result<Self, PandaNodeError> {
        println!("Initializing PandaNodeInner...");
        Ok(PandaNodeInner {})
    }
}
