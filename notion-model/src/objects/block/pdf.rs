use serde::{Deserialize, Serialize};

use crate::objects::rich_text::RichText;

/// no pdf builder bc cant upload files
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Pdf {
    /// A caption, if provided, for the PDF block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<RichText>>,
}
