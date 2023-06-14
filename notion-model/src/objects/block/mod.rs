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

pub use bookmark::Bookmark;
pub use breadcrumb::Breadcrumb;
pub use bulleted_list_item::BulletedListItem;
pub use callout::Callout;
pub use child_database::ChildDatabase;
pub use child_page::ChildPage;
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
pub use mention::Mention;
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
    // #[serde(rename = "heading_2")]
    pub data: BlockData,
}

impl BlockBuilder {
    pub fn new(data: BlockData) -> Self {
        Self { data }
    }
}

// TODO: use the `type` field when serializing. currently, when deserializing,
// the `type` field is just ignored.
// could possibly just add a type field to each variant of BlockData
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case", untagged)]

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
    Heading1 {
        heading_1: Heading1,
    },
    Heading2 {
        heading_2: Heading2,
    },
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]

pub struct Unsupported;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]

pub struct Emoji;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]

pub struct FFile;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]

pub enum FileOrEmoji {
    Emoji(Emoji),
    File(FFile),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]

pub enum FileType {
    File,
    External,
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

enum SupportedImageTypes {
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

enum SupportedVideoTypes {
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
