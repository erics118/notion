use serde::{Deserialize, Serialize};

use crate::ids::{BlockId, DatabaseId, PageId};

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ParentData {
    DatabaseId(DatabaseId),
    Page(PageId),
    Workspace(bool),
    Block(BlockId),
}

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BlockParent {
    #[serde(flatten)]
    pub data: ParentData,
}
