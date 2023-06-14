use serde::{Deserialize, Serialize};

use crate::objects::rich_text::RichText;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Bookmark {
    /// The caption for the bookmark.
    pub caption: Vec<RichText>,
    /// The link for the bookmark.
    pub url: String,
}
