use std::sync::Arc;
use std::time::{SystemTime, SystemTimeError};

use p2panda_core::{Body, Hash, Header, Operation, PrivateKey, PublicKey};
use p2panda_net::TopicId;
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
    inner: SqliteStore<'static>,
    // FIXME: This makes sure we only create one operation at the time and not in parallel
    // Since we would mess up the sequence of operations
    semaphore_operation_store: Arc<Semaphore>,
}

impl OperationStore {
    pub async fn new(_pool: sqlx::SqlitePool) -> Result<Self, SqliteError> {
        let database_url = std::env::var("OPERATION_DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:operations.sqlite".to_string());

        let inner = SqliteStoreBuilder::new()
            .database_url(&database_url)
            .create_database(false)
            .run_default_migrations(false)
            .build()
            .await?;

        Ok(Self {
            inner,
            semaphore_operation_store: Arc::new(Semaphore::new(1)),
        })
    }

    pub fn clone_inner(&self) -> SqliteStore<'static> {
        self.inner.clone()
    }

    /// Creates, signs and stores new operation in the author's append-only log.
    pub async fn create_operation(
        &self,
        topic_id: TopicId,
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
        let latest_entry: Option<(Hash, u64)> =
            <SqliteStore<'static> as LogStore<
                Operation<LoResMeshExtensions>,
                PublicKey,
                LogId,
                u64,
                Hash,
            >>::get_latest_entry(&self.inner, &public_key, &log_id)
            .await?;

        let (seq_num, backlink) = match latest_entry {
            Some((hash, seq_num)) => (seq_num + 1, Some(hash)),
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
            timestamp,
            seq_num,
            backlink,
            previous: vec![],
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
            .insert_operation(&operation.hash, operation.clone(), log_id)
            .await?;

        Ok(operation)
    }
}
