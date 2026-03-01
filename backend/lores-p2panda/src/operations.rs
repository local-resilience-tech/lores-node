use p2panda_core::{Extension, Header, Operation, PruneFlag};
use p2panda_net::TopicId;
use serde::{Deserialize, Serialize};
use std::hash::Hash as StdHash;

use super::topic::LogId;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, StdHash, Serialize, Deserialize)]
pub enum LogType {
    #[default]
    Admin,
}

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

    /// We may want multiple logs per author per subject in order to prioritise messages.
    #[serde(rename = "t")]
    pub log_type: LogType,

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

impl Extension<LogType> for LoResMeshExtensions {
    fn extract(header: &Header<Self>) -> Option<LogType> {
        Some(header.extensions.log_type)
    }
}

impl Extension<LogId> for LoResMeshExtensions {
    fn extract(header: &Header<Self>) -> Option<LogId> {
        let log_type: LogType = header.extension()?;
        let id: TopicId = header.extension()?;

        Some(LogId::new(log_type, &id))
    }
}

pub type LoresOperation = Operation<LoResMeshExtensions>;
