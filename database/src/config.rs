use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum DatabaseConfig {
    Postgres { url: String },
    Sqlite { path: PathBuf },
}
