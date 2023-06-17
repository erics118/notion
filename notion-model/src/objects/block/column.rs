use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Column {
    /// The nested blocks.
    pub children: Vec<Block>,
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Column { column: self })
    }

    pub fn children(mut self, children: Vec<Block>) -> Self {
        self.children = children;
        self
    }
}
