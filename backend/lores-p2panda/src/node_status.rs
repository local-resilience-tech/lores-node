use std::collections::HashMap;
use std::sync::Arc;

use p2panda_core::Topic;
use tokio::sync::RwLock;

use crate::topic_status::TopicStatus;

/// Aggregates the connection status for every topic this node has subscribed to.
#[derive(Debug, Default)]
pub struct NodeStatus {
    topics: HashMap<Topic, Arc<RwLock<TopicStatus>>>,
}

impl NodeStatus {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new topic and return its shared [`TopicStatus`]. Called by
    /// [`crate::PandaNode`] when a subscription is created.
    pub(crate) fn register_topic(&mut self, topic: Topic) -> Arc<RwLock<TopicStatus>> {
        let status = Arc::new(RwLock::new(TopicStatus::new()));
        self.topics.insert(topic, status.clone());
        status
    }

    /// Returns the [`TopicStatus`] for a given topic, or `None` if not subscribed.
    pub fn get_topic(&self, topic: &Topic) -> Option<Arc<RwLock<TopicStatus>>> {
        self.topics.get(topic).cloned()
    }

    /// Returns all tracked topics and their statuses.
    pub fn topics(&self) -> &HashMap<Topic, Arc<RwLock<TopicStatus>>> {
        &self.topics
    }
}
