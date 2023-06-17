use serde::{Deserialize, Serialize};

use super::MentionType;

/// TODO: mention builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct Mention {
    /// The type of mention.
    #[serde(rename = "type")]
    pub type_: MentionType,
    // An object with type-specific information about the mention.
    // TODO: data object
}
