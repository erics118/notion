use serde::{Deserialize, Serialize};

use super::{Block, BlockBuilder, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename = "paragraph")]
pub struct Paragraph {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> ParagraphBuilder {
        ParagraphBuilder(Self::new())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ParagraphBuilder(Paragraph);

impl ParagraphBuilder {
    pub fn build(&self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Paragraph {
            paragraph: self.0.clone(),
        })
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.0.rich_text = rich_text;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.0.color = color;
        self
    }

    pub fn children(mut self, children: Vec<Block>) -> Self {
        self.0.children = Some(children);
        self
    }
}
