use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Equation {
    /// A KaTeX compatible string.
    pub expression: String,
}

impl Equation {
    pub fn builder() -> EquationBuilder {
        EquationBuilder(Self {
            expression: String::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct EquationBuilder(Equation);

impl EquationBuilder {
    pub fn build(&self) -> BlockBuilder {
        BlockBuilder::new(BlockData::Equation {
            equation: self.0.clone(),
        })
    }

    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.0.expression = expression.into();
        self
    }
}
