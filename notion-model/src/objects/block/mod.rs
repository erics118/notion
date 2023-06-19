use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{parent::ParentData, rich_text::Mention, user::PartialUser};
use crate::ids::BlockId;

mod bookmark;
mod breadcrumb;
mod bulleted_list_item;
mod callout;
mod child_database;
mod child_page;
mod code;
mod column;
mod column_list;
mod divider;
mod embed;

mod file;
mod heading_1;
mod heading_2;
mod heading_3;
mod image;
mod link_preview;
mod numbered_list_item;
mod paragraph;
mod pdf;
mod quote;
mod synced_block;
mod table;
mod table_of_contents;
mod table_row;
mod template;
mod to_do;
mod toggle;
mod video;

pub use bookmark::Bookmark;
pub use breadcrumb::Breadcrumb;
pub use bulleted_list_item::BulletedListItem;
pub use callout::Callout;
pub use child_database::ChildDatabase;
pub use child_page::ChildPage;
pub use code::Code;
pub use column::Column;
pub use column_list::ColumnList;
pub use divider::Divider;
pub use embed::Embed;
pub use file::File;
pub use heading_1::Heading1;
pub use heading_2::Heading2;
pub use heading_3::Heading3;
pub use image::Image;
pub use link_preview::LinkPreview;
pub use numbered_list_item::NumberedListItem;
pub use paragraph::Paragraph;
pub use pdf::Pdf;
pub use quote::Quote;
pub use synced_block::SyncedBlock;
pub use table::Table;
pub use table_of_contents::TableOfContents;
pub use table_row::TableRow;
pub use template::Template;
pub use to_do::ToDo;
pub use toggle::Toggle;
pub use video::Video;

/// # Block datatype
///
/// A block object represents a piece of content within Notion. The API
/// translates the headings, toggles, paragraphs, lists, media, and more that
/// you can interact with in the Notion UI as different block type objects.
///
/// Each field except for `data` represents a property that all blocks have.
/// When calling the API, none of these fields are required and don't do
/// anything, except for `archived`, which can be used to "delete" a block.
///
/// But, when you receive a block object from the API, these fields
/// should all be present.
///
/// The `data` field contains block-specific data.
///
/// To create a block, it is preferred to use the `build` function on a
/// `BlockData` variant.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(tag = "object", rename = "block")]
pub struct Block {
    /// Identifier for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<BlockId>,

    /// Information about the block's parent. See [`BlockParent`] for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ParentData>,

    /// Date and time when this block was created. Formatted as an ISO 8601 date
    /// time string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime<Utc>>,

    /// Date and time when this block was last updated. Formatted as an ISO 8601
    /// date time string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_time: Option<DateTime<Utc>>,

    /// User who created the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<PartialUser>,

    /// User who last edited the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_by: Option<PartialUser>,

    /// Whether or not the block has children blocks nested within it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_children: Option<bool>,

    /// The archived status of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    /// The block-specific data. See [`BlockData`] for details.
    #[serde(flatten)]
    pub data: BlockData,
}

impl Block {
    pub fn new(data: BlockData) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }

    pub fn new_() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: BlockId) -> Self {
        self.id = Some(id);
        self
    }

    pub fn archived(mut self, archived: Option<bool>) -> Self {
        self.archived = archived;
        self
    }

    pub fn data(mut self, data: BlockData) -> Self {
        self.data = data;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum BlockData {
    Bookmark(Bookmark),
    Breadcrumb(Breadcrumb),
    BulletedListItem(BulletedListItem),
    Callout(Callout),
    ChildDatabase(ChildDatabase),
    ChildPage(ChildPage),
    Column(Column),
    ColumnList(ColumnList),
    Code(Code),
    Divider(Divider),
    Embed(Embed),
    File(File),
    #[serde(rename = "heading_1")]
    Heading1(Heading1),
    #[serde(rename = "heading_2")]
    Heading2(Heading2),
    #[serde(rename = "heading_3")]
    Heading3(Heading3),
    Image(Image),
    LinkPreview(LinkPreview),
    Mention(Mention),
    NumberedListItem(NumberedListItem),
    Paragraph(Paragraph),
    Pdf(Pdf),
    Quote(Quote),
    SyncedBlock(SyncedBlock),
    Table(Table),
    TableOfContents(TableOfContents),
    TableRow(TableRow),
    Template(Template),
    ToDo(ToDo),
    Toggle(Toggle),
    Video(Video),
    #[default]
    Unsupported,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum SupportedImageTypes {
    Bmp,
    Gif,
    Heic,
    Jpeg,
    Jpg,
    Png,
    Svg,
    Tif,
    Tiff,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum SupportedVideoTypes {
    Amv,
    Asf,
    Avi,
    F4v,
    Flv,
    Gifv,
    Mkv,
    Mov,
    Mpg,
    Mpeg,
    Mpv,
    Mp4,
    M4v,
    Qt,
    Wmv,
    YoutubeEmbed,
    YoutubeWatch,
}
