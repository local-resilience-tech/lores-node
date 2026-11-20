use sqlx::{Sqlite, SqlitePool, Transaction};

#[derive(sqlx::FromRow)]
pub struct RegisteredDomainProviderRow {
    pub registration_id: String,
    pub domain_provider: String,
    pub credential: String,
    pub username: String,
    pub registered_by: String,
}

pub struct RegisteredDomainProvidersRepo {}

impl RegisteredDomainProvidersRepo {
    pub fn init() -> Self {
        RegisteredDomainProvidersRepo {}
    }

    pub async fn upsert(&self, tx: &mut Transaction<'_, Sqlite>, credentials: &RegisteredDomainProviderRow) -> Result<(), sqlx::Error> {
        sqlx::query::<Sqlite>(
            "
            INSERT INTO registered_domain_providers (registration_id, domain_provider, credential, username, registered_by)
            VALUES(?, ?, ?, ?, ?)
            ON CONFLICT(registration_id) DO UPDATE SET credential = excluded.credential AND username = excluded.username AND registered_by = excluded.registered_by
            ",
        )
        .bind(&credentials.registration_id)
        .bind(&credentials.domain_provider)
        .bind(&credentials.credential)
        .bind(&credentials.username)
        .bind(&credentials.registered_by)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn all(&self, pool: &SqlitePool) -> Result<Vec<RegisteredDomainProviderRow>, sqlx::Error> {
        let rows = sqlx::query_as::<Sqlite, RegisteredDomainProviderRow>(
            "
                SELECT registration_id, domain_provider, credential, username, registered_by
                FROM registered_domain_providers
            ",
        )
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    pub async fn delete(&self, pool: &SqlitePool, registration_id: &str) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query("DELETE FROM registered_domain_providers WHERE registration_id = ?")
            .bind(registration_id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }
}
