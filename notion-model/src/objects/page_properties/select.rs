use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};
use crate::{ids::SelectOptionId, objects::color::OptionColor};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Select(Option<SelectOption>);

impl Select {
    pub fn new(option: SelectOption) -> Self {
        Self(Some(option))
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (name.to_string(), Property::new(PropertyData::Select(self)))
    }
}

/// A select option.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct SelectOption {
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
