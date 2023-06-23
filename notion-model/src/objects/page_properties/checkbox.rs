use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Checkbox(bool);

impl Checkbox {
    pub fn new(checked: bool) -> Self {
        Self(checked)
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (
            name.to_string(),
            Property::new(PropertyData::Checkbox(self)),
        )
    }
}
