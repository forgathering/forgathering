pub mod config;
pub mod postgres;
pub mod sqlite;

use argon2::{Argon2, Params};
use thiserror::Error;

use crate::{config::DatabaseConfig, postgres::PostgresBackend, sqlite::SqliteBackend};

#[derive(Clone)]
pub enum DatabaseBackend {
    Postgres(PostgresBackend),
    Sqlite(SqliteBackend),
}

#[derive(Clone)]
pub struct Database<'a> {
    argon2: Argon2<'a>,

    backend: DatabaseBackend,
}

#[derive(Error, Debug)]
pub enum OpenDatabaseError {
    #[error("Invalid Argon2 Parameters: {0}")]
    InvalidArgonParameters(argon2::Error),

    #[error("Error connecting to database: {0}")]
    ConnectionFailed(sqlx::Error),

    #[error("Error migrating database: {0}")]
    MigrationsFailed(sqlx::migrate::MigrateError),
}

pub trait BackendInterface {}

impl Database<'_> {
    pub async fn open(config: DatabaseConfig) -> Result<Self, OpenDatabaseError> {
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            Params::new(19 * 1024, 2, 1, Some(32))
                .map_err(OpenDatabaseError::InvalidArgonParameters)?,
        );

        let backend = match config {
            DatabaseConfig::Postgres { url } => {
                DatabaseBackend::Postgres(PostgresBackend::open(&url).await?)
            }
            DatabaseConfig::Sqlite { path } => {
                DatabaseBackend::Sqlite(SqliteBackend::open(&path).await?)
            }
        };

        Ok(Self { argon2, backend })
    }
}
