use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::objects::rich_text::RichText;

/// no file builder.
/// The Notion API does not yet support uploading files to Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct File {
    /// The caption of the file block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<RichText>>,
    /// A file object that details information about the file contained in
    /// the block.
    #[serde(flatten)]
    pub data: FileData,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FileData {
    File { file: InternalFile },
    External { external: ExternalFile },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct InternalFile {
    pub url: String,
    pub expiry_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ExternalFile {
    pub url: String,
}
