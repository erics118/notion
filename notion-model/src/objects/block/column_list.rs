use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

/// # Column list block
///
/// Column lists are parent blocks for columns.
///
/// When creating a column_list block via Append block children, the column_list
/// must have at least two columns, and each column must have at least one
/// child.
///
/// # Retrieve the content in a column list
///
/// Follow these steps to fetch the content in a column_list:
///
/// 1. Get the column_list ID from a query to Retrieve block children for the
/// parent page.
///
/// 2. Get the column children from a query to Retrieve block children for the
/// column_list.
///
/// 3. Get the content in each individual column from a query to Retrieve block
/// children for the unique column ID.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ColumnList {
    /// The nested columns. Must be of the [`BlockData::Column`] type. This
    /// field cannot be `None` when making the API call. But, the API will
    /// return it as `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl ColumnList {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::ColumnList(self))
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
