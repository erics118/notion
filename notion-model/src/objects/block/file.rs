use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData, FileType};
use crate::objects::rich_text::RichText;

// TODO: file builder
/// The Notion API does not yet support uploading files to Notion.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct File {
    /// The caption of the file block.
    pub caption: Vec<RichText>,
    /// The type of the file.
    #[serde(rename = "type")]
    pub type_: FileType,
    /// A file object that details information about the file contained in
    /// the block.
    #[serde(flatten)]
    pub data: FileData,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]

pub enum FileData {
    File(InternalFile),
    External(ExternalFile),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct InternalFile {
    pub url: String,
    pub expiry_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ExternalFile {
    pub url: String,
}

impl File {
    pub fn builder() -> Self {
        todo!()
        // File {}
    }
}

impl File {
    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::File { file: self })
    }

    pub fn caption(mut self, caption: Vec<RichText>) -> Self {
        self.caption = caption;
        self
    }
}
