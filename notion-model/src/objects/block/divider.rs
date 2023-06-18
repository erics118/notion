use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

/// # Divider block
///
/// Divider block objects do not contain any information.
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

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Divider { divider: self })
    }
}
