use sqlx::{Sqlite, SqlitePool};

#[derive(sqlx::FromRow)]
pub struct RegisteredDomainProvidersRow {
    pub registration_id: String,
    pub domain_provider: String,
    pub credential: String,
    pub username: String,
}

pub struct RegisteredDomainProvidersRepo {}

impl RegisteredDomainProvidersRepo {
    pub fn init() -> Self {
        RegisteredDomainProvidersRepo {}
    }

    pub async fn upsert(&self, pool: &SqlitePool, credentials: &RegisteredDomainProvidersRow) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "
            INSERT INTO registered_domain_providers (registration_id, domain_provider, credential, username)
            VALUES(?, ?, ?, ?)
            ON CONFLICT(registration_id) DO UPDATE SET credential = excluded.credential AND username = excluded.username
            ",
            credentials.registration_id,
            credentials.domain_provider,
            credentials.credential,
            credentials.username
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn find(&self, pool: &SqlitePool, registration_id: &str) -> Result<Option<RegisteredDomainProvidersRow>, sqlx::Error> {
        let row = sqlx::query_as::<Sqlite, RegisteredDomainProvidersRow>(
            "
                SELECT registration_id, domain_provider, credential, username
                FROM registered_domain_providers
                WHERE registration_id = ?
            ",
        )
        .bind(registration_id)
        .fetch_optional(pool)
        .await?;

        Ok(row)
    }
}
