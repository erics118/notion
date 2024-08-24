use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Email(Option<String>);

impl Email {
    pub fn new(email: String) -> Self {
        Self(Some(email))
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (name.to_string(), Property::new(PropertyData::Email(self)))
    }
}
