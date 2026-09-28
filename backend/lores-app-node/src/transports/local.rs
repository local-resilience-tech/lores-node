use std::pin::Pin;

use futures::{StreamExt, stream};
use lores_p2panda_client::SubscriptionFrom;
use sqlx::SqlitePool;

use crate::transports::{OperationStream, OperationTransport, RawEvent, RawOperationEvent, TransportError, TransportPublishResult};

#[derive(sqlx::FromRow)]
struct LocalOperationRow {
    #[allow(dead_code)]
    id: i64,
    payload: Vec<u8>,
}

// impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for LocalOperationRow {
//     fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
//         Ok(LocalOperationRow {
//             id: row.try_get("id")?,
//             payload: row.try_get("payload")?,
//         })
//     }
// }

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

    async fn stored_operations(&self) -> Result<Vec<LocalOperationRow>, sqlx::Error> {
        let rows = sqlx::query_as::<_, LocalOperationRow>("SELECT id, payload FROM lores_app_operations ORDER BY id ASC")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows)
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
            let rows = self.stored_operations().await.map_err(|e| TransportError::Other(e.to_string()))?;
            let total_operations = rows.len() as u32;

            let events = std::iter::once(RawEvent::ReplayStarted { total_operations })
                .chain(
                    rows.into_iter()
                        .map(|row| RawEvent::Operation(RawOperationEvent::new_local(row.payload))),
                )
                .chain(std::iter::once(RawEvent::ReplayEnded))
                .map(Ok);

            let s: OperationStream = Box::pin(stream::iter(events).chain(stream::pending()));
            Ok(s)
        })
    }
}
