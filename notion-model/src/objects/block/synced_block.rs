use serde::{Deserialize, Serialize};

use super::{Block, BlockData};
use crate::ids::BlockId;

/// # Synced block
///
/// Similar to the Notion UI, there are two versions of a synced_block object:
/// the original block that was created first and doesn't yet sync with anything
/// else, and the duplicate block or blocks synced to the original.
///
/// # 📘
/// An original synced block must be created before corresponding duplicate
/// block or blocks can be made.
///
/// # Original synced block
///
/// The value is always `None` to signify that this is an original synced block
/// that does not refer to another block.
///
/// # Duplicate synced block
///
/// The value of the synced_from is an object containing a [`BlockId`]
/// that refers to the original synced block.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct SyncedBlock {
    /// The original synced block that may or may not refer to another block.
    pub synced_from: Option<SyncedFrom>,
    /// The nested children. When calling the API, if this field is None, a
    /// Paragraph block is created by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

/// # Synced from
///
/// See [`SyncedBlock`] for more information.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default, Copy)]
#[serde(tag = "type", rename = "block_id")]
pub struct SyncedFrom {
    /// Id of the original synced block.
    pub block_id: BlockId,
}

impl SyncedBlock {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::SyncedBlock { synced_block: self })
    }

    pub fn synced_from(mut self, synced_from: Option<SyncedFrom>) -> Self {
        self.synced_from = synced_from;
        self
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}
