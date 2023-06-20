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
/// Retrieve a page gets the identifier, type, and value of a page's properties.
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
pub struct Property {
    /// An underlying identifier for the property. id may be a UUID, but it's
    /// often a short random string.
    ///
    /// id may be used in place of name when creating or updating pages.
    ///
    /// id remains constant when the property name changes.
    pub id: PropertyId,
    /// A type object that contains data specific to the page property type,
    /// including the page property value.
    #[serde(flatten)]
    pub data: PropertyData,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PropertyData {
    Checkbox(bool),
    CreatedBy(PartialUser),
    CreatedTime(DateTime<Utc>),
    // Date(Date),
    Email(Option<String>),
    Files(Vec<File>),
    Formula(FormulaData),
    LastEditedBy(PartialUser),
    LastEditedTime(DateTime<Utc>),
    // MultiSelect(MultiSelect),
    // TODO: find max value of number
    Number(Option<u32>),
    People(Vec<PartialUser>),
    PhoneNumber(Option<String>),
    // Relation(Relation),
    // Rollup(Rollup),
    RichText(Vec<RichText>),
    // Select(Select),
    // Status(Status),
    Title(Vec<RichText>),
    Url(Option<String>),
    UniqueId(UniqueId),
}

/// TODO: remove tag, use a struct if possible
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FormulaData {
    Boolean { boolean: bool },
    Date { date: DateTime<Utc> },
    Number { number: u32 },
    String { string: String },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct UniqueId {
    pub prefix: String,
    pub number: u32,
}
