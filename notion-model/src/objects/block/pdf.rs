use serde::{Deserialize, Serialize};

use super::FileType;
use crate::objects::rich_text::RichText;

/// TODO: pdf builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Pdf {
    /// A caption, if provided, for the PDF block.
    pub caption: Vec<RichText>,
    /// The type of the PDF.
    #[serde(rename = "type")]
    pub type_: FileType, /* file for internal, external for external
                          * then a { url:} */
}
