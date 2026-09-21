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

impl BackendInterface for SqliteBackend {
    fn add_user(
        &self,
        user: crate::models::user::User,
    ) -> Result<(), crate::DatabaseOperationError> {
        todo!()
    }

    fn get_user(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<crate::models::user::User>, crate::DatabaseOperationError> {
        todo!()
    }

    fn find_user_by_username(
        &self,
        username: String,
    ) -> Result<Option<crate::models::user::User>, crate::DatabaseOperationError> {
        todo!()
    }

    fn find_user_by_email(
        &self,
        email: String,
    ) -> Result<Option<crate::models::user::User>, crate::DatabaseOperationError> {
        todo!()
    }

    fn add_external_user(
        &self,
        user: crate::models::user::ExternalUser,
    ) -> Result<(), crate::DatabaseOperationError> {
        todo!()
    }

    fn get_external_user(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<crate::models::user::ExternalUser>, crate::DatabaseOperationError> {
        todo!()
    }

    fn add_session(
        &self,
        session: crate::models::session::Session,
    ) -> Result<(), crate::DatabaseOperationError> {
        todo!()
    }

    fn get_session(
        &self,
        session_id: uuid::Uuid,
    ) -> Result<Option<crate::models::session::Session>, crate::DatabaseOperationError> {
        todo!()
    }

    fn get_sessions_for_user(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<Vec<crate::models::session::Session>, crate::DatabaseOperationError> {
        todo!()
    }
}
