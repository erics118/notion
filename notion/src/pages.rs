use anyhow::{Context, Result};
use notion_model::{
    ids::{BlockId, NotionId},
    objects::{block::Block, page::Page},
};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

use crate::{
    client::Notion,
    errors::NotionApiError,
    model::paginated::List,
    result_types::{self},
};

/// Internal module to store the results of the API calls.
///
/// The API returns a JSON object with a `object` field that indicates the type
/// of the result. This module defines the types of the result and the
/// deserialization logic.
///
/// For the user-facing API, we return the deserialized result or an error,
/// rather than a struct in this module.

impl Notion {
    pub async fn retrieve_page(&self, page_id: impl NotionId) -> Result<Page> {
        let text = self
            .api_get(&format!("pages/{page_id}"))
            .send()
            .await?
            .text()
            .await?;

        // let text = test_json();
        println!("{}", text);

        let res = serde_json::from_str::<result_types::Page>(&text)?;

        match res {
            result_types::Page::Page(page) => Ok(page),
            result_types::Page::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }
}
