use serde::{Deserialize, Serialize};

use super::{Block, BlockBuilder, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Toggle {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl Toggle {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Toggle {
    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Toggle { toggle: self })
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
