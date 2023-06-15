use serde::{Deserialize, Serialize};

use super::{Block, BlockBuilder, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ToDo {
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// Whether the To do is checked.
    pub checked: bool,
    /// The color of the block.
    pub color: Color,
    /// The nested children (if any).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}

impl ToDo {
    pub fn builder() -> ToDoBuilder {
        ToDoBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct ToDoBuilder(ToDo);

impl ToDoBuilder {
    pub fn build(&self) -> BlockBuilder {
        BlockBuilder::new(BlockData::ToDo {
            to_do: self.0.clone(),
        })
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.0.rich_text = rich_text;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.0.checked = checked;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.0.color = color;
        self
    }

    pub fn children(mut self, children: Option<Vec<Block>>) -> Self {
        self.0.children = children;
        self
    }
}
