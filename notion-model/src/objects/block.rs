use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{color::Color, rich_text::RichText, user::UserMetadata};
use crate::ids::BlockId;

/// Fields common to all block types.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BlockMetadata {
    /// Always "block"
    // pub object = "block"

    /// Identifier for the block.
    pub id: BlockId,
    /// Information about the block's parent. See [`BlockParent`] for details.
    // pub parent: BlockParent,

    /// Type of block. See [`BlockType`] for details.
    // pub r#type: BlockType,

    /// Date and time when this block was created. Formatted as an ISO 8601 date
    /// time string.
    pub created_time: DateTime<Utc>,

    /// User who created the block.
    pub created_by: UserMetadata,
    /// Date and time when this block was last updated. Formatted as an ISO 8601
    /// date time string.
    pub last_edited_time: DateTime<Utc>,
    /// User who last edited the block.
    pub last_edited_by: UserMetadata,

    /// The archived status of the block.
    pub archived: bool,

    /// Whether or not the block has children blocks nested within it.
    pub has_children: bool,
    // pub {type}
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum Block {
    Bookmark(Bookmark),
    Breadcrumb(Breadcrumb),
    BulletedListItem(BulletedListItem),
    Callout(Callout),
    ChildDatabase(ChildDatabase),
    ChildPage(ChildPage),
    Column(Column),
    ColumnList(ColumnList),
    Divider(Divider),
    Embed(Embed),
    Equation(Equation),
    File(File),
    Heading1(Heading1),
    Heading2(Heading2),
    Heading3(Heading3),
    Image(Image),
    LinkPreview(LinkPreview),
    Mention(Mention),
    NumberedListItem(NumberedListItem),
    Paragraph(Paragraph),
    PDF(PDF),
    Quote(Quote),
    SyncedBlock(SyncedBlock),
    // Table(Table),
    TableOfContents(TableOfContents),
    // TableRow(TableRow),
    // Template(Template),
    ToDo(ToDo),
    Toggle(Toggle),
    Unsupported(Unsupported),
    Video(Video),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Emoji;
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct FFile;
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum FileOrEmoji {
    Emoji(Emoji),
    File(FFile),
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum FileType {
    File,
    External,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Bookmark {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The caption for the bookmark.
    pub caption: Vec<RichText>,
    /// The link for the bookmark.
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Breadcrumb {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BulletedListItem {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,

    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nested children (if any).
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Callout {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// An emoji or file object that represents the callout's icon.
    pub icon: FileOrEmoji,
    /// The color of the block.
    pub color: Color,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildDatabase {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPage {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Column {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnList {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Divider {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

/// Embed block objects include information about another website displayed
/// within the Notion UI.
///
/// # 🚧 Differences in embed blocks between the Notion app and the API
///
/// The Notion app uses a 3rd-party service, iFramely, to validate and request
/// metadata for embeds given a URL. This works well in a web app because Notion
/// can kick off an asynchronous request for URL information, which might take
/// seconds or longer to complete, and then update the block with the metadata
/// in the UI after receiving a response from iFramely.
///
/// We chose not to call iFramely when creating embed blocks in the API because
/// the API needs to be able to return faster than the UI, and because the
/// response from iFramely could actually cause us to change the block type.
/// This would result in a slow and potentially confusing experience as the
/// block in the response would not match the block sent in the request.
///
/// The result is that embed blocks created via the API may not look exactly
/// like their counterparts created in the Notion app.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Embed {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,

    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Equation {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// A KaTeX compatible string.
    pub expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct File {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The caption of the file block.
    pub caption: Vec<RichText>,
    /// The type of the file.
    pub type_: FileType,
    /// A file object that details information about the file contained in the
    /// block.
    pub file: FFile,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Heading1 {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// Whether or not the heading block is a toggle heading or not. If `true`,
    /// then the heading block toggles and can support children. If `false`,
    /// then the heading block is a static heading block.
    pub is_toggleable: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Heading2 {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// Whether or not the heading block is a toggle heading or not. If `true`,
    /// then the heading block toggles and can support children. If `false`,
    /// then the heading block is a static heading block.
    pub is_toggleable: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Heading3 {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// Whether or not the heading block is a toggle heading or not. If `true`,
    /// then the heading block toggles and can support children. If `false`,
    /// then the heading block is a static heading block.
    pub is_toggleable: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum SupportedImageTypes {
    BMP,
    GIF,
    HEIC,
    JPEG,
    JPG,
    PNG,
    SVG,
    TIF,
    TIFF,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Image {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// A file object that details information about the file contained in the
    /// block.
    pub file: FFile,
}

/// The [`LinkPreview`] block can only be returned as part of a response. The
/// API does not support creating or appending [`LinkPreview`] blocks.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreview {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The originally pasted url.
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum MentionType {
    Database,
    Data,
    LinkPreview,
    Page,
    User,
}

/// Aka LinkToPage
///
/// A mention block object is a child of a rich text object that is nested
/// within a paragraph block object. This block type represents any @ tag in the
/// Notion UI, for a user, date, Notion page, Notion database, or a miniaturized
/// version of a Link Preview.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Mention {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The type of mention.
    pub type_: MentionType,
    // An object with type-specific information about the mention.
    // TODO: pub object,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct NumberedListItem {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nexted child blocks (if it has any).
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Paragraph {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nexted child blocks (if it has any).
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PDF {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// A caption, if provided, for the PDF block.
    pub caption: Vec<RichText>,
    /// The type of the PDF.
    pub type_: FileType, /* file for internal, external for external
                          * then a { url:} */
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Quote {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nexted child blocks (if it has any).
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SyncedBlock {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// Similar to the Notion UI, there are two versions of a synced_block
    /// object: the original block that was created first and doesn't yet sync
    /// with anything else, and the duplicate block or blocks synced to the
    /// original.
    ///
    /// 📘 An original synced block must be created before corresponding
    /// duplicate block or blocks can be made. # Original synced block
    /// The value of `synced_from` is empty to signify that this is an original
    /// synced block that does not refer to another block.
    ///
    /// # Duplicate synced block
    /// The value of the synced_from is an object containing a [`BlockId`]
    pub synced_from: Option<BlockId>,
    /// The nexted child blocks (if it has any).
    pub children: Vec<Block>,
}

// TODO: table
// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// pub struct Table {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableOfContents {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

// TODO: tablerow
// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// pub struct TableRow {}

// TODO: template
// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// pub struct Template {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToDo {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// Whether the To do is checked.
    pub checked: bool,
    /// The color of the block.
    pub color: Color,
    /// The nexted child blocks (if it has any).
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Toggle {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// The rich text displayed in the block.
    pub rich_text: Vec<RichText>,
    /// The color of the block.
    pub color: Color,
    /// The nexted child blocks (if it has any).
    pub children: Vec<Block>,
}

pub enum SupportedVideoTypes {
    AMV,
    ASF,
    AVI,
    F4V,
    FLV,
    GIFV,
    MKV,
    MOV,
    MPG,
    MPEG,
    MPV,
    MP4,
    M4V,
    QT,
    WMV,
    YoutubeEmbed,
    YoutubeWatch,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Video {
    /// Metadata of the block.
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    /// A file object that details information about the file contained in the
    /// block.
    pub file: FFile,
}

/// The API does not spport all types of [`Block`]s, so everything that isn't
/// supported falls under here.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Unsupported {}
