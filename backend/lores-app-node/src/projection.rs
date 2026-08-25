use sha2::{Digest, Sha256};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

/// A SQLite pool initialised from a DDL schema string.
///
/// The pool is the projection read model — event handlers write to it,
/// API handlers read from it via plain SQL queries.
///
/// Schema versioning is handled automatically via a SHA-256 content hash.
/// App developers only write domain tables in their `schema.sql`; the
/// `_schema` bookkeeping table is owned by this library.
///
/// # In-memory
///
/// [`ProjectionDb::in_memory`] creates a fresh database on every process start
/// and applies the schema immediately.  State is populated by replaying the
/// full operation log.
///
/// # On-disk
///
/// [`ProjectionDb::open`] opens (or creates) an on-disk database.  On startup
/// it compares the hash of the supplied schema against the stored hash.  If
/// they differ the database is wiped, the new schema applied, and the caller
/// is expected to replay all operations before serving traffic.
pub struct ProjectionDb;

impl ProjectionDb {
    /// Create a new in-memory SQLite pool and apply `schema_sql`.
    ///
    /// The `_schema` table is created and populated automatically.
    /// App developers should not include it in their schema file.

    pub async fn in_memory(schema_sql: &str) -> Result<(SqlitePool, bool), sqlx::Error> {
        tracing::info!("creating in-memory projection database");
        let options = SqliteConnectOptions::new().filename(":memory:").create_if_missing(true);

        // A pool with more than one connection would give each connection its
        // own isolated in-memory database. Pin to one connection so all
        // callers share the same database.
        let pool = sqlx::pool::PoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;

        Self::apply_schema(&pool, schema_sql).await?;
        tracing::info!("projection database ready");
        Ok((pool, true))
    }

    /// Open (or create) an on-disk SQLite database.
    ///
    /// If the stored schema hash differs from the hash of `schema_sql`, all
    /// user tables are dropped and the schema is re-applied.  Returns `true`
    /// if a rebuild occurred (signalling that the caller should replay ops).
    pub async fn open(path: &str, schema_sql: &str) -> Result<(SqlitePool, bool), sqlx::Error> {
        let options = SqliteConnectOptions::new().filename(path).create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;
        let new_hash = Self::hash(schema_sql);

        let stored = sqlx::query_scalar::<_, String>("SELECT hash FROM _schema LIMIT 1")
            .fetch_optional(&pool)
            .await;

        let needs_rebuild = match stored {
            Ok(Some(h)) => h != new_hash,
            _ => true, // table missing or any error → rebuild
        };

        if needs_rebuild {
            tracing::info!(path, new_hash = %new_hash, "schema changed, rebuilding projection database");
            Self::drop_all_tables(&pool).await?;
            Self::apply_schema(&pool, schema_sql).await?;
            tracing::info!("projection database rebuild complete");
        } else {
            tracing::info!(path, "projection database schema unchanged, skipping rebuild");
        }

        Ok((pool, needs_rebuild))
    }

    // ── internals ────────────────────────────────────────────────────────────

    fn hash(schema_sql: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(schema_sql.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    async fn apply_schema(pool: &SqlitePool, schema_sql: &str) -> Result<(), sqlx::Error> {
        let hash = Self::hash(schema_sql);
        tracing::info!(schema_hash = %hash, "applying projection schema");

        // Create the framework-owned bookkeeping table first.
        sqlx::raw_sql("CREATE TABLE _schema (hash TEXT NOT NULL);")
            .execute(pool)
            .await?;

        sqlx::query("INSERT INTO _schema (hash) VALUES (?)")
            .bind(&hash)
            .execute(pool)
            .await?;

        // Apply the app-supplied schema.
        sqlx::raw_sql(schema_sql).execute(pool).await?;

        tracing::info!("projection schema applied");
        Ok(())
    }

    async fn drop_all_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        // Collect all user-created tables (excluding SQLite internals).
        let tables = sqlx::query_scalar::<_, String>(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'",
        )
        .fetch_all(pool)
        .await?;

        for table in tables {
            sqlx::raw_sql(&format!("DROP TABLE IF EXISTS \"{table}\""))
                .execute(pool)
                .await?;
        }

        Ok(())
    }
}
