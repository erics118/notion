use serde::{Deserialize, Serialize};

/// cannot create a child database block, as child dbs are children of pages,
/// not children of blocks.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildDatabase {
    pub title: String,
}
