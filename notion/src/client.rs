use anyhow::{Context, Result};
use notion_model::constants::{API_BASE_URL, API_VERSION};
use reqwest::{
    header::{self, HeaderMap, HeaderValue, CONTENT_TYPE},
    Client, ClientBuilder, RequestBuilder,
};

use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Notion {
    http: Client,
}

macro_rules! api_method {
    ($($method:ident;)*) => {
        paste::item! {
            $(
                #[allow(dead_code)]
                pub(crate) fn [< api_ $method>](&self, path: &str) -> RequestBuilder {
                    let url = self.api_url(path);
                    self.http.$method(&url)
                }
            )*
        }
    };
}

impl Notion {
    pub fn new(api_token: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();

        headers.insert("Notion-Version", HeaderValue::from_static(API_VERSION));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let auth = HeaderValue::from_str(&format!("Bearer {}", api_token))
            .context(Error::InvalidApiToken)?;

        headers.insert(header::AUTHORIZATION, auth);

        let http = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .context(Error::ClientBuild)?;

        Ok(Self { http })
    }

    /// Returns the absolute URL for an endpoint in the API.
    pub(crate) fn api_url(&self, path: &str) -> String {
        API_BASE_URL.to_owned() + path
    }

    // TODO: ratelimits
    api_method! {
        delete;
        get;
        head;
        patch;
        post;
        put;
    }
}
