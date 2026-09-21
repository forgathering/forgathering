#![feature(duration_constructors)]

pub mod config;
pub mod models;
pub mod postgres;
pub mod sqlite;

use thiserror::Error;
use uuid::Uuid;

use crate::{
    config::DatabaseConfig,
    models::{
        session::Session,
        user::{ExternalUser, User},
    },
    postgres::PostgresBackend,
    sqlite::SqliteBackend,
};

#[derive(Clone)]
pub enum DatabaseBackend {
    Postgres(PostgresBackend),
    Sqlite(SqliteBackend),
}

#[derive(Clone)]
pub struct Database {
    backend: DatabaseBackend,
}

#[derive(Error, Debug)]
pub enum OpenDatabaseError {
    #[error("Error connecting to database: {0}")]
    ConnectionFailed(sqlx::Error),

    #[error("Error migrating database: {0}")]
    MigrationsFailed(sqlx::migrate::MigrateError),
}

#[derive(Error, Debug)]
pub enum DatabaseOperationError {}

pub trait BackendInterface {
    // user operations
    fn add_user(&self, user: User) -> Result<(), DatabaseOperationError>;
    fn get_user(&self, id: Uuid) -> Result<Option<User>, DatabaseOperationError>;
    fn find_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<User>, DatabaseOperationError>;
    fn find_user_by_email(&self, email: String) -> Result<Option<User>, DatabaseOperationError>;

    // external user operations
    fn add_external_user(&self, user: ExternalUser) -> Result<(), DatabaseOperationError>;
    fn get_external_user(&self, id: Uuid) -> Result<Option<ExternalUser>, DatabaseOperationError>;

    // session operations
    fn add_session(&self, session: Session) -> Result<(), DatabaseOperationError>;
    fn get_session(&self, session_id: Uuid) -> Result<Option<Session>, DatabaseOperationError>;
    fn get_sessions_for_user(&self, user_id: Uuid) -> Result<Vec<Session>, DatabaseOperationError>;
}

impl Database {
    pub async fn open(config: DatabaseConfig) -> Result<Self, OpenDatabaseError> {
        let backend = match config {
            DatabaseConfig::Postgres { url } => {
                DatabaseBackend::Postgres(PostgresBackend::open(&url).await?)
            }
            DatabaseConfig::Sqlite { path } => {
                DatabaseBackend::Sqlite(SqliteBackend::open(&path).await?)
            }
        };

        Ok(Self { backend })
    }

    // user operations
    pub fn add_user(&self, user: User) -> Result<(), DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => postgres_backend.add_user(user),
            DatabaseBackend::Sqlite(sqlite_backend) => sqlite_backend.add_user(user),
        }
    }

    pub fn get_user(&self, id: Uuid) -> Result<Option<User>, DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => postgres_backend.get_user(id),
            DatabaseBackend::Sqlite(sqlite_backend) => sqlite_backend.get_user(id),
        }
    }

    pub fn find_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<User>, DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => {
                postgres_backend.find_user_by_username(username)
            }
            DatabaseBackend::Sqlite(sqlite_backend) => {
                sqlite_backend.find_user_by_username(username)
            }
        }
    }

    pub fn find_user_by_email(
        &self,
        email: String,
    ) -> Result<Option<User>, DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => {
                postgres_backend.find_user_by_email(email)
            }
            DatabaseBackend::Sqlite(sqlite_backend) => sqlite_backend.find_user_by_email(email),
        }
    }

    // external user operations
    pub fn add_external_user(&self, user: ExternalUser) -> Result<(), DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => postgres_backend.add_external_user(user),
            DatabaseBackend::Sqlite(sqlite_backend) => sqlite_backend.add_external_user(user),
        }
    }

    pub fn get_external_user(
        &self,
        id: Uuid,
    ) -> Result<Option<ExternalUser>, DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => postgres_backend.get_external_user(id),
            DatabaseBackend::Sqlite(sqlite_backend) => sqlite_backend.get_external_user(id),
        }
    }

    // session operations
    pub fn add_session(&self, session: Session) -> Result<(), DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => postgres_backend.add_session(session),
            DatabaseBackend::Sqlite(sqlite_backend) => sqlite_backend.add_session(session),
        }
    }

    pub fn get_session(&self, session_id: Uuid) -> Result<Option<Session>, DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => postgres_backend.get_session(session_id),
            DatabaseBackend::Sqlite(sqlite_backend) => sqlite_backend.get_session(session_id),
        }
    }

    pub fn get_sessions_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Session>, DatabaseOperationError> {
        match &self.backend {
            DatabaseBackend::Postgres(postgres_backend) => {
                postgres_backend.get_sessions_for_user(user_id)
            }
            DatabaseBackend::Sqlite(sqlite_backend) => {
                sqlite_backend.get_sessions_for_user(user_id)
            }
        }
    }
}
