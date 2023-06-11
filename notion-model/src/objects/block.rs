use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{color::Color, rich_text::RichText, user::UserMetadata};
use crate::ids::BlockId;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BlockMetadata {
    // pub object = "block"
    pub id: BlockId,
    // pub parent: Box<BlockParent>,
    // pub r#type: BlockType,
    pub created_time: DateTime<Utc>,
    pub created_by: UserMetadata,
    pub last_edited_time: DateTime<Utc>,
    pub last_edited_by: UserMetadata,
    pub archived: bool,
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
    LinkToPage(LinkToPage),
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
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub caption: Vec<RichText>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Breadcrumb {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct BulletedListItem {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub color: Color,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Callout {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub icon: FileOrEmoji,
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
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ColumnList {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Divider {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Embed {
    #[serde(flatten)]
    pub metadata: BlockMetadata,

    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Equation {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub expression: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct File {
    #[serde(flatten)]
    pub metadata: BlockMetadata,

    pub caption: String,
    pub type_: FileType,
    pub file: FFile,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Heading1 {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub color: Color,
    pub is_toggleable: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Heading2 {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub color: Color,
    pub is_toggleable: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Heading3 {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub color: Color,
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
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    // File object
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkPreview {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
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

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct LinkToPage {
    pub type_: MentionType,
    // object-specific information
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct NumberedListItem {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub color: Color,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Paragraph {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub color: Color,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct PDF {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub caption: String,
    pub type_: String, /* file for internal, external for external
                        * then a { url:} */
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Quote {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub color: Color,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct SyncedBlock {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub synced_from: Option<BlockId>,
    pub children: Vec<Block>,
}

// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// pub struct Table {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct TableOfContents {}

// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// pub struct TableRow {}

// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// pub struct Template {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ToDo {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub checked: bool,
    pub color: Color,
    pub children: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Toggle {
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    pub rich_text: Vec<RichText>,
    pub color: Color,
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
    #[serde(flatten)]
    pub metadata: BlockMetadata,
    // file object
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Unsupported {}
