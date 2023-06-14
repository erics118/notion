use anyhow::Result;
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
    pub fn builder() -> DividerBuilder {
        DividerBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct DividerBuilder(Divider);

impl DividerBuilder {
    pub fn build(&self) -> Result<BlockBuilder> {
        Ok(BlockBuilder::new(BlockData::Divider { divider: self.0 }))
    }
}
