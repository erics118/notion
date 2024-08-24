use serde::{Deserialize, Serialize};

use crate::objects::date::DateOrDateTime;

/// Value of a formula property.
/// TODO: remove tag, use a struct if possible
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormulaData {
    Boolean { boolean: bool },
    Date { date: DateOrDateTime },
    Number { number: u32 },
    String { string: String },
}
