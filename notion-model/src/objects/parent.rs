use serde::{Deserialize, Serialize};

use crate::ids::{BlockId, DatabaseId, PageId};

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", untagged)]
pub enum ParentData {
    /// # Database parent
    DatabaseId {
        /// The ID of the database that this page belongs to.
        database_id: DatabaseId,
    },
    /// # Page parent
    PageId {
        /// The ID of the page that this page belongs to.
        page_id: PageId,
    },
    /// # Workspace parent
    /// A page with a workspace parent is a top-level page within a Notion
    /// workspace. The parent property is an object containing the following
    /// keys:
    Workspace {
        /// Always true.
        workspace: bool,
    },
    /// # Block parent
    /// A page may have a block parent if it is created inline in a chunk of
    /// text, or is located beneath another block like a toggle or bullet
    /// block. The parent property is an object containing the following
    /// keys:
    BlockId {
        /// The ID of the page that this page belongs to.
        block_id: BlockId,
    },
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
