use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::{BlockBuilder, BlockData};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Embed {
    /// The link for the embed.
    pub url: String,
}

impl Embed {
    pub fn builder() -> EmbedBuilder {
        EmbedBuilder(Self::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct EmbedBuilder(Embed);

impl EmbedBuilder {
    pub fn build(&self) -> Result<BlockBuilder> {
        if self.0.url.is_empty() {
            anyhow::bail!("Embed caption and url must not be empty");
        }

        Ok(BlockBuilder::new(BlockData::Embed {
            embed: self.0.clone(),
        }))
    }

    pub fn url(mut self, url: String) -> Self {
        self.0.url = url;
        self
    }
}
