use std::str::FromStr;
pub mod errors;
use anyhow::{Context, Result};
pub use errors::Error;
use notion_model::{
    constants::{API_BASE_URL, API_VERSION},
    ids::BlockId,
    objects::block::{Block, BlockMetadata},
};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

#[derive(Debug, Clone)]
pub struct Notion {
    client: Client,
}

impl Notion {
    pub fn new(api_token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();

        headers.insert("Notion-Version", HeaderValue::from_static(API_VERSION));

        let auth = HeaderValue::from_str(&format!("Bearer {}", api_token))
            .context(Error::InvalidApiToken)?;

        headers.insert(header::AUTHORIZATION, auth);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context(Error::ClientBuild)?;

        Ok(Self { client })
    }

    /// Returns the absolute URL for an endpoint in the API.
    fn api_url(&self, url: &str) -> String {
        let mut base = API_BASE_URL.to_owned();
        if !base.ends_with('/') {
            base.push('/');
        }
        base + url
    }

    #[inline]
    async fn api_get(&self, path: &str) -> Result<String> {
        let url = self.api_url(path);
        let res = self.client.get(&url).send().await?;
        // TODO: ratelimits
        let json = res.text().await?;

        Ok(json)
    }

    /// Retrieve a block
    ///
    /// Retrieves a [`Block`] object using the ID specified.
    ///
    /// 📘 If a block contains the key `has_children: true`, use the Retrieve
    /// block children endpoint to get the list of children
    ///
    /// 📘 Integration capabilities
    ///
    /// This endpoint requires an integration to have read content capabilities.
    /// Attempting to call this API without read content capabilities will
    /// return an HTTP response with a 403 status code. For more information on
    /// integration capabilities, see the capabilities guide.
    ///
    /// # Errors
    ///
    /// Returns a 404 HTTP response if the block doesn't exist, or if the
    /// integration doesn't have access to the block.
    ///
    /// Returns a 400 or 429 HTTP response if the request exceeds the request
    /// limits.

    pub async fn retrieve_block(&self, block_id: BlockId) -> Result<String> {
        let response = self.api_get(&format!("blocks/{block_id}")).await?;

        // TODO: handle ratelimits

        Ok(response)
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let notion = Notion::new("secret_PGwr76Ldv4dwJ8HqAdGQtS2CZzcFofNqDflUOdVc12v")?;

    let a = notion
        .retrieve_block(BlockId::from_str("d3d710f97c874e6c8e4d9b2576a6fb29").unwrap())
        .await?;

    println!("{a}");
    Ok(())
}
