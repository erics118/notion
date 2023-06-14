use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{Block, BlockBuilder, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct BulletedListItem {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl BulletedListItem {
    pub fn builder() -> BulletedListItemBuilder {
        BulletedListItemBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct BulletedListItemBuilder(BulletedListItem);

impl BulletedListItemBuilder {
    pub fn build(&self) -> Result<BlockBuilder> {
        Ok(BlockBuilder::new(BlockData::BulletedListItem {
            bulleted_list_item: self.0.clone(),
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
