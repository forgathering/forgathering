use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, ts_rs::TS)]
#[ts(export, export_to = "v0/Version.ts")]
pub struct MaxApiVersion {
    version: usize,
}
