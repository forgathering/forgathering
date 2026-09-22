use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Session.ts")]
pub struct SessionToken(#[ts(type = "string")] auth::session_token::SessionToken);

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Session.ts")]
pub struct SessionCreate {
    pub designator: String,
    pub password: String,
}
