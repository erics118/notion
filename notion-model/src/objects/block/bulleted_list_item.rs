use serde::{Deserialize, Serialize};

use super::{Block, BlockBuilder, BlockData};
use crate::objects::{color::Color, rich_text::RichText};

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
}

impl BulletedListItem {
    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::BulletedListItem {
            bulleted_list_item: self,
        })
    }

    pub fn rich_text(mut self, rich_text: Vec<RichText>) -> Self {
        self.rich_text = rich_text;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn empty() {
    //     assert_eq!(
    //         serde_json::to_string(&BulletedListItem::new().build_block()).
    // unwrap(),         r#"{"object":"block","type":"bulleted_list_item","
    // bulleted_list_item":{}}"#,     );
    // }

    // #[test]
    // fn rich_text() {
    //     assert_eq!(
    //         serde_json::to_string(
    //             &BulletedListItem::new()
    //                 .rich_text(vec![RichText::new_text("hello")])
    //                 .build_block()
    //         )
    //         .unwrap(),
    //         r#"{"object":"block","type":"bulleted_list_item","
    // bulleted_list_item":{}}"#,     );
    // }
}
