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
