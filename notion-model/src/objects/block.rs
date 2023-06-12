use chrono::{DateTime, Utc};
use monostate::MustBe;
use serde::{Deserialize, Serialize};

use super::{color::Color, parent::BlockParent, rich_text::RichText, user::PartialUser};
use crate::ids::BlockId;

/// Fields common to all block types.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Block {
    /// Always "block"
    object: MustBe!("block"),

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
pub struct BlockBuilder {
    /// Always "block"
    // object: MustBe!("block"),

    #[serde(flatten)]
    pub data: BlockData,
}

impl BlockBuilder {
    #[allow(non_upper_case_globals, dead_code)]
    const object: &str = "block";

    pub fn new(data: BlockData) -> Self {
        println!("{}", serde_json::to_string(&data).unwrap());
        Self { data }
    }

}

// TODO: use the `type` field when serializing. currently, when deserializing,
// the `type` field is just ignored.
// could possibly just add a type field to each variant of BlockData
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BlockData {
    Bookmark {
        /// The caption for the bookmark.
        caption: Vec<RichText>,
        /// The link for the bookmark.
        url: String,
    },
    Breadcrumb {},
    BulletedListItem {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// The color of the block.
        color: Color,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    Callout {
        /// An emoji or file object that represents the callout's icon.
        icon: FileOrEmoji,
        /// The color of the block.
        color: Color,
    },
    ChildDatabase {
        title: String,
    },
    ChildPage {
        title: String,
    },
    Column,
    ColumnList,
    Divider,
    Embed {
        url: String,
    },
    Equation {
        /// A KaTeX compatible string.
        expression: String,
    },
    File {
        /// The caption of the file block.
        caption: Vec<RichText>,
        /// The type of the file.
        #[serde(rename = "type")]
        type_: FileType,
        /// A file object that details information about the file contained in
        /// the block.
        file: FFile,
    },
    #[serde(rename = "heading_1")]
    Heading1 {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// The color of the block.
        color: Color,
        /// Whether or not the heading block is a toggle heading or not. If
        /// `true`, then the heading block toggles and can support
        /// children. If `false`, then the heading block is a static
        /// heading block.
        is_toggleable: bool,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    #[serde(rename = "heading_2")]
    Heading2 {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// The color of the block.
        color: Color,
        /// Whether or not the heading block is a toggle heading or not. If
        /// `true`, then the heading block toggles and can support
        /// children. If `false`, then the heading block is a static
        /// heading block.
        is_toggleable: bool,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    #[serde(rename = "heading_3")]
    Heading3 {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// The color of the block.
        color: Color,
        /// Whether or not the heading block is a toggle heading or not. If
        /// `true`, then the heading block toggles and can support
        /// children. If `false`, then the heading block is a static
        /// heading block.
        is_toggleable: bool,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    Image {
        /// A file object that details information about the file contained in
        /// the block.
        file: FFile,
    },
    LinkPreview {
        /// The originally pasted url.
        url: String,
    },
    Mention {
        /// The type of mention.
        #[serde(rename = "type")]
        type_: MentionType,
        // An object with type-specific information about the mention.
        // TODO: object,
    },
    NumberedListItem {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// The color of the block.
        color: Color,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    Paragraph {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// The color of the block.
        color: Color,
        /// The nested child blocks (if it has any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    Pdf {
        /// A caption, if provided, for the PDF block.
        caption: Vec<RichText>,
        /// The type of the PDF.
        #[serde(rename = "type")]
        type_: FileType, /* file for internal, external for external
                          * then a { url:} */
    },
    Quote {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// The color of the block.
        color: Color,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    SyncedBlock {
        /// Similar to the Notion UI, there are two versions of a synced_block
        /// object: the original block that was created first and doesn't yet
        /// sync with anything else, and the duplicate block or blocks
        /// synced to the original.
        ///
        /// 📘 An original synced block must be created before corresponding
        /// duplicate block or blocks can be made. # Original synced block
        /// The value of `synced_from` is empty to signify that this is an
        /// original synced block that does not refer to another block.
        ///
        /// # Duplicate synced block
        /// The value of the synced_from is an object containing a [`BlockId`]
        synced_from: Option<BlockId>,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    // Table(Table),
    TableOfContents,
    // TableRow(TableRow),
    // Template(Template),
    ToDo {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// Whether the To do is checked.
        checked: bool,
        /// The color of the block.
        color: Color,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    Toggle {
        /// The rich text displayed in the block.
        rich_text: Vec<RichText>,
        /// The color of the block.
        color: Color,
        /// The nested children (if any).
        #[serde(default)]
        children: Option<Vec<Block>>,
    },
    Video {
        /// A file object that details information about the file contained in
        /// the block.
        file: FFile,
    },
    #[serde(other)]
    Unsupported,
}

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Emoji;
#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FFile;
#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum FileOrEmoji {
    Emoji(Emoji),
    File(FFile),
}

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum FileType {
    File,
    External,
}

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum MentionType {
    Database,
    Data,
    LinkPreview,
    Page,
    User,
}
#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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
#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
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
