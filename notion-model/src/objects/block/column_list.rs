use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ColumnList {
    /// The nested columns. Must be of the [`BlockData::Column`] type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl ColumnList {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build_block(self) -> Block {
        Block::new(BlockData::ColumnList { column_list: self })
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
