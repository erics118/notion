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
        // make sure that children is not None
        if let Some(children) = &self.children {
            // make sure there is at least 2 children
            if children.len() < 2 {
                panic!("Column must have at least 2 children");
            }

            // make sure each child is of type Column
            for child in children {
                if let BlockData::Column(_) = &child.data {
                    continue;
                } else {
                    panic!("ColumnList children must be of type Column");
                }
            }
        } else {
            panic!("Column cannot be empty.");
        }

        Block::new(BlockData::ColumnList(self))
    }

    /// Not Option<Vec<Block> here because it must be set when calling the API
    pub fn children(mut self, children: Vec<Block>) -> Self {
        self.children = Some(children);
        self
    }
}
