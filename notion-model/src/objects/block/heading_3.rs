use serde::{Deserialize, Serialize};

use super::Block;
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename = "heading_3")]
pub struct Heading3 {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// Whether or not the heading block is a toggle heading or not. If
    /// `true`, then the heading block toggles and can support
    /// children. If `false`, then the heading block is a static
    /// heading block.
    pub is_toggleable: bool,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}
