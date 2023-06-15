use serde::{Deserialize, Serialize};

use super::{FFile, FileType};
use crate::objects::rich_text::RichText;

// TODO: file builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct File {
    /// The caption of the file block.
    pub caption: Vec<RichText>,
    /// The type of the file.
    #[serde(rename = "type")]
    pub type_: FileType,
    /// A file object that details information about the file contained in
    /// the block.
    pub file: FFile,
}

impl File {
    pub fn builder() -> FileBuilder {
        FileBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct FileBuilder(File);

impl FileBuilder {
    pub fn build(&self) -> anyhow::Result<super::BlockBuilder> {
        Ok(super::BlockBuilder::new(super::BlockData::File {
            file: self.0.clone(),
        }))
    }

    pub fn caption(mut self, caption: Vec<RichText>) -> Self {
        self.0.caption = caption;
        self
    }

    pub fn type_(mut self, type_: FileType) -> Self {
        self.0.type_ = type_;
        self
    }

    pub fn file(mut self, file: FFile) -> Self {
        self.0.file = file;
        self
    }
}
