use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Divider {
    /// This is present so that serde serializes this into `{}` rather than as
    /// `null`.
    #[serde(skip)]
    _nothing: (),
}

impl Divider {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Divider {
    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Divider { divider: self })
    }
}
