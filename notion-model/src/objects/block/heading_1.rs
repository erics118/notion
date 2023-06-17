use serde::{Deserialize, Serialize};

use super::{Block, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename = "heading_1")]
pub struct Heading1 {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// Whether or not the heading block is a toggle heading or not. If
    /// `true`, then the heading block toggles and can support
    /// children. If `false`, then the heading block is a static
    /// heading block.
    #[serde(rename = "is_toggleable")]
    pub toggleable: bool,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl Heading1 {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Heading1 { heading_1: self })
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn toggleable(mut self, toggleable: bool) -> Self {
        self.toggleable = toggleable;
        self
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
