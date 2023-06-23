use serde::{Deserialize, Serialize};

use crate::ids::{BlockId, DatabaseId, PageId};

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", untagged)]
pub enum ParentData {
    DatabaseId { database_id: DatabaseId },
    PageId { page_id: PageId },
    Workspace { workspace: bool },
    BlockId { block_id: BlockId },
}

impl From<DatabaseId> for ParentData {
    fn from(database_id: DatabaseId) -> Self {
        Self::DatabaseId { database_id }
    }
}

impl From<PageId> for ParentData {
    fn from(page_id: PageId) -> Self {
        Self::PageId { page_id }
    }
}

impl From<BlockId> for ParentData {
    fn from(block_id: BlockId) -> Self {
        Self::BlockId { block_id }
    }
}
