use serde::{Deserialize, Serialize};

use super::Block;
use crate::ids::BlockId;

/// TODO: synced block builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_from: Option<BlockId>,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}
