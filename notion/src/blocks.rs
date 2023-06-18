use anyhow::{Context, Result};
use notion_model::{
    ids::NotionId,
    objects::block::{Block, BlockData},
};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

use crate::{client::Notion, errors::NotionApiError, model::paginated::List, result_types};

impl Notion {
    /// # Append block children
    ///
    /// Creates and appends new children blocks to the parent block_id
    /// specified. Blocks can be parented by other blocks, pages, or databases.
    ///
    /// Returns a paginated list of newly created first level children block
    /// objects.
    ///
    /// Existing blocks cannot be moved using this endpoint. Blocks are appended
    /// to the bottom of the parent block. Once a block is appended as a child,
    /// it can't be moved elsewhere via the API.
    ///
    /// For blocks that allow children, we allow up to two levels of nesting in
    /// a single request.
    ///
    /// # 📘 Integration capabilities
    ///
    /// This endpoint requires an integration to have insert content
    /// capabilities. Attempting to call this API without insert content
    /// capabilities will return an HTTP response with a 403 status code. For
    /// more information on integration capabilities, see the capabilities
    /// guide.
    ///
    /// # Errors
    ///
    /// Returns a 404 HTTP response if the block specified by id doesn't exist,
    /// or if the integration doesn't have access to the block.
    ///
    /// Returns a 400 or 429 HTTP response if the request exceeds the request
    /// limits.
    pub async fn append_block_children(
        &self,
        block_id: impl NotionId,
        children: Vec<Block>,
    ) -> Result<List<Block>> {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
        struct AppendBlockChildren {
            children: Vec<Block>,
        }

        let text = self
            .api_patch(&format!("blocks/{block_id}/children"))
            .header(CONTENT_TYPE, "application/json")
            .json(&AppendBlockChildren { children })
            .send()
            .await?
            .text()
            .await?;

        let res = serde_json::from_str::<result_types::List<Block>>(&text)?;

        match res {
            result_types::List::List(block_list) => Ok(block_list),
            result_types::List::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }

    /// # Retrieve a block
    ///
    /// # 📘
    /// If a block contains the key has_children: true, use the Retrieve block
    /// children endpoint to get the list of children
    ///
    /// # 📘 Integration capabilities
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
    pub async fn retrieve_block(&self, block_id: impl NotionId) -> Result<Block> {
        let text = self
            .api_get(&format!("blocks/{block_id}"))
            .send()
            .await?
            .text()
            .await?;

        // let text = test_json();
        println!("{}", text);

        let res = serde_json::from_str::<result_types::Block>(&text)?;

        match res {
            result_types::Block::Block(block) => Ok(block),
            result_types::Block::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }

    /// # Retrieve block children
    ///
    /// Returns a paginated array of child block objects contained in the block
    /// using the ID specified. In order to receive a complete representation of
    /// a block, you may need to recursively retrieve the block children of
    /// child blocks.
    ///
    /// # 🚧
    /// Returns only the first level of children for the specified block. See
    /// block objects for more detail on determining if that block has nested
    /// children. The response may contain fewer than page_size of results.
    ///
    /// See Pagination for details about how to use a cursor to iterate through
    /// the list.
    ///
    /// # 📘 Integration capabilities
    ///
    /// This endpoint requires an integration to have read content capabilities.
    /// Attempting to call this API without read content capabilities will
    /// return an HTTP response with a 403 status code. For more information on
    /// integration capabilities, see the capabilities guide.
    ///
    /// # Errors
    ///
    /// Returns a 404 HTTP response if the block specified by id doesn't exist,
    /// or if the integration doesn't have access to the block.
    ///
    /// Returns a 400 or 429 HTTP response if the request exceeds the request
    /// limits.
    pub async fn retrieve_block_children(&self, block_id: impl NotionId) -> Result<List<Block>> {
        let text = self
            .api_get(&format!("blocks/{block_id}/children"))
            .send()
            .await?
            .text()
            .await?;

        println!("{}", text);

        let res = serde_json::from_str::<result_types::List<Block>>(&text)
            .context("failed to turn into result_types::List<Block>")?;

        match res {
            result_types::List::List(block_list) => Ok(block_list),
            result_types::List::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }

    /// # Update a block
    ///
    /// Updates the content for the specified block_id based on the block type.
    /// Supported fields based on the block object type (see Block object for
    /// available fields and the expected input for each field).
    ///
    /// Note: The update replaces the entire value for a given field. If a field
    /// is omitted (ex: omitting checked when updating a to_do block), the value
    /// will not be changed.
    ///
    /// # 📘 Updating child_page blocks
    ///
    /// To update child_page type blocks, use the Update page endpoint. Updating
    /// the page's title updates the text displayed in the associated child_page
    /// block.
    ///
    /// # 📘 Updating child_database blocks
    ///
    /// To update child_database type blocks, use the Update database endpoint.
    /// Updating the page's title updates the text displayed in the associated
    /// child_database block.
    ///
    /// # 📘 Updating children
    ///
    /// A block's children CANNOT be directly updated with this endpoint.
    /// Instead use Append block children to add children.
    ///
    /// # 📘 Updating heading blocks
    ///
    /// To update the toggle of a heading block, you can include the optional
    /// is_toggleable property in the request. Toggle can be added and removed
    /// from a heading block. However, you cannot remove toggle from a heading
    /// block if it has children. All children MUST be removed before revoking
    /// toggle from a heading block. Success
    ///
    /// Returns a 200 HTTP response containing the updated block object on
    /// success.
    ///
    /// # 📘 Integration capabilities
    ///
    /// This endpoint requires an integration to have update content
    /// capabilities. Attempting to call this API without update content
    /// capabilities will return an HTTP response with a 403 status code. For
    /// more information on integration capabilities, see the capabilities
    /// guide.
    ///
    /// # Errors
    ///
    /// Returns a 404 HTTP response if the block doesn't exist, has been
    /// archived, or if the integration doesn't have access to the page.
    ///
    /// Returns a 400 if the type for the block is incorrect or the input is
    /// incorrect for a given field.
    ///
    /// Returns a 400 or a 429 HTTP response if the request exceeds the request
    /// limits.
    ///
    /// must have the block id and block data
    ///
    /// Optionally the block archived field can be set to archive/unarchive the
    /// block.
    /// TODO: finish this
    pub async fn update_block(
        &self,
        // block_id: impl NotionId,
        // block_data: BlockData,
        // archived: Option<bool>,
        block: Block,
    ) -> Result<Block> {
        #[derive(Serialize)]
        struct PartialBlock {
            #[serde(skip_serializing_if = "Option::is_none")]
            archived: Option<bool>,
            #[serde(flatten)]
            data: BlockData,
        }

        // let partial_block = PartialBlock {
        //     id: block_id.to_string(),
        //     block_data,
        //     archived,
        // };

        let block_id = block.id.context("fd")?.to_string();

        let partial_block = PartialBlock {
            data: block.data,
            archived: block.archived,
        };
        let body =
            serde_json::to_string(&partial_block).context("failed to serialize partial_block")?;
        // let body = r#"{
        //     "paragraph": {
        //       "rich_text": [{
        //         "text": { "content": "Lacifdsafdsfasnato kale" }
        //         }]
        //     }
        //   }"#;

        println!("{}", body);

        let text = self
            .http
            .patch(self.api_url(&format!("blocks/{block_id}")))
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await?
            .text()
            .await?;

        println!("{}", text);

        let res = serde_json::from_str::<result_types::Block>(&text)?;

        match res {
            result_types::Block::Block(block) => Ok(block),
            result_types::Block::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }

    /// # Delete a block
    ///
    /// Sets a Block object, including page blocks, to archived: true using the
    /// ID specified. Note: in the Notion UI application, this moves the block
    /// to the "Trash" where it can still be accessed and restored.
    ///
    /// To restore the block with the API, use the Update a block or Update page
    /// respectively.
    ///
    /// # 📘 Integration capabilities
    ///
    /// This endpoint requires an integration to have update content
    /// capabilities. Attempting to call this API without update content
    /// capabilities will return an HTTP response with a 403 status code. For
    /// more information on integration capabilities, see the capabilities
    /// guide.
    ///
    /// # Errors
    ///
    /// Returns a 404 HTTP response if the block doesn't exist, or if the
    /// integration doesn't have access to the block.
    ///
    /// Returns a 400 or 429 HTTP response if the request exceeds the request
    /// limits.
    /// /// TODO: test this
    pub async fn delete_block(&self, block_id: impl NotionId) -> Result<Block> {
        let text = self
            .api_delete(&format!("blocks/{block_id}"))
            .send()
            .await?
            .text()
            .await?;

        let res = serde_json::from_str::<result_types::Block>(&text)?;

        match res {
            result_types::Block::Block(block) => Ok(block),
            result_types::Block::Error(e) => anyhow::bail!(NotionApiError::from(e)),
        }
    }
}
