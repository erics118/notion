use serde::{Deserialize, Serialize};

use super::File;

// TODO: image builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Image {
    /// A file object that details information about the file contained in
    /// the block.
    pub file: File,
}

impl Image {
    pub fn builder() -> ImageBuilder {
        todo!()
        // ImageBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ImageBuilder(Image);

impl ImageBuilder {
    pub fn build(&self) -> super::BlockBuilder {
        todo!()
        // super::BlockBuilder::new(super::BlockData::Image { image: self.0 })
    }
}
