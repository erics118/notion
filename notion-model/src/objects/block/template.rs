use serde::{Deserialize, Serialize};

use super::Block;
use crate::objects::rich_text::RichText;

/// # Template block
///
/// # ❗️ Deprecation Notice
/// As of March 27, 2023 creation of template blocks will no longer be
/// supported.
///
/// Template blocks represent template buttons in the Notion UI.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Template {
    /// The rich text displayed in the title of the template.
    pub rich_text: Vec<RichText>,
    /// The nested child blocks, if any, of the template block. These blocks are
    /// duplicated when the template block is used in the UI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Block>>,
}
