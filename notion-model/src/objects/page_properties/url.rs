use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Url(Option<String>);

impl Url {
    pub fn new(url: String) -> Self {
        Self(Some(url))
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (name.to_string(), Property::new(PropertyData::Url(self)))
    }
}
