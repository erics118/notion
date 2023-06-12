use serde::{Deserialize, Serialize};

use crate::ids::{BlockId, DatabaseId, PageId};

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum ParentData {
    #[serde(rename = "database")]
    Database(DatabaseId),
    #[serde(rename = "page_id")]
    Page(PageId),
    #[serde(rename = "workspace")]
    Workspace,
    #[serde(rename = "block_id")]
    Block(BlockId),
}

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BlockParent {
    #[serde(flatten)]
    pub data: ParentData,
}
