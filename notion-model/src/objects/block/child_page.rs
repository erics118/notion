use serde::{Deserialize, Serialize};

/// cannot create a child page block, as child pages are children of pages,
/// not children of blocks.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPage {
    pub title: String,
}
