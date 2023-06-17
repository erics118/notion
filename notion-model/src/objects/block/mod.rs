use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::{
    file::{ExternalFile, InternalFile},
    video::Video,
};
use super::{parent::BlockParent, rich_text::Mention, user::PartialUser};
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
mod equation;
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
pub use equation::Equation;
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

/// Fields common to all block types.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "object", rename = "block")]
// the `type` field is disregarded
pub struct Block {
    /// Identifier for the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<BlockId>,

    /// Information about the block's parent. See [`BlockParent`] for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<BlockParent>,

    /// Type of block. See [`BlockType`] for details.
    // pub r#type: BlockType,

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

    #[serde(flatten)]
    pub data: BlockData,
}

impl Block {
    pub const fn new(data: BlockData) -> Self {
        Self {
            id: None,
            parent: None,
            created_time: None,
            last_edited_time: None,
            created_by: None,
            last_edited_by: None,
            has_children: None,
            archived: None,
            data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum BlockData {
    Bookmark {
        bookmark: Bookmark,
    },
    Breadcrumb {
        breadcrumb: Breadcrumb,
    },
    BulletedListItem {
        bulleted_list_item: BulletedListItem,
    },
    Callout {
        callout: Callout,
    },
    ChildDatabase {
        child_database: ChildDatabase,
    },
    ChildPage {
        child_page: ChildPage,
    },
    Column {
        column: Column,
    },
    ColumnList {
        column_list: ColumnList,
    },
    Code {
        code: Code,
    },
    Divider {
        divider: Divider,
    },
    Embed {
        embed: Embed,
    },
    Equation {
        equation: Equation,
    },
    File {
        file: File,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        heading_1: Heading1,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        heading_2: Heading2,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        heading_3: Heading3,
    },
    Image {
        image: Image,
    },
    LinkPreview {
        link_preview: LinkPreview,
    },
    Mention {
        mention: Mention,
    },
    NumberedListItem {
        numbered_list_item: NumberedListItem,
    },
    Paragraph {
        paragraph: Paragraph,
    },
    Pdf {
        pdf: Pdf,
    },
    Quote {
        quote: Quote,
    },
    SyncedBlock {
        synced_block: SyncedBlock,
    },
    Table {
        table: Table,
    },
    TableOfContents {
        table_of_contents: TableOfContents,
    },
    TableRow {
        table_row: TableRow,
    },
    Template {
        template: Template,
    },
    ToDo {
        to_do: ToDo,
    },
    Toggle {
        toggle: Toggle,
    },
    Video {
        video: Video,
    },
    Unsupported,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename = "emoji")]
pub struct Emoji {
    pub emoji: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum FileOrEmoji {
    Emoji { emoji: String },
    File { file: InternalFile },
    External { external: ExternalFile },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum MentionType {
    Database,
    Data,
    LinkPreview,
    Page,
    User,
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
