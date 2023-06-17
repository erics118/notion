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
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Column { column: self })
    }
}
