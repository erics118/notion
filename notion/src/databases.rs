use anyhow::{Context, Result};
use notion_model::{ids::DatabaseId, objects::database::Database};

use crate::{
    client::{Notion, SendAndGetText},
    errors::{Error, NotionApiError},
    result_types,
};

impl Notion {
    /// Create a database
    /// POST
    /// https://api.notion.com/v1/databases
    /// Creates a database as a subpage in the specified parent database, with
    /// the specified properties schema. Currently, the parent of a new
    /// database must be a Notion database or a wiki database.
    ///
    /// # 📘
    /// Integration capabilities
    ///
    /// This endpoint requires an integration to have insert content
    /// capabilities. Attempting to call this API without insert content
    /// capabilities will return an HTTP response with a 403 status code. For
    /// more information on integration capabilities, see the capabilities
    /// guide.
    /// # 🚧
    /// Limitations
    ///
    /// Only empty status database properties can be created
    /// # Errors
    ///
    /// Returns a 404 if the specified parent database does not exist, or if the
    /// integration does not have access to the parent database.
    ///
    /// Returns a 400 if the request is incorrectly formatted, or a 429 HTTP
    /// response if the request exceeds the request limits.
    ///
    /// Note: Each Public API endpoint can return several possible error codes.
    /// See the Error codes section of the Status codes documentation for more
    /// information.
    pub async fn create_database(&self, database: Database) -> Result<Database> {
        let text = self
            .api_post("databases")
            .json(&database)
            .send_and_get_text()
            .await?;

        let res = serde_json::from_str::<result_types::Database>(&text)
            .context(Error::SerializeResponse("Database", "create_database"))?;

        match res {
            result_types::Database::Database(database) => Ok(database),
            result_types::Database::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }

    /// # Retrieve a database
    ///
    /// Retrieves a database object — information that describes the structure
    /// and columns of a database — for a provided database ID. The response
    /// adheres to any limits to an integration’s capabilities.
    ///
    /// To fetch database rows rather than columns, use the Query a database
    /// endpoint.
    ///
    /// To find a database ID, navigate to the database URL in your Notion
    /// workspace. The ID is the string of characters in the URL that is between
    /// the slash following the workspace name (if applicable) and the question
    /// mark. The ID is a 32 characters alphanumeric string.
    ///
    /// Refer to the Build your first integration guide for more details.
    ///
    /// # Errors
    ///
    /// Each Public API endpoint can return several possible error codes. See
    /// the Error codes section of the Status codes documentation for more
    /// information.
    ///
    /// # 📘
    /// Database relations must be shared with your integration
    pub async fn retrieve_database(&self, database_id: DatabaseId) -> Result<Database> {
        let text = self
            .api_get(&format!("databases/{database_id}"))
            .send_and_get_text()
            .await?;


        // let text = include_str!("../../test_data/database.json");

        println!("{}", text);

        let res = serde_json::from_str::<result_types::Database>(&text)
            .context(Error::SerializeResponse("Database", "retrieve_database"))?;
        match res {
            result_types::Database::Database(mut database) => {
                // only keep the first 25 database properties
                // database.properties = database.properties.into_iter().take(25).collect();
                Ok(database)
            },
            result_types::Database::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }
}
