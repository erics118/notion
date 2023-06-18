use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

/// # Column block
///
/// Columns are parent blocks for any block types listed in this reference
/// except for other columns. They do not contain any information within the
/// column property. They can only be appended to column_lists.
///
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Column {
    /// The nested blocks. cannot be `None` when making the API call, but the
    /// API will return it as None.
    pub children: Option<Vec<Block>>,
}

impl Column {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Column { column: self })
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
