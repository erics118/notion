use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

// TODO
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct ColumnList {
    /// This is present so that serde serializes this into `{}` rather than as
    /// `null`.
    #[serde(skip)]
    _nothing: (),
}

impl ColumnList {
    pub fn builder() -> ColumnListBuilder {
        ColumnListBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct ColumnListBuilder(ColumnList);

impl ColumnListBuilder {
    pub fn build(&self) -> Result<BlockBuilder> {
        Ok(BlockBuilder::new(BlockData::ColumnList {
            column_list: self.0,
        }))
    }
}
