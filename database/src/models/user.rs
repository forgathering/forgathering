use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub created_at: OffsetDateTime,

    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password_hash: String,

    pub avatar: Vec<u8>,
    pub bio: String,
    pub links: Vec<String>,
    pub following: Vec<Uuid>,

    pub landing_page: String,
    pub show_nsfw: bool,
}

impl User {
    pub fn new(username: String, nickname: String, email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            created_at: OffsetDateTime::from(SystemTime::now()),

            username,
            nickname,
            email,
            password_hash,

            avatar: Vec::new(),
            bio: String::new(),
            links: Vec::new(),
            following: Vec::new(),

            landing_page: String::new(),
            show_nsfw: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct ExternalUser {
    pub id: Uuid,
    pub server: String,
}
