use serde::{Deserialize, Serialize};

use super::file::FileData;
use crate::objects::rich_text::RichText;

/// # Video block
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Video {
    /// The rich text in the caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<RichText>>,
    #[serde(flatten)]
    pub data: FileData,
}
