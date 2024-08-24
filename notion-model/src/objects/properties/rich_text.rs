use serde::{Deserialize, Serialize};

use super::{Property, PropertyData};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RichText(Vec<crate::objects::rich_text::RichText>);

impl RichText {
    pub fn new(text: Vec<crate::objects::rich_text::RichText>) -> Self {
        Self(text)
    }

    pub fn build_with_name(self, name: &str) -> (String, Property) {
        (
            name.to_string(),
            Property::new(PropertyData::RichText(self)),
        )
    }
}
