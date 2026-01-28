use serde::{Deserialize, Serialize};

/// Custom extensions for p2panda header.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoResMeshExtensions {
    // /// Identifier of the topic this operation relates to.
    // #[serde(rename = "d")]
    // pub topic: TopicId,
}
