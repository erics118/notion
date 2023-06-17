use serde::{Deserialize, Serialize};

use super::{Block, BlockData, FileOrEmoji};
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Callout {
    pub rich_text: Vec<RichText>,
    /// An emoji or file object that represents the callout's icon.
    pub icon: FileOrEmoji,
    /// The color of the block.
    pub color: Color,
}

impl Callout {
    pub fn with_emoji(emoji: String) -> Self {
        Self {
            icon: FileOrEmoji::Emoji { emoji },
            color: Default::default(),
            rich_text: Default::default(),
        }
    }

    pub fn build_block(self) -> Block {
        Block::new(BlockData::Callout { callout: self })
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
