use std::sync::Arc;
use std::time::{SystemTime, SystemTimeError};

use p2panda_core::{Body, Hash, Header, Operation, PrivateKey, PublicKey, Topic};
use p2panda_store::{
    logs::LogStore, operations::OperationStore as TraitOperationStore, SqliteError, SqliteStore,
    SqliteStoreBuilder,
};
use thiserror::Error;
use tokio::sync::Semaphore;

use crate::operations::LogType;

use super::{operations::LoResMeshExtensions, topic::LogId};

#[derive(Debug, Error)]
pub enum CreationError {
    #[error(transparent)]
    SystemTime(#[from] SystemTimeError),
    #[error(transparent)]
    Store(#[from] SqliteError),
}

#[derive(Debug)]
pub struct OperationStore {
    inner: SqliteStore,
    // FIXME: This makes sure we only create one operation at the time and not in parallel
    // Since we would mess up the sequence of operations
    semaphore_operation_store: Arc<Semaphore>,
}

impl OperationStore {
    pub async fn new(database_url: &str) -> Result<Self, SqliteError> {
        let inner = SqliteStoreBuilder::new()
            .database_url(database_url)
            .create_database(true)
            .run_default_migrations(true)
            .build()
            .await?;

        Ok(Self {
            inner,
            semaphore_operation_store: Arc::new(Semaphore::new(1)),
        })
    }

    pub fn clone_inner(&self) -> SqliteStore {
        self.inner.clone()
    }

    /// Creates, signs and stores new operation in the author's append-only log.
    pub async fn create_operation(
        &self,
        topic_id: Topic,
        log_type: LogType,
        private_key: &PrivateKey,
        body: Option<&[u8]>,
    ) -> Result<Operation<LoResMeshExtensions>, CreationError> {
        let _permit = self
            .semaphore_operation_store
            .acquire()
            .await
            .expect("OperationStore semaphore not to be closed");

        let body = body.map(Body::new);
        let public_key = private_key.public_key();

        let log_id = LogId::new(log_type, &topic_id);
        let latest_entry: Option<Operation<LoResMeshExtensions>> =
            <SqliteStore as LogStore<
                Operation<LoResMeshExtensions>,
                PublicKey,
                LogId,
                u64,
                Hash,
            >>::get_latest_entry(&self.inner, &public_key, &log_id)
            .await?;

        let (seq_num, backlink) = match latest_entry {
            Some(op) => (op.header.seq_num + 1, Some(op.hash)),
            None => (0, None),
        };

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let extensions = LoResMeshExtensions {
            prune_flag: Default::default(),
            topic: topic_id,
            log_type,
        };

        let mut header = Header {
            version: 1,
            public_key,
            signature: None,
            payload_size: body.as_ref().map_or(0, |body| body.size()),
            payload_hash: body.as_ref().map(|body| body.hash()),
            timestamp: timestamp.into(),
            seq_num,
            backlink,
            extensions,
        };
        header.sign(private_key);

        let operation = Operation {
            hash: header.hash(),
            header,
            body,
        };

        let inner_clone = self.clone_inner();
        inner_clone
            .insert_operation(&operation.hash, &operation, &log_id)
            .await?;

        Ok(operation)
    }
}
