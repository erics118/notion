use serde::{Deserialize, Serialize};

use super::{Block, BlockBuilder, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct NumberedListItem {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl NumberedListItem {
    pub fn builder() -> NumberedListItemBuilder {
        NumberedListItemBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct NumberedListItemBuilder(NumberedListItem);

impl NumberedListItemBuilder {
    pub fn build(&self) -> anyhow::Result<BlockBuilder> {
        Ok(BlockBuilder::new(BlockData::NumberedListItem {
            numbered_list_item: self.0.clone(),
        }))
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.0.rich_text = rich_text;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.0.color = color;
        self
    }
}
