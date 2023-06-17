use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Table {
    /// max table width is 100
    pub table_width: u32,
    pub has_column_header: bool,
    pub has_row_header: bool,
    /// must be TableRow, and not None when calling api, even though api returns
    /// it as None
    pub children: Option<Vec<Block>>,
}

impl Table {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Table { table: self })
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
