use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PhoneNumber(Option<String>);

impl PhoneNumber {
    pub fn new(phone_number: String) -> Self {
        Self(Some(phone_number))
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (name.to_string(), Property::new(PropertyData::PhoneNumber(self)))
    }
}
