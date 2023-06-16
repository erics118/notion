use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct TableOfContents {
    /// This is present so that serde serializes this into `{}` rather than as
    /// `null`.
    #[serde(skip)]
    _nothing: (),
}

impl TableOfContents {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TableOfContents {
    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::TableOfContents {
            table_of_contents: self,
        })
    }
}
