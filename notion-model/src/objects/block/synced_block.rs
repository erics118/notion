use serde::{Deserialize, Serialize};

use super::{Block, BlockData};
use crate::ids::BlockId;

/// TODO: synced block builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct SyncedBlock {
    /// Similar to the Notion UI, there are two versions of a synced_block
    /// object: the original block that was created first and doesn't yet
    /// sync with anything else, and the duplicate block or blocks
    /// synced to the original.
    ///
    /// 📘 An original synced block must be created before corresponding
    /// duplicate block or blocks can be made. # Original synced block
    /// The value of `synced_from` is empty to signify that this is an
    /// original synced block that does not refer to another block.
    ///
    /// # Duplicate synced block
    /// The value of the synced_from is an object containing a [`BlockId`]
    pub synced_from: Option<BlockId>,
    /// The nested children. cannot be `None` when making the API call, but the
    /// API will return it as None.
    pub children: Option<Vec<Block>>,
}

impl SyncedBlock {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::SyncedBlock { synced_block: self })
    }

    pub fn synced_from(mut self, synced_from: Option<BlockId>) -> Self {
        self.synced_from = synced_from;
        self
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
