use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{file_and_emoji::FileOrEmoji, parent::BlockParent, user::PartialUser, properties::PropertyData};

/// All pages have a Parent. If the parent is a database, the property values
/// conform to the schema laid out database's properties. Otherwise, the only
/// property value is the title.
///
/// Page content is available as blocks. The content can be read using retrieve
/// block children and appended using append block children.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "object")]
pub struct Page {
    pub id: String,
    pub created_time: DateTime<Utc>,
    pub created_by: PartialUser,
    pub last_edited_time: DateTime<Utc>,
    pub last_edited_by: PartialUser,
    pub archived: bool,
    /// emoji or external, can't be internal
    pub icon: Option<FileOrEmoji>,
    /// emoji or external, can't be internal
    pub cover: Option<FileOrEmoji>,
    pub properties: HashMap<String, PropertyData>,
    pub parent: BlockParent,
    pub url: String,
    pub public_url: String,
}
