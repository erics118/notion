use serde::{Deserialize, Serialize};

/// Link Preview block objects contain the originally pasted url.
///
/// 🚧 The link_preview block can only be returned as part of a response. The
/// API does not support creating or appending link_preview blocks.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct LinkPreview {
    /// The originally pasted url.
    pub url: String,
}
