use serde::{Deserialize, Serialize};

use super::{Block, BlockBuilder, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename = "heading_2")]
pub struct Heading2 {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// Whether or not the heading block is a toggle heading or not. If
    /// `true`, then the heading block toggles and can support
    /// children. If `false`, then the heading block is a static
    /// heading block.
    pub is_toggleable: bool,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl Heading2 {
    pub fn new_block(&self) -> BlockBuilder {
        println!("{}", serde_json::to_string(&self).unwrap());
        BlockBuilder::new(BlockData::Heading2 {
            heading_2: self.clone(),
        })
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn is_toggleable(mut self, is_toggleable: bool) -> Self {
        self.is_toggleable = is_toggleable;
        self
    }

    pub fn children(mut self, children: Vec<Block>) -> Self {
        self.children = Some(children);
        self
    }
}
