use serde::{Deserialize, Serialize};

use crate::objects::color::Color;

/// The types of rich text objects.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum RichTextType {
    Text,
    Mention,
    Equation,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct RichText {
    /// The type of this rich text object.
    pub type_: RichTextType,
    /// An object containing type-specific configuration.
    pub annotations: Annotations,
    /// The plain text without annotations.
    pub plain_text: String,
    /// The URL of any link or Notion mention in this text, if any.
    pub href: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Annotations {
    /// Whether the text is bolded.
    pub bold: bool,
    /// Whether the text is italicized.
    pub italic: bool,
    /// Whether the text is struck through.
    pub strikethrough: bool,
    /// Whether the text is underlined.
    pub underline: bool,
    /// The color of the text.
    pub color: Color,
    /// The background color of the text.
    pub background_color: Color,
    /// Whether the text is code.
    pub code: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Equation {
    /// The LaTeX string representing the inline equation.
    pub expression: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Mention {
    /// The type of mention.
    pub type_: MentionType,
    // An object with type-specific information about the mention.
    // TODO: pub object,
}