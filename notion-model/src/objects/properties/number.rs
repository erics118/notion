use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Number(Option<f64>);

impl Number {
    pub fn new(number: f64) -> Self {
        Self(Some(number))
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (name.to_string(), Property::new(PropertyData::Number(self)))
    }
}
