use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    block::File, date::DateOrDateTime, file_and_emoji::FileOrEmoji, page_properties::Property,
    parent::ParentData, user::PartialUser,
};
use crate::ids::PageId;

/// All pages have a Parent. If the parent is a database, the property values
/// conform to the schema laid out database's properties. Otherwise, the only
/// property value is the title.
///
/// Page content is available as blocks. The content can be read using retrieve
/// block children and appended using append block children.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(tag = "object", rename_all = "snake_case", rename = "page")]
pub struct Page {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateOrDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<PartialUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_time: Option<DateOrDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_by: Option<PartialUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// emoji or external, can't be internal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<FileOrEmoji>,
    /// file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<File>,
    pub properties: HashMap<String, Property>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ParentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_url: Option<String>,
}

impl Page {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Option<PageId>) -> Self {
        self.id = id;
        self
    }

    pub fn archived(mut self, archived: Option<bool>) -> Self {
        self.archived = archived;
        self
    }

    pub fn icon(mut self, icon: Option<FileOrEmoji>) -> Self {
        self.icon = icon;
        self
    }

    pub fn cover(mut self, cover: Option<File>) -> Self {
        self.cover = cover;
        self
    }

    pub fn properties(mut self, properties: HashMap<String, Property>) -> Self {
        self.properties = properties;
        self
    }

    pub fn parent(mut self, parent: Option<ParentData>) -> Self {
        self.parent = parent;
        self
    }
}
