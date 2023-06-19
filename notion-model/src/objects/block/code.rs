use serde::{Deserialize, Serialize};

use super::{Block, BlockData};
use crate::objects::{code_languages::CodeLanguage, rich_text::RichText};

/// # Code block
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Code {
    /// The rich text in the caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<RichText>>,
    /// The rich text in the code block.
    ///
    /// This usually is just a single [`RichTextData::Text`] element, but the
    /// API does allow you to use the full set of rich text features here,
    /// including all annotations, and the UI does show these annotations
    /// correctly, allowing for some fun possibilities.
    ///
    /// Syntax highlighting from a language does not result in any annotations.
    ///
    /// [`RichTextData::Text`]: crate::objects::rich_text::RichTextData::Text
    pub rich_text: Vec<RichText>,
    /// The language of the code contained in the code block.
    pub language: CodeLanguage,
}

impl Code {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Code(self))
    }

    pub fn caption(mut self, caption: Option<Vec<RichText>>) -> Self {
        self.caption = caption;
        self
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn language(mut self, language: CodeLanguage) -> Self {
        self.language = language;
        self
    }
}
