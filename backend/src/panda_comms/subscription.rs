use p2panda_core::Operation;
use p2panda_net::{sync::SyncHandle, TopicId};
use p2panda_sync::protocols::TopicLogSyncEvent;
use thiserror::Error;

use super::{
    network::{LogSync, LogSyncError},
    operations::LoResMeshExtensions,
};

#[derive(Error, Debug)]
pub enum SubscriptionError {
    #[error(transparent)]
    LogSyncError(#[from] LogSyncError),
}

pub struct Subscription {
    topic_id: TopicId,
    sync_tx: SyncHandle<Operation<LoResMeshExtensions>, TopicLogSyncEvent<LoResMeshExtensions>>,
}

impl Subscription {
    pub async fn new(topic_id: TopicId, log_sync: &LogSync) -> Result<Self, SubscriptionError> {
        let sync_tx = log_sync.stream(topic_id, true).await?;

        Ok(Subscription { topic_id, sync_tx })
    }
}
