use std::pin::Pin;

use futures::{StreamExt, stream};
use lores_p2panda_client::SubscriptionFrom;
use sqlx::SqlitePool;

use crate::transports::{OperationStream, OperationTransport, RawEvent, RawOperationEvent, TransportError, TransportPublishResult};

/// [`OperationTransport`] implementation backed by a local SQLite database.
///
/// Operations are persisted in insertion order. This transport is the foundation
/// for offline operation and the outgoing queue that a future drain task will
/// deliver to lores-node.
pub(crate) struct LocalTransport {
    pool: SqlitePool,
}

impl LocalTransport {
    pub(crate) async fn new(pool: SqlitePool) -> Result<Self, sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS lores_app_operations (
                id         INTEGER PRIMARY KEY AUTOINCREMENT,
                payload    BLOB    NOT NULL,
                created_at INTEGER NOT NULL DEFAULT (unixepoch())
            )",
        )
        .execute(&pool)
        .await?;
        Ok(Self { pool })
    }

    /// Insert a payload and return the assigned id (used as idempotency key).
    pub(crate) async fn insert(&self, payload: Vec<u8>) -> Result<i64, sqlx::Error> {
        let result = sqlx::query("INSERT INTO lores_app_operations (payload) VALUES (?)")
            .bind(payload)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    /// Remove an entry by id after successful delivery.
    pub(crate) async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM lores_app_operations WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

impl OperationTransport for LocalTransport {
    fn publish(
        &mut self,
        payload: Vec<u8>,
        _idempotency_key: Option<String>,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<TransportPublishResult, TransportError>> + Send + '_>> {
        Box::pin(async move {
            self.insert(payload)
                .await
                .map(|_| TransportPublishResult {
                    operation_id: None,
                    node_id: None,
                })
                .map_err(|e| TransportError::Other(e.to_string()))
        })
    }

    fn subscribe(
        &mut self,
        _start_from: SubscriptionFrom,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<OperationStream, TransportError>> + Send + '_>> {
        Box::pin(async move {
            let s: OperationStream = Box::pin(stream::pending());
            Ok(s)
        })
    }

    fn replay(&mut self) -> Pin<Box<dyn std::future::Future<Output = Result<OperationStream, TransportError>> + Send + '_>> {
        Box::pin(async move {
            let rows = sqlx::query_as::<_, (Vec<u8>,)>("SELECT payload FROM lores_app_operations ORDER BY id ASC")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| TransportError::Other(e.to_string()))?;

            let s: OperationStream =
                Box::pin(stream::iter(rows).map(|(payload,)| Ok(RawEvent::Operation(RawOperationEvent::new_local(payload)))));
            Ok(s)
        })
    }
}
