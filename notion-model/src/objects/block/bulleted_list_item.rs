use serde::{Deserialize, Serialize};

use super::{Block, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

/// Bulleted list item block
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct BulletedListItem {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl BulletedListItem {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::BulletedListItem(self))
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.children = children;
        self
    }
}

#[cfg(test)]
mod test {
    use serde_test::{assert_tokens, Token};

    use super::*;

    #[test]
    fn empty() {
        let value = BulletedListItem::new().build();

        assert_tokens(
            &value,
            &[
                Token::Map { len: None },
                Token::Str("object"),
                Token::Str("block"),
                Token::Str("bulleted_list_item"),
                Token::Struct {
                    name: "BulletedListItem",
                    len: 2,
                },
                Token::Str("rich_text"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("color"),
                Token::UnitVariant {
                    name: "Color",
                    variant: "default",
                },
                Token::StructEnd,
                Token::MapEnd,
            ],
        );
    }

    #[test]
    fn complete() {
        let value = BulletedListItem::new()
            .rich_text(vec![RichText::new_text("hi")])
            .color(Color::BlueBackground)
            .build();

        assert_tokens(
            &value,
            &[
                Token::Map { len: None },
                Token::Str("object"),
                Token::Str("block"),
                Token::Str("bulleted_list_item"),
                Token::Struct {
                    name: "BulletedListItem",
                    len: 2,
                },
                Token::Str("rich_text"),
                Token::Seq { len: Some(1) },
                Token::Map { len: None },
                Token::Str("text"),
                Token::Struct {
                    name: "Text",
                    len: 1,
                },
                Token::Str("content"),
                Token::Str("hi"),
                Token::StructEnd,
                Token::MapEnd,
                Token::SeqEnd,
                Token::Str("color"),
                Token::UnitVariant {
                    name: "Color",
                    variant: "blue_background",
                },
                Token::StructEnd,
                Token::MapEnd,
            ],
        );
    }
}
