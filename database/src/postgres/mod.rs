use sqlx::PgPool;

use crate::{BackendInterface, OpenDatabaseError};

#[derive(Clone)]
pub struct PostgresBackend {
    pub db_pool: PgPool,
}

impl PostgresBackend {
    pub async fn open(url: &str) -> Result<Self, OpenDatabaseError> {
        let db_pool = PgPool::connect(url)
            .await
            .map_err(OpenDatabaseError::ConnectionFailed)?;

        sqlx::migrate!("migrations/postgres")
            .run(&db_pool)
            .await
            .map_err(OpenDatabaseError::MigrationsFailed)?;

        Ok(Self { db_pool })
    }
}

impl BackendInterface for PostgresBackend {}
