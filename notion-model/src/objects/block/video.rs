use serde::{Deserialize, Serialize};

use super::FFile;

// TODO: video builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Video {
    /// A file object that details information about the file contained in
    /// the block.
    pub file: FFile,
}
