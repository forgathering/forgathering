use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,

    pub expires_at: OffsetDateTime,
}

impl Session {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_id,
            expires_at: OffsetDateTime::from(SystemTime::now() + Duration::from_days(3)),
        }
    }

    pub fn expired(&self) -> bool {
        SystemTime::now() < self.expires_at
    }
}
