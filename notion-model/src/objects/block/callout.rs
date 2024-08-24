use serde::{Deserialize, Serialize};

use super::{Block, BlockData};
use crate::objects::{color::Color, file_and_emoji::FileOrEmoji, rich_text::RichText};

/// # Callout block
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Callout {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// An emoji or file object that represents the callout's icon.
    pub icon: FileOrEmoji,
    /// The color of the block.
    pub color: Color,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl Callout {
    pub fn with_emoji(emoji: String) -> Self {
        Self {
            icon: FileOrEmoji::Emoji { emoji },
            color: Default::default(),
            rich_text: Default::default(),
            children: Default::default(),
        }
    }

    /// TODO: implement this function
    pub fn with_internal_file() {
        todo!()
    }

    /// TODO: implement this function
    pub fn with_external_file() {
        todo!()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Callout(self))
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn icon(mut self, icon: FileOrEmoji) -> Self {
        self.icon = icon;
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
mod tests {
    use super::*;
    use crate::objects::block::Paragraph;

    #[test]
    fn empty() {
        let value = Callout::with_emoji("👋".to_string()).build();

        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            r#"{"object":"block","callout":{"rich_text":[],"icon":{"emoji":"👋"},"color":"default"}}"#
        );
    }

    #[test]
    fn simple() {
        let value = Callout::with_emoji("👋".to_string())
            .color(Color::BlueBackground)
            .build();

        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            r#"{"object":"block","callout":{"rich_text":[],"icon":{"emoji":"👋"},"color":"blue_background"}}"#
        );
    }

    #[test]
    fn children() {
        let value = Callout::with_emoji("👋".to_string())
            .color(Color::BlueBackground)
            .children(Some(vec![
                Paragraph::new()
                    .rich_text(vec![RichText::new_text("hi")])
                    .build(),
            ]))
            .build();

        assert_eq!(
            serde_json::to_string(&value).unwrap(),
            r#"{"object":"block","callout":{"rich_text":[],"icon":{"emoji":"👋"},"color":"blue_background","children":[{"object":"block","paragraph":{"rich_text":[{"text":{"content":"hi"}}],"color":"default"}}]}}"#
        );
    }
}
