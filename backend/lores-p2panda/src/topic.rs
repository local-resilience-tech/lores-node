use p2panda_core::PublicKey;
use p2panda_net::{NodeId, TopicId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash as StdHash;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::operations::LogType;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, StdHash, Serialize, Deserialize)]
pub struct LogId(LogType, TopicId);

impl LogId {
    pub fn new(log_type: LogType, topic: &TopicId) -> Self {
        Self(log_type, *topic)
    }
}

pub type Logs<L> = HashMap<PublicKey, Vec<L>>;

#[derive(Clone, Default, Debug)]
pub struct LoResNodeTopicMap(Arc<RwLock<HashMap<TopicId, Logs<LogId>>>>);

impl LoResNodeTopicMap {
    pub async fn insert(&self, topic_id: TopicId, node_id: NodeId, log_id: LogId) {
        let mut map = self.0.write().await;
        map.entry(topic_id)
            .and_modify(|logs| {
                logs.insert(node_id, vec![log_id.clone()]);
            })
            .or_insert({
                let mut value = HashMap::new();
                value.insert(node_id, vec![log_id.clone()]);
                value
            });
    }
}
