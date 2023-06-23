use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Title(Vec<crate::objects::rich_text::RichText>);

impl Title {
    pub fn new(text: &str) -> Self {
        Self(vec![crate::objects::rich_text::RichText::new_text(text)])
    }

    pub fn build(self) -> PropertyData {
        PropertyData::Title(self)
    }
    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (name.to_string(), Property::new(PropertyData::Title(self)))
    }
}
