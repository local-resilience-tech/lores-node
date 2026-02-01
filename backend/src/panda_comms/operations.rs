use p2panda_core::{Extension, Header, Operation, PruneFlag};
use p2panda_net::TopicId;
use serde::{Deserialize, Serialize};

use crate::panda_comms::{operation_store::LOG_ID, topic::LogId};

/// Custom extensions for p2panda header.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LoResMeshExtensions {
    /// If flag is true we can remove all previous operations in this log.
    ///
    /// This usually indicates that a "snapshot" has been inserted into the body of this operation,
    /// containing all required state to reconstruct the full version including all previous edits
    /// of this topic.
    #[serde(
        rename = "p",
        skip_serializing_if = "PruneFlag::is_not_set",
        default = "PruneFlag::default"
    )]
    pub prune_flag: PruneFlag,

    /// Identifier of the topic this operation relates to.
    #[serde(rename = "d")]
    pub topic: TopicId,
}

impl Extension<PruneFlag> for LoResMeshExtensions {
    fn extract(header: &Header<Self>) -> Option<PruneFlag> {
        Some(header.extensions.prune_flag.clone())
    }
}

impl Extension<TopicId> for LoResMeshExtensions {
    fn extract(header: &Header<Self>) -> Option<TopicId> {
        Some(header.extensions.topic)
    }
}

impl Extension<LogId> for LoResMeshExtensions {
    fn extract(_header: &Header<Self>) -> Option<LogId> {
        Some(LOG_ID)
    }
}

pub type LoresOperation = Operation<LoResMeshExtensions>;
