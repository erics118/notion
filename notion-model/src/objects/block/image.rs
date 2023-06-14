use serde::{Deserialize, Serialize};

use super::FFile;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Image {
    /// A file object that details information about the file contained in
    /// the block.
    pub file: FFile,
}
