use serde::{Deserialize, Serialize};

use super::{Block, BlockData};
use crate::objects::rich_text::RichText;

/// # Table row block
///
/// Follow these steps to fetch the table_rows of a table:
///
/// Get the table ID from a query to Retrieve block children for the parent
/// page.
///
/// Get the table_rows from a query to Retrieve block children for the table.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct TableRow {
    /// An array of cell contents in horizontal display order. Each cell is an
    /// array of rich text objects.
    ///
    /// This is indeed supposed to be a `Vec<Vec<RichText>>` and not a
    /// `Vec<RichText>`. `cells[x]` would be the contents of a single cell.
    pub cells: Vec<Vec<RichText>>,
}

impl TableRow {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::TableRow { table_row: self })
    }

    pub fn cells(mut self, cells: Vec<Vec<RichText>>) -> Self {
        self.cells = cells;
        self
    }
}
