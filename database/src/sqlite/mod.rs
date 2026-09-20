use std::path::Path;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use crate::{BackendInterface, OpenDatabaseError};

#[derive(Clone)]
pub struct SqliteBackend {
    pub db_pool: SqlitePool,
}

impl SqliteBackend {
    pub async fn open(file_path: &Path) -> Result<Self, OpenDatabaseError> {
        let options = SqliteConnectOptions::new()
            .filename(file_path)
            .create_if_missing(true);

        let db_pool = SqlitePool::connect_with(options)
            .await
            .map_err(OpenDatabaseError::ConnectionFailed)?;

        sqlx::migrate!("migrations/sqlite")
            .run(&db_pool)
            .await
            .map_err(OpenDatabaseError::MigrationsFailed)?;

        Ok(Self { db_pool })
    }
}

impl BackendInterface for SqliteBackend {}
