use p2panda_core::PublicKey;
use p2panda_net::{NodeId, TopicId};
use p2panda_sync::traits::TopicMap;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type LogId = u64;

pub type Logs<L> = HashMap<PublicKey, Vec<L>>;

#[derive(Clone, Default, Debug)]
pub struct LoResNodeTopicMap(Arc<RwLock<HashMap<TopicId, Logs<LogId>>>>);

impl LoResNodeTopicMap {
    pub async fn insert(&self, topic_id: TopicId, node_id: NodeId, log_id: LogId) {
        let mut map = self.0.write().await;
        map.entry(topic_id)
            .and_modify(|logs| {
                logs.insert(node_id, vec![log_id]);
            })
            .or_insert({
                let mut value = HashMap::new();
                value.insert(node_id, vec![log_id]);
                value
            });
    }
}

impl TopicMap<TopicId, Logs<LogId>> for LoResNodeTopicMap {
    type Error = Infallible;

    async fn get(&self, topic_query: &TopicId) -> Result<Logs<LogId>, Self::Error> {
        let map = self.0.read().await;
        Ok(map.get(topic_query).cloned().unwrap_or_default())
    }
}
