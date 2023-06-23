use serde::{Deserialize, Serialize};


use crate::{ids::SelectOptionId, objects::color::OptionColor};



/// A select option, but for a status object. Looks and functions the exact
/// same.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Status {
    /// The color of the option.
    pub color: OptionColor,
    /// The ID of the option.
    ///
    /// You can use id or name to update a select property.
    pub id: SelectOptionId,
    /// The name of the option as it appears in Notion.
    ///
    /// If the select database property doesn't have an option by that name yet,
    /// then the name is added to the database schema if the integration also
    /// has write access to the parent database.
    ///
    /// Note: Commas (",") are not valid for select values.
    pub name: String,
}
