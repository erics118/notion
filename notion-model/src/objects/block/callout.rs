use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData, FileOrEmoji};
use crate::objects::color::Color;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Callout {
    /// An emoji or file object that represents the callout's icon.
    pub icon: FileOrEmoji,
    /// The color of the block.
    pub color: Color,
}

impl Callout {
    pub fn builder() -> CalloutBuilder {
        CalloutBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct CalloutBuilder(Callout);

impl CalloutBuilder {
    pub fn build(&self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Callout { callout: self.0 })
    }

    pub fn icon(mut self, icon: FileOrEmoji) -> Self {
        self.0.icon = icon;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.0.color = color;
        self
    }
}
