use serde::{Deserialize, Serialize};

use super::FFile;

// TODO: fix working with file
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Image {
    /// A file object that details information about the file contained in
    /// the block.
    pub file: FFile,
}

impl Image {
    pub fn builder() -> ImageBuilder {
        ImageBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct ImageBuilder(Image);

impl ImageBuilder {
    pub fn build(&self) -> super::BlockBuilder {
        super::BlockBuilder::new(super::BlockData::Image { image: self.0 })
    }

    pub fn file(mut self, file: FFile) -> Self {
        self.0.file = file;
        self
    }
}
