use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Block {
    pub id: Uuid,

    pub created_at: OffsetDateTime,
    pub modified_at: OffsetDateTime,

    pub markdown: Option<String>,
    pub link: Option<String>,
    pub image: Option<Vec<u8>>,
}

impl Block {
    pub fn new(markdown: Option<String>, link: Option<String>, image: Option<Vec<u8>>) -> Self {
        Self {
            id: Uuid::now_v7(),

            created_at: OffsetDateTime::from(SystemTime::now()),
            modified_at: OffsetDateTime::from(SystemTime::now()),

            markdown,
            link,
            image,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ExternalBlock {
    pub id: Uuid,

    pub server: String,
}
