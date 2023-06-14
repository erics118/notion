use serde::{Deserialize, Serialize};

use super::{FFile, FileType};
use crate::objects::rich_text::RichText;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct File {
    /// The caption of the file block.
    pub caption: Vec<RichText>,
    /// The type of the file.
    #[serde(rename = "type")]
    pub type_: FileType,
    /// A file object that details information about the file contained in
    /// the block.
    pub file: FFile,
}
