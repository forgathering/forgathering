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

impl BackendInterface for PostgresBackend {
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
