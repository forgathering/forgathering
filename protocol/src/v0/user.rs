use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{deserialize_image, serialize_image};

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/User.ts")]
pub struct User {
    pub id: Uuid,

    #[ts(type = "Date")]
    pub created_at: OffsetDateTime,

    pub username: String,
    pub nickname: String,
    pub email: String,

    #[serde(
        serialize_with = "serialize_image",
        deserialize_with = "deserialize_image"
    )]
    #[ts(type = "string | null")]
    pub avatar: Option<Vec<u8>>,
    pub bio: String,
    pub links: Vec<String>,

    pub users_followed: Vec<Uuid>,

    pub collections_followed: Vec<Uuid>,

    pub landing_page: String,
    pub show_nsfw: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/User.ts")]
pub struct UserCreate {
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/User.ts")]
pub struct UserUpdate {
    pub username: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,

    #[serde(
        serialize_with = "serialize_image",
        deserialize_with = "deserialize_image"
    )]
    #[ts(type = "string | null")]
    pub avatar: Option<Vec<u8>>,
    pub bio: Option<String>,
    pub links: Option<Vec<String>>,

    pub users_followed: Option<Vec<Uuid>>,

    pub collections_followed: Option<Vec<Uuid>>,

    pub landing_page: Option<String>,
    pub show_nsfw: Option<bool>,
}
