use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};
use crate::objects::rich_text::DateMention;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Date(Option<DateMention>);

impl Date {
    pub fn new(date: DateMention) -> Self {
        Self(Some(date))
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (name.to_string(), Property::new(PropertyData::Date(self)))
    }
}
