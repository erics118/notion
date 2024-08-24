use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FileOrEmoji {
    Emoji { emoji: String },
    File { file: InternalFile },
    External { external: ExternalFile },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ExternalFile {
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename = "file")]
pub struct InternalFile {
    url: String,
    expiry_time: DateTime<Utc>,
}
