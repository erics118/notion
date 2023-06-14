use anyhow::Result;
use notion_model::{
    ids::BlockId,
    objects::block::{Block, BlockBuilder},
};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

use crate::client::Notion;

impl Notion {
    /// Append block children
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
    /// For blocks that allow children, we allow up to **two** levels of nesting
    /// in a single request.
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
        block_id: BlockId,
        children: Vec<BlockBuilder>,
    ) -> Result<()> {
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
        struct AppendBlockChildren {
            children: Vec<BlockBuilder>,
        }

        println!(
            "{}",
            serde_json::to_string(&AppendBlockChildren {
                children: children.clone()
            })?
        );
        let text = self
            .api_patch(&format!("blocks/{block_id}/children"))
            .header(CONTENT_TYPE, "application/json")
            .json(&AppendBlockChildren { children })
            .send()
            .await?
            .text()
            .await?;
        println!("{text}");
        Ok(())
    }

    /// Retrieve block children
    ///
    /// Returns a paginated array of child block objects contained in the block
    /// using the ID specified. In order to receive a complete representation of
    /// a block, you may need to recursively retrieve the block children of
    /// child blocks.
    ///
    /// 🚧 Returns only the first level of children for the specified block. See
    /// block objects for more detail on determining if that block has nested
    /// children.
    ///
    /// The response may contain fewer than page_size of results.
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
    pub async fn retrieve_block_children(&self, block_id: BlockId) -> Result<()> {
        let _text = self
            .api_get(&format!("blocks/{block_id}/children"))
            .send()
            .await?
            .text()
            .await?;

        todo!()
    }

    /// Update a block
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
    /// toggle from a heading block.
    ///
    /// # Success
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
    pub async fn update_block(&self, block_id: BlockId) -> Result<()> {
        let _text = self
            .api_get(&format!("blocks/{block_id}"))
            .send()
            .await?
            .text()
            .await?;

        todo!()
    }

    /// Delete a block
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
    pub async fn delete_block(&self, block_id: BlockId) -> Result<()> {
        let _text = self
            .api_delete(&format!("blocks/{block_id}"))
            .send()
            .await?
            .text()
            .await?;

        todo!()
    }

    /// Retrieve a block
    ///
    /// Retrieves a [`Block`] object using the ID specified.
    ///
    /// 📘 If a block contains the key `has_children: true`, use the Retrieve
    /// block children endpoint to get the list of children
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
    pub async fn retrieve_block(&self, block_id: BlockId) -> Result<Block> {
        let text = self
            .api_get(&format!("blocks/{block_id}"))
            .send()
            .await?
            .text()
            .await?;

        let deserialized = serde_json::from_str::<Block>(&text)?;

        Ok(deserialized)
    }
}
