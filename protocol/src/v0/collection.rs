use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Collection.ts")]
pub struct Collection {
    pub id: Uuid,

    #[ts(type = "Date")]
    pub created_at: OffsetDateTime,
    #[ts(type = "Date")]
    pub modified_at: OffsetDateTime,

    pub owner_id: Uuid,
    pub private: bool,
    pub collaborators: Vec<Uuid>,

    pub name: String,

    pub important_blocks: Vec<Uuid>,
    pub blocks: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Collection.ts")]
pub struct CollectionCreate {
    pub name: String,
    pub private: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Collection.ts")]
pub struct CollectionOwnerUpdate {
    pub owner_id: Uuid,
    pub private: bool,
    pub collaborators: Vec<Uuid>,

    pub name: String,

    pub important_blocks: Vec<Uuid>,
    pub blocks: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Collection.ts")]
pub struct CollectionCollaboratorUpdate {
    pub important_blocks: Vec<Uuid>,
    pub blocks: Vec<Uuid>,
}
