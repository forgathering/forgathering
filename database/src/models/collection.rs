use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Collection {
    pub id: Uuid,

    pub created_at: OffsetDateTime,
    pub modified_at: OffsetDateTime,

    pub owner_id: Uuid,
    pub private: bool,
    pub collaborators: Vec<Uuid>,

    pub name: String,

    pub important_blocks: Vec<Uuid>,
    pub blocks: Vec<Uuid>,
}

impl Collection {
    pub fn new(owner_id: Uuid, name: String, private: bool) -> Self {
        Self {
            id: Uuid::now_v7(),

            created_at: OffsetDateTime::from(SystemTime::now()),
            modified_at: OffsetDateTime::from(SystemTime::now()),

            owner_id,
            private,
            collaborators: Vec::new(),

            name,

            important_blocks: Vec::new(),
            blocks: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ExternalCollection {
    pub id: Uuid,

    pub server: String,
}
