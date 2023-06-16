use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Breadcrumb {
    /// This is present so that serde serializes this into `{}` rather than as
    /// `null`.
    #[serde(skip)]
    _nothing: (),
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Breadcrumb {
    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Breadcrumb { breadcrumb: self })
    }
}
