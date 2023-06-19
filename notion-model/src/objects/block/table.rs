use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

/// # Table block
///
/// Table block objects are parent blocks for table row children.
///
/// # 🚧
/// table_width can only be set when the table is first created.
///
/// Note that the number of columns in a table can only be set when the table is
/// first created. Calls to the Update block endpoint to update table_width
/// fail.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Table {
    /// The number of columns in the table.
    ///
    /// Note that this cannot be changed via the API once a table is
    /// created.
    ///
    /// The maximum table width is 100.
    pub table_width: u32,
    /// Whether the table has a column header. If true, then the first row in
    /// the table appears visually distinct from the other rows.
    pub has_column_header: bool,
    /// Whether the table has a header row. If true, then the first column in
    /// the table appears visually distinct from the other columns.
    pub has_row_header: bool,
    /// The nested children. Must be of the [`BlockData::TableRow`] type. This
    /// field cannot be `None` when making the API call. But, the API will
    /// return it as `None`.
    pub children: Option<Vec<Block>>,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Table(self))
    }

    pub fn table_width(mut self, table_width: u32) -> Self {
        self.table_width = table_width;
        self
    }

    pub fn column_header(mut self, column_header: bool) -> Self {
        self.has_column_header = column_header;
        self
    }

    pub fn row_header(mut self, row_header: bool) -> Self {
        self.has_row_header = row_header;
        self
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
