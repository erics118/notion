use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Column {
    /// The nested blocks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build_block(self) -> Block {
        Block::new(BlockData::Column { column: self })
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
