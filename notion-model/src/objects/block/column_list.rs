use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

// TODO: column list builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct ColumnList {
    /// This is present so that serde serializes this into `{}` rather than as
    /// `null`.
    #[serde(skip)]
    _nothing: (),
}

impl ColumnList {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn build_block(self) -> Block {
        Block::new(BlockData::ColumnList { column_list: self })
    }
}
