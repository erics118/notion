use serde::{Deserialize, Serialize};

use crate::ids::PropertyId;

pub mod checkbox;
pub mod created_by;
pub mod created_time;
pub mod date;
pub mod email;
pub mod files;
pub mod formula;
pub mod last_edited_by;
pub mod last_edited_time;
pub mod multi_select;
pub mod number;
pub mod people;
pub mod phone_number;
pub mod relation;
pub mod rich_text;
pub mod rollup;
pub mod select;
pub mod status;
pub mod title;
pub mod url;

pub use checkbox::Checkbox;
pub use created_by::CreatedBy;
pub use created_time::CreatedTime;
pub use date::Date;
pub use email::Email;
pub use files::Files;
pub use formula::Formula;
pub use last_edited_by::LastEditedBy;
pub use last_edited_time::LastEditedTime;
pub use multi_select::MultiSelect;
pub use number::Number;
pub use people::People;
pub use phone_number::PhoneNumber;
pub use relation::Relation;
pub use rich_text::RichText;
pub use rollup::Rollup;
pub use select::Select;
pub use status::Status;
pub use title::Title;
pub use url::Url;

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
#[serde(tag = "type")]
pub enum PropertyData {
    Checkbox { checkbox: Checkbox },
    // CreatedBy { created_by: CreatedBy },
    // CreatedTime { created_time: CreatedTime },
    // Date { date: Date },
    // Email { email: Email },
    // Files { files: Files },
    // Formula { formula: Formula },
    // LastEditedBy { last_edited_by: LastEditedBy },
    // LastEditedTime { last_edited_time: LastEditedTime },
    // MultiSelect { multi_select: MultiSelect },
    // Number { number: Number },
    // People { people: People },
    // PhoneNumber { phone_number: PhoneNumber },
    // Relation { relation: Relation },
    // Rollup { rollup: Rollup },
    // RichText { rich_text: RichText },
    // Select { select: Select },
    // Status { status: Status },
    // Title { title: Title },
    // Url { url: Url },
}
