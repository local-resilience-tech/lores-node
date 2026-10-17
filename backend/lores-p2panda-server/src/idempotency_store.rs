use std::time::{Duration, SystemTime, UNIX_EPOCH};

use lores_p2panda::{RegionAppTopic, RegionTopic};
use sqlx::{QueryBuilder, Sqlite, SqlitePool};
use tokio::time::interval;
use tonic::Status;

const TABLE_NAME: &str = "publish_idempotency_keys_v1";
const DEPRECATED_TABLES: &[&str] = &["publish_idempotency_keys"];

pub struct IdempotencyStore {
    db: SqlitePool,
}

impl IdempotencyStore {
    pub async fn new(db: SqlitePool, cleanup_frequency: Duration, retention: Duration) -> Result<Self, sqlx::Error> {
        Self::setup_table(&db).await?;
        Self::spawn_cleanup(&db, cleanup_frequency, retention);
        Ok(Self { db })
    }

    /// Returns the stored operation_id if the key is a duplicate, or `None` if it is new.
    /// Returns `None` immediately when no key is supplied.
    pub async fn check_duplicate(&self, region_app_topic: &RegionAppTopic, idempotency_key: &[u8]) -> Result<Option<Vec<u8>>, Status> {
        if idempotency_key.is_empty() {
            return Ok(None);
        }

        let topic_bytes = region_app_topic.p2panda_topic().to_bytes().to_vec();
        let mut query = QueryBuilder::<Sqlite>::new("SELECT operation_id FROM ");
        query.push(TABLE_NAME);
        query.push(" WHERE topic = ");
        query.push_bind(&topic_bytes);
        query.push(" AND key = ");
        query.push_bind(idempotency_key);
        let row: Option<(Vec<u8>,)> = query
            .build_query_as()
            .fetch_optional(&self.db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(row.map(|(id,)| id))
    }

    /// Records an idempotency key with its operation_id. No-op when the key is empty.
    pub async fn record(&self, region_app_topic: &RegionAppTopic, idempotency_key: &[u8], operation_id: &[u8]) -> Result<(), Status> {
        if idempotency_key.is_empty() {
            return Ok(());
        }

        let topic_bytes = region_app_topic.p2panda_topic().to_bytes().to_vec();
        let seen_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        let mut query = QueryBuilder::<Sqlite>::new("INSERT OR IGNORE INTO ");
        query.push(TABLE_NAME);
        query.push(" (topic, key, operation_id, seen_at) VALUES (");
        {
            let mut values = query.separated(", ");
            values.push_bind(&topic_bytes);
            values.push_bind(idempotency_key);
            values.push_bind(operation_id);
            values.push_bind(seen_at);
        }
        query.push(")");
        query
            .build()
            .execute(&self.db)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(())
    }

    async fn setup_table(db: &SqlitePool) -> Result<(), sqlx::Error> {
        for table in DEPRECATED_TABLES {
            let mut query = QueryBuilder::<Sqlite>::new("DROP TABLE IF EXISTS ");
            query.push(table);
            query.build().execute(db).await?;
        }

        let mut create_table = QueryBuilder::<Sqlite>::new("CREATE TABLE IF NOT EXISTS ");
        create_table.push(TABLE_NAME);
        create_table.push(
            " (topic BLOB NOT NULL, key BLOB NOT NULL, operation_id BLOB NOT NULL, seen_at INTEGER NOT NULL, PRIMARY KEY (topic, key))",
        );
        create_table.build().execute(db).await?;

        let mut create_index = QueryBuilder::<Sqlite>::new("CREATE INDEX IF NOT EXISTS ");
        create_index.push("idx_pik_seen_at");
        create_index.push(" ON ");
        create_index.push(TABLE_NAME);
        create_index.push(" (seen_at)");
        create_index.build().execute(db).await?;

        Ok(())
    }

    fn spawn_cleanup(db: &SqlitePool, cleanup_frequency: Duration, retention: Duration) {
        let db = db.clone();
        let retention_secs = retention.as_secs() as i64;

        tokio::spawn(async move {
            let mut timer = interval(cleanup_frequency);
            loop {
                timer.tick().await;
                let cutoff = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64 - retention_secs;
                let mut query = QueryBuilder::<Sqlite>::new("DELETE FROM ");
                query.push(TABLE_NAME);
                query.push(" WHERE seen_at < ");
                query.push_bind(cutoff);
                let _ = query.build().execute(&db).await;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lores_p2panda::{RegionAppTopic, RegionId};
    use sqlx::sqlite::SqlitePoolOptions;

    async fn test_store() -> IdempotencyStore {
        // Use a single in-memory connection so all queries see the same database state.
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        // Use a very long cleanup frequency so the background task never fires during tests.
        IdempotencyStore::new(db, Duration::from_secs(u64::MAX / 2), Duration::from_hours(48))
            .await
            .unwrap()
    }

    fn topic(namespace: &str) -> RegionAppTopic {
        RegionAppTopic::new(RegionId::from([1u8; 32]), namespace)
    }

    // 1. is_duplicate returns false when key has not been recorded.
    #[tokio::test]
    async fn test_unknown_key_is_not_duplicate() {
        let store = test_store().await;
        let result = store.check_duplicate(&topic("app"), b"key-1").await.unwrap();
        assert!(result.is_none());
    }

    // 2. is_duplicate returns false when no key is supplied.
    #[tokio::test]
    async fn test_empty_key_is_not_duplicate() {
        let store = test_store().await;
        let result = store.check_duplicate(&topic("app"), b"").await.unwrap();
        assert!(result.is_none());
    }

    // 3. After record, is_duplicate returns true for the same key and topic.
    #[tokio::test]
    async fn test_recorded_key_is_duplicate() {
        let store = test_store().await;
        store.record(&topic("app"), b"key-1", b"op-id-1").await.unwrap();
        let result = store.check_duplicate(&topic("app"), b"key-1").await.unwrap();
        assert_eq!(result.as_deref(), Some(b"op-id-1".as_slice()));
    }

    // 4. Calling record twice with the same key does not error.
    #[tokio::test]
    async fn test_record_is_idempotent() {
        let store = test_store().await;
        store.record(&topic("app"), b"key-1", b"op-id-1").await.unwrap();
        store.record(&topic("app"), b"key-1", b"op-id-1").await.unwrap();
    }

    // 5. After manual cleanup removes an expired row, is_duplicate returns false.
    #[tokio::test]
    async fn test_expired_key_removed_by_cleanup() {
        let store = test_store().await;

        // Insert a row with seen_at in the distant past.
        let topic = topic("app");
        let topic_bytes = topic.p2panda_topic().to_bytes().to_vec();
        let mut insert = QueryBuilder::<Sqlite>::new("INSERT INTO ");
        insert.push(TABLE_NAME);
        insert.push(" (topic, key, operation_id, seen_at) VALUES (");
        {
            let mut values = insert.separated(", ");
            values.push_bind(&topic_bytes);
            values.push_bind(b"old-key".as_slice());
            values.push_bind(b"op-id".as_slice());
            values.push_bind(0i64); // epoch — definitely expired
        }
        insert.push(")");
        insert.build().execute(&store.db).await.unwrap();

        // Confirm it looks like a duplicate before cleanup.
        assert!(store.check_duplicate(&topic, b"old-key").await.unwrap().is_some());

        // Run cleanup with a cutoff of now (removes anything seen_at < now).
        let cutoff = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let mut delete = QueryBuilder::<Sqlite>::new("DELETE FROM ");
        delete.push(TABLE_NAME);
        delete.push(" WHERE seen_at < ");
        delete.push_bind(cutoff);
        delete.build().execute(&store.db).await.unwrap();

        // Now the key should be gone.
        assert!(store.check_duplicate(&topic, b"old-key").await.unwrap().is_none());
    }

    // 6. Keys are scoped by topic: the same key on different topics is independent.
    #[tokio::test]
    async fn test_keys_are_scoped_by_topic() {
        let store = test_store().await;
        let topic_a = topic("app-a");
        let topic_b = topic("app-b");

        store.record(&topic_a, b"key-1", b"op-id-1").await.unwrap();

        assert!(store.check_duplicate(&topic_a, b"key-1").await.unwrap().is_some());
        assert!(store.check_duplicate(&topic_b, b"key-1").await.unwrap().is_none());
    }

    // 7. Booting the store drops superseded (legacy unversioned) tables.
    #[tokio::test]
    async fn test_boot_drops_deprecated_tables() {
        let db = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            "CREATE TABLE publish_idempotency_keys (
                topic   BLOB    NOT NULL,
                key     BLOB    NOT NULL,
                seen_at INTEGER NOT NULL,
                PRIMARY KEY (topic, key)
            )",
        )
        .execute(&db)
        .await
        .unwrap();

        let store = IdempotencyStore::new(db, Duration::from_secs(u64::MAX / 2), Duration::from_hours(48))
            .await
            .unwrap();

        let legacy: Option<String> =
            sqlx::query_scalar("SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'publish_idempotency_keys'")
                .fetch_optional(&store.db)
                .await
                .unwrap();
        assert!(legacy.is_none());

        store.record(&topic("app"), b"key-1", b"op-id-1").await.unwrap();
        assert!(store.check_duplicate(&topic("app"), b"key-1").await.unwrap().is_some());
    }

    // 8. Re-initializing the store on the same database keeps existing rows
    //    (CREATE TABLE IF NOT EXISTS on the current version only).
    #[tokio::test]
    async fn test_reinit_preserves_current_version_rows() {
        let store = test_store().await;
        store.record(&topic("app"), b"key-1", b"op-id-1").await.unwrap();

        let store2 = IdempotencyStore::new(store.db.clone(), Duration::from_secs(u64::MAX / 2), Duration::from_hours(48))
            .await
            .unwrap();

        assert_eq!(
            store2.check_duplicate(&topic("app"), b"key-1").await.unwrap().as_deref(),
            Some(b"op-id-1".as_slice())
        );
    }
}
