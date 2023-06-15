use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

// TODO: column builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Column {
    /// This is present so that serde serializes this into `{}` rather than as
    /// `null`.
    #[serde(skip)]
    _nothing: (),
}

impl Column {
    pub fn builder() -> ColumnBuilder {
        ColumnBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct ColumnBuilder(Column);

impl ColumnBuilder {
    pub fn build(&self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Column { column: self.0 })
    }
}
