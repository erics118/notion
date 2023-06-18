use serde::{Deserialize, Serialize};

use super::file::FileData;
use crate::objects::rich_text::RichText;

/// # PDF block
///
/// *The Notion API does not support uploading files to Notion.
///
/// TODO: external image builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Pdf {
    /// The rich text in the caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<RichText>>,
    /// A file object that details information about the file contained in
    /// the block.
    #[serde(flatten)]
    pub data: FileData,
}
