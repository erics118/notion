use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};
use crate::objects::rich_text::RichText;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Bookmark {
    /// The caption for the bookmark.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<RichText>>,
    /// The link for the bookmark.
    ///
    /// It is technically possible to make a bookmark without a caption, the
    /// caption isn't shown until you have added a URL.
    pub url: String,
}

impl Bookmark {
    pub fn builder() -> BookmarkBuilder {
        BookmarkBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct BookmarkBuilder(Bookmark);

impl BookmarkBuilder {
    pub fn build(&self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Bookmark {
            bookmark: self.0.clone(),
        })
    }

    pub fn caption(mut self, caption: Option<Vec<RichText>>) -> Self {
        self.0.caption = caption;
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url = url.into();
        self
    }
}
