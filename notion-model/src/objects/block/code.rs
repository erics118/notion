use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};
use crate::objects::{code_languages::CodeLanguage, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Code {
    /// The rich text in the caption of the code block.
    pub caption: Vec<RichText>,
    /// The rich text in the code block.
    ///
    /// Usually is only a single [`RichTextType::Text`] element, but the API
    /// does allow you to use the full set of rich text features here,
    /// including all annotations, and the UI does show these annotations
    /// correctly, allowing for some fun possibilities.
    ///
    /// Syntax highlighting from a language does not result in any annotations.
    ///
    /// [`RichTextType::Text`]: crate::objects::rich_text::RichTextType::Text
    pub rich_text: Vec<RichText>,
    /// The language of the code contained in the code block.
    pub language: CodeLanguage,
}

impl Code {
    pub fn builder() -> CodeBuilder {
        CodeBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct CodeBuilder(Code);

impl CodeBuilder {
    pub fn build(&self) -> Result<BlockBuilder> {
        Ok(BlockBuilder::new(BlockData::Code {
            code: self.0.clone(),
        }))
    }

    pub fn caption(mut self, caption: Vec<RichText>) -> Self {
        self.0.caption = caption;
        self
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.0.rich_text = rich_text;
        self
    }

    pub fn language(mut self, language: CodeLanguage) -> Self {
        self.0.language = language;
        self
    }
}
