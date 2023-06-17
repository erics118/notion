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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Bookmark {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Bookmark { bookmark: self })
    }

    pub fn caption(mut self, caption: Option<Vec<RichText>>) -> Self {
        self.caption = caption;
        self
    }

    pub fn url(mut self, url: Option<impl Into<String>>) -> Self {
        self.url = url.map(|a| a.into());
        self
    }
}

#[cfg(test)]
mod test {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn empty() {
        let value = Bookmark::new().build_block();

        assert_tokens(
            &value,
            &[
                Token::Map { len: None },
                Token::Str("object"),
                Token::Str("block"),
                Token::Str("type"),
                Token::Str("bookmark"),
                Token::Str("bookmark"),
                Token::Struct {
                    name: "Bookmark",
                    len: 0,
                },
                Token::StructEnd,
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn complete() {
        let value = Bookmark::new()
            .url(Some("https://google.com/".to_string()))
            .build_block();

        assert_tokens(
            &value,
            &[
                Token::Map { len: None },
                Token::Str("object"),
                Token::Str("block"),
                Token::Str("type"),
                Token::Str("bookmark"),
                Token::Str("bookmark"),
                Token::Struct {
                    name: "Bookmark",
                    len: 1,
                },
                Token::Str("url"),
                Token::Some,
                Token::Str("https://google.com/"),
                Token::StructEnd,
                Token::MapEnd,
            ],
        );
    }
}
