use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{block::File, rich_text::RichText, user::PartialUser};
use crate::ids::PropertyId;

/// # Page properties
///
/// A page object is made up of page properties that contain data about the
/// page.
///
/// When you send a request to Create a page, you set the page properties in the
/// properties object body param.
///
/// Retrieve a page gets the identifier, type, and value of a page’s properties.
/// Retrieve a page property item returns information about a single property
/// ID.
///
/// An Update page query modifies the page property values specified in the
/// properties object body param.
///
/// Size limits for page property values
///
/// For information about size limitations for specific page property objects,
/// refer to the limits for property values documentation.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
struct Property {
    /// An underlying identifier for the property. id may be a UUID, but it’s
    /// often a short random string.
    ///
    /// id may be used in place of name when creating or updating pages.
    ///
    /// id remains constant when the property name changes.
    pub id: PropertyId,
    /// A type object that contains data specific to the page property type,
    /// including the page property value.
    pub data: PropertyData,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PropertyData {
    Checkbox {
        id: PropertyId,
        checkbox: bool,
    },
    CreatedBy {
        id: PropertyId,
        created_by: PartialUser,
    },
    CreatedTime {
        id: PropertyId,
        created_time: DateTime<Utc>,
    },
    // Date { date: Date },
    Email {
        id: PropertyId,
        email: Option<String>,
    },
    Files {
        id: PropertyId,
        files: Vec<File>,
    },
    Formula {
        id: PropertyId,
        // #[serde(flatten)]
        formula: FormulaData,
    },
    LastEditedBy {
        id: PropertyId,
        last_edited_by: PartialUser,
    },
    LastEditedTime {
        id: PropertyId,
        last_edited_time: DateTime<Utc>,
    },
    // MultiSelect { multi_select: MultiSelect },
    // TODO: see max
    Number {
        id: PropertyId,
        number: Option<u32>,
    },
    People {
        id: PropertyId,
        people: Vec<PartialUser>,
    },
    PhoneNumber {
        id: PropertyId,
        phone_number: Option<String>,
    },
    // Relation { relation: Relation },
    // Rollup { rollup: Rollup },
    RichText {
        id: PropertyId,
        rich_text: Vec<RichText>,
    },
    // Select { select: Select },
    // Status { status: Status },
    Title {
        id: PropertyId,
        title: Vec<RichText>,
    },
    Url {
        id: PropertyId,
        url: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormulaData {
    Boolean { boolean: bool },
    Date { date: DateTime<Utc> },
    Number { number: u32 },
    String { string: String },
}
