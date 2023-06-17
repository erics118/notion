use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Equation {
    /// A KaTeX compatible string.
    pub expression: String,
}

impl Equation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_block(self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Equation { equation: self })
    }

    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = expression.into();
        self
    }
}
