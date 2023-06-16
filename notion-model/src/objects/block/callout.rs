use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData, FileOrEmoji};
use crate::objects::color::Color;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Callout {
    /// An emoji or file object that represents the callout's icon.
    pub icon: FileOrEmoji,
    /// The color of the block.
    pub color: Color,
}

impl Callout {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Callout {
    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Callout { callout: self })
    }

    pub fn icon(mut self, icon: FileOrEmoji) -> Self {
        self.icon = icon;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}
