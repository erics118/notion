use serde::{Deserialize, Serialize};

use super::{Block, BlockData};

/// # Embed block
///
/// Embed block objects include information about another website displayed
/// within the Notion UI.
///
/// 🚧 Differences in embed blocks between the Notion app and the API
///
/// The Notion app uses a 3rd-party service, iFramely, to validate and request
/// metadata for embeds given a URL. This works well in a web app because Notion
/// can kick off an asynchronous request for URL information, which might take
/// seconds or longer to complete, and then update the block with the metadata
/// in the UI after receiving a response from iFramely.
///
/// We chose not to call iFramely when creating embed blocks in the API because
/// the API needs to be able to return faster than the UI, and because the
/// response from iFramely could actually cause us to change the block type.
/// This would result in a slow and potentially confusing experience as the
/// block in the response would not match the block sent in the request.
///
/// The result is that embed blocks created via the API may not look exactly
/// like their counterparts created in the Notion app.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Embed {
    /// The link to the website that the embed block displays.
    pub url: String,
}

impl Embed {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(self) -> Block {
        Block::new(BlockData::Embed { embed: self })
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }
}
