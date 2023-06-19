use serde::{Deserialize, Serialize};

use crate::objects::{
    file_and_emoji::{ExternalFile, InternalFile},
    rich_text::RichText,
};

/// # File block
///
/// The Notion API does not support uploading files to Notion.
///
/// TODO: external file builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct File {
    /// The rich text in the caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<RichText>>,
    /// A file object that details information about the file contained in
    /// the block.
    #[serde(flatten)]
    pub data: FileData,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FileData {
    File(InternalFile),
    External(ExternalFile),
}
