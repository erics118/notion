use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::video::Video;
use super::{parent::BlockParent, user::PartialUser};
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
mod mention;
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

pub use bookmark::*;
pub use breadcrumb::*;
pub use bulleted_list_item::*;
pub use callout::*;
pub use child_database::*;
pub use child_page::*;
pub use code::*;
pub use column::*;
pub use column_list::*;
pub use divider::*;
pub use embed::*;
pub use equation::*;
pub use file::*;
pub use heading_1::*;
pub use heading_2::*;
pub use heading_3::*;
pub use image::*;
pub use link_preview::*;
pub use mention::*;
pub use numbered_list_item::*;
pub use paragraph::*;
pub use pdf::*;
pub use quote::*;
pub use synced_block::*;
pub use table::*;
pub use table_of_contents::*;
pub use table_row::*;
pub use template::*;
pub use to_do::*;
pub use toggle::*;

/// Fields common to all block types.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "object", rename = "block")]
// the `type` field is disregarded
pub struct Block {
    /// Identifier for the block.
    pub id: BlockId,

    /// Information about the block's parent. See [`BlockParent`] for details.
    pub parent: BlockParent,

    /// Type of block. See [`BlockType`] for details.
    // pub r#type: BlockType,

    /// Date and time when this block was created. Formatted as an ISO 8601 date
    /// time string.
    pub created_time: DateTime<Utc>,

    /// Date and time when this block was last updated. Formatted as an ISO 8601
    /// date time string.
    pub last_edited_time: DateTime<Utc>,

    /// User who created the block.
    pub created_by: PartialUser,

    /// User who last edited the block.
    pub last_edited_by: PartialUser,

    /// Whether or not the block has children blocks nested within it.
    pub has_children: bool,

    /// The archived status of the block.
    pub archived: bool,

    #[serde(flatten)]
    pub data: BlockData,
}

/// Fields common to all block types.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "object", rename = "block")]
pub struct BlockBuilder {
    #[serde(flatten)]
    pub data: BlockData,
}

impl BlockBuilder {
    pub const fn new(data: BlockData) -> Self {
        Self { data }
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
    Emoji(Emoji),
    File(File),
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
