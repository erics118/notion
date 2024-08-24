//! there are three locations for property docs:
//! Database properties object: <https://developers.notion.com/reference/property-object>
//! Page properties: <https://developers.notion.com/reference/page-property-values>
//! Property values: <https://developers.notion.com/reference/property-value-object>

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    block::File, date::DateOrDateTime, file_and_emoji::FileOrEmoji, parent::ParentData,
    properties::Property, rich_text::RichText, user::PartialUser,
};
use crate::ids::DatabaseId;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[serde(tag = "object", rename_all = "snake_case", rename = "database")]
pub struct Database {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<DatabaseId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateOrDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<PartialUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_time: Option<DateOrDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_by: Option<PartialUser>,
    pub title: Vec<RichText>,
    pub description: Vec<RichText>,
    /// emoji or external, can't be internal
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub icon: Option<FileOrEmoji>,
    /// file
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub cover: Option<File>,
    // pub properties: HashMap<String, Property>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ParentData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_trash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_url: Option<String>,
}

impl Database {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: Option<DatabaseId>) -> Self {
        self.id = id;
        self
    }

    // pub fn archived(mut self, archived: Option<bool>) -> Self {
    //     self.archived = archived;
    //     self
    // }

    // pub fn icon(mut self, icon: Option<FileOrEmoji>) -> Self {
    //     self.icon = icon;
    //     self
    // }

    // pub fn cover(mut self, cover: Option<File>) -> Self {
    //     self.cover = cover;
    //     self
    // }

    // pub fn title(mut self, title: Vec<RichText>) -> Self {
    //     self.title = title;
    //     self
    // }

    // pub fn description(mut self, description: Vec<RichText>) -> Self {
    //     self.description = description;
    //     self
    // }

    // pub fn properties(mut self, properties: HashMap<String, Property>) -> Self {
    //     self.properties = properties;
    //     self
    // }

    // pub fn parent(mut self, parent: Option<ParentData>) -> Self {
    //     self.parent = parent;
    //     self
    // }
}
