use serde::{Deserialize, Serialize};

use super::File;

// TODO: video builder
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Video {
    /// A file object that details information about the file contained in
    /// the block.
    pub file: File,
}
