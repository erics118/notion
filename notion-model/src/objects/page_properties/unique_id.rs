use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct UniqueId {
    pub prefix: String,
    pub number: u32,
}

impl UniqueId {
    pub fn new(prefix: &str, number: u32) -> Self {
        Self {
            prefix: prefix.to_string(),
            number,
        }
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (
            name.to_string(),
            Property::new(PropertyData::UniqueId(self)),
        )
    }
}
