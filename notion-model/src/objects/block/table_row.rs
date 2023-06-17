use serde::{Deserialize, Serialize};

use super::{Block, BlockData};
use crate::objects::rich_text::RichText;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct TableRow {
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
