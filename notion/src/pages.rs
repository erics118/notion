use anyhow::{Context, Result};
use notion_model::{ids::PageId, objects::page::Page};

use crate::{
    client::{Notion, SendAndGetText},
    errors::{Error, NotionApiError},
    result_types,
};

impl Notion {
    /// # Create a page
    ///
    /// Creates a new page that is a child of an existing page or database.
    ///
    /// If the new page is a child of an existing page,title is the only valid
    /// property in the properties body param.
    ///
    /// If the new page is a child of an existing database, the keys of the
    /// properties object body param must match the parent database's
    /// properties.
    ///
    /// This endpoint can be used to create a new page with or without content
    /// using the children option. To add content to a page after creating it,
    /// use the Append block children endpoint.
    ///
    /// Returns a new page object.
    ///
    /// # 🚧
    /// Some page properties are not supported via the API.
    ///
    /// A request body that includes rollup, created_by, created_time,
    /// last_edited_by, or last_edited_time values in the properties object
    /// returns an error. These Notion-generated values cannot be created or
    /// updated via the API. If the parent contains any of these properties,
    /// then the new page's corresponding values are automatically created.
    /// # 📘
    /// Requirements
    ///
    /// Your integration must have update content capabilities on the target
    /// parent page or database in order to call this endpoint. To update your
    /// integrations capabilities, navigation to the My integrations dashboard,
    /// select your integration, go to the Capabilities tab, and update your
    /// settings as needed.
    ///
    /// Attempting a query without update content capabilities returns an HTTP
    /// response with a 403 status code.
    pub async fn create_page(&self, page: Page) -> Result<Page> {
        let text = self
            .api_post("pages")
            .json(&page)
            .send_and_get_text()
            .await?;

        let res = serde_json::from_str::<result_types::Page>(&text)
            .context(Error::SerializeResponse("Page", "create_page"))?;

        match res {
            result_types::Page::Page(page) => Ok(page),
            result_types::Page::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }

    /// # Retrieve a page
    ///
    /// Retrieves a Page object using the ID specified.
    ///
    /// Responses contains page properties, not page content. To fetch page
    /// content, use the Retrieve block children endpoint.
    ///
    /// Page properties are limited to up to 25 references per page property. To
    /// retrieve data related to properties that have more than 25 references,
    /// use the Retrieve a page property endpoint.
    ///
    /// # Parent objects: Pages vs. databases
    ///
    /// If a page's Parent object is a database, then the property values will
    /// conform to the database property schema.
    ///
    /// If a page object is not part of a database, then the only property value
    /// available for that page is its title.
    ///
    /// # Limits
    ///
    /// The endpoint returns a maximum of 25 page or person references per page
    /// property. If a page property includes more than 25 references, then the
    /// 26th reference and beyond might be returned as Untitled, Anonymous, or
    /// not be returned at all.
    ///
    /// This limit affects the following properties:
    ///
    /// people: response object can/t be guaranteed to return more than 25
    /// people. relation: the has_more value of the relation in the response
    /// object is true if a relation contains more than 25 related pages.
    /// Otherwise, has_more is false. rollup: the has_more value of the
    /// rollup in the response object is true if a rollup contains more than 25
    /// references. Otherwise, has_more is false. rich_text: response object
    /// includes a maximum of 25 populated inline page or person mentions.
    /// title: response object includes a maximum of 25 inline page or person
    /// mentions.
    ///
    /// # 🚧
    /// This endpoint will not accurately return properties that exceed 25
    /// references
    ///
    /// Do not use this endpoint if a page property includes more than 25
    /// references to receive the full list of references. Instead, use the
    /// Retrieve a page property endpoint for the specific property to get its
    /// complete reference list.
    ///
    /// # Implementation details
    ///
    /// We limit the number of references returned in the response object to 25
    /// by truncating the list of references to 25.
    ///
    /// # 📘 Integration capabilities
    ///
    /// This endpoint requires an integration to have read content capabilities.
    /// Attempting to call this API without read content capabilities will
    /// return an HTTP response with a 403 status code. For more information on
    /// integration capabilities, see the capabilities guide. Errors
    ///
    /// Returns a 404 HTTP response if the page doesn't exist, or if the
    /// integration doesn't have access to the page.
    ///
    /// Returns a 400 or 429 HTTP response if the request exceeds the request
    /// limits.
    pub async fn retrieve_page(
        &self,
        page_id: PageId,
        filter_properties: Option<Vec<&str>>,
    ) -> Result<Page> {
        let query = filter_properties
            .iter()
            .map(|p| ("filter_properties", p))
            .collect::<Vec<_>>();

        let text = self
            .api_get(&format!("pages/{page_id}"))
            .query(&query)
            .send_and_get_text()
            .await?;

        println!("{}", text);

        // let text = include_str!("../../test_data/page.json");

        let res = serde_json::from_str::<result_types::Page>(&text)
            .context(Error::SerializeResponse("Page", "retrieve_page"))?;

        match res {
            result_types::Page::Page(mut page) => {
                // only keep the first 25 page properties
                page.properties = page.properties.into_iter().take(25).collect();
                Ok(page)
            },
            result_types::Page::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }
}
