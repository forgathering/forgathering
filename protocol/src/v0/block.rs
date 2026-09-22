use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{deserialize_image, serialize_image};

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Block.ts")]
pub struct Block {
    pub id: Uuid,

    #[ts(type = "Date")]
    pub created_at: OffsetDateTime,
    #[ts(type = "Date")]
    pub modified_at: OffsetDateTime,

    pub markdown: Option<String>,
    pub link: Option<String>,
    #[serde(
        serialize_with = "serialize_image",
        deserialize_with = "deserialize_image"
    )]
    #[ts(type = "string | null")]
    pub image: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Block.ts")]
pub struct BlockCreate {
    pub id: Uuid,

    pub markdown: Option<String>,
    pub link: Option<String>,
    #[serde(
        serialize_with = "serialize_image",
        deserialize_with = "deserialize_image"
    )]
    #[ts(type = "string | null")]
    pub image: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Block.ts")]
pub struct BlockUpdate {
    pub id: Uuid,

    pub markdown: Option<String>,
    pub link: Option<String>,
    #[serde(
        serialize_with = "serialize_image",
        deserialize_with = "deserialize_image"
    )]
    #[ts(type = "string | null")]
    pub image: Option<Vec<u8>>,
}
