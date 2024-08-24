use formula::FormulaData;
use serde::{Deserialize, Serialize};

use super::{block::File, date::DateOrDateTime, rich_text::PageMention, user::PartialUser};
use crate::ids::PropertyId;

mod checkbox;
// mod created_by;
// mod created_time;
mod date;
mod email;
// mod files;
mod formula;
// mod last_edited_time;
mod multi_select;
mod number;
mod people;
mod phone_number;
mod relation;
mod rich_text;
mod rollup;
mod select;
mod status;
mod title;
mod unique_id;
mod url;

pub use checkbox::Checkbox;
// pub use created_by::CreatedBy;
// pub use created_time::CreatedTime;
pub use date::Date;
pub use email::Email;
// pub use files::Files;
//* pub use formula::Formula;
// pub use last_edited_time::LastEditedTime;
// pub use last_edited_by::LastEditedBy;
//* pub use multi_select::MultiSelect;
pub use number::Number;
//* pub use people::People;
pub use phone_number::PhoneNumber;
//* pub use relation::Relation;
pub use rich_text::RichText;
pub use rollup::Rollup;
pub use select::{Select, SelectOption};
pub use status::{Status, StatusOption};
pub use title::Title;
pub use unique_id::UniqueId;
pub use url::Url;

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
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Property {
    /// An underlying identifier for the property. id may be a UUID, but it's
    /// often a short random string.
    ///
    /// id may be used in place of name when creating or updating pages.
    ///
    /// id remains constant when the property name changes.
    ///
    /// Does not need to be set when creating or editing a page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<PropertyId>,
    /// A type object that contains data specific to the page property type,
    /// including the page property value.
    #[serde(flatten)]
    pub data: PropertyData,
    /// Only applies to `Relation`
    ///
    /// If a relation has more than 25 references, then the has_more value for
    /// the relation in the response object is true. If a relation doesn't
    /// exceed the limit, then has_more is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl Property {
    pub fn new(data: PropertyData) -> Self {
        Self {
            id: None,
            data,
            has_more: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PropertyData {
    /// Simple checkboxes that indicate whether something is done or
    /// not.
    Checkbox(Checkbox),
    /// Automatically records the user who created the item.
    ///
    /// A user object containing information about the user who created the
    /// page.
    CreatedBy(PartialUser),
    /// Timestamps an item's creation.
    ///
    /// The date and time that the page was created.
    ///
    /// The created_time value can't be updated.
    CreatedTime(DateOrDateTime),
    /// Accepts a date or a date range, allowing you to timestamp and set
    /// reminders.
    Date(Date),
    /// Accepts email addresses and launches your mail client when clicked.
    ///
    /// A string describing an email address.
    Email(Email),
    /// Allows you to upload files relevant to your database
    /// item.
    ///
    /// An array of objects containing information about the files.
    ///
    /// # 📘
    ///
    /// The Notion API does not yet support uploading files to Notion.
    ///
    /// # 📘
    ///
    /// When updating a file page property value, the value is overwritten by
    /// the array of files passed.
    ///
    /// Although Notion doesn't support uploading files via the API, if you pass
    /// a file object containing a file hosted by Notion, it remains one of the
    /// files. To remove any file, just don't pass it in the update response.
    Files(Vec<File>),
    /// Lets you perform calculations or trigger actions based on other
    /// properties.
    ///
    /// The value of the result of the formula.
    ///
    /// Formula property value objects represent the result of evaluating a
    /// formula described in the database's properties.
    ///
    /// # 📘
    ///
    /// The Retrieve a page endpoint returns a maximum of 25 inline page or
    /// person references for a formula property. If a formula property includes
    /// more than 25 references, then you can use the Retrieve a page property
    /// item endpoint for the specific formula property to get its complete list
    /// of references.
    Formula(FormulaData),
    /// Records the user who edited the item last.
    ///
    /// A user object containing information about the user who last updated the
    /// page.
    ///
    /// last_edited_by can't be updated.
    LastEditedBy(PartialUser),
    /// Timestamps an item's last edit.
    ///
    /// The date and time that the page was last edited.
    ///
    /// The last_edited_time value can't be updated.
    LastEditedTime(DateOrDateTime),
    /// Dropdown menu of tags letting you add more than one at a
    /// time.
    ///
    /// # 📘
    ///
    ///  If you want to add a new option to a multi-select property via the
    /// Update page or Update database endpoint, then your integration needs
    /// write access to the parent database.
    MultiSelect(Vec<SelectOption>),
    /// Numerical formats like currencies and percentages. Useful for price,
    /// etc.
    ///
    /// A number representing some value.
    // TODO: figure out how to use int/float
    Number(Number),
    /// Lets you mention other users in your workspace (or assign them
    /// to things).
    ///
    /// An array of user objects.
    ///
    /// # 📘
    /// The Retrieve a page endpoint can't be guaranteed to return more than 25
    /// people per people page property. If a people page property includes more
    /// than 25 people, then you can use the Retrieve a page property item
    /// endpoint for the specific people property to get a complete list of
    /// people.
    People(Vec<PartialUser>),
    /// Accepts a phone number and prompts your phone or computer to call
    /// it when clicked.
    ///
    /// A string representing a phone number. No phone number format is
    /// enforced.
    PhoneNumber(PhoneNumber),
    /// Lets you add items from another database as a property.
    ///
    /// An array of related page references.
    ///
    /// # 📘
    /// To update a relation property value via the API, share the related
    /// parent database with the integration.
    ///
    /// # 📘
    /// If a relation property value is unexpectedly empty, then make sure that
    /// you have shared the original source database that the relation points to
    /// with the integration.
    Relation(Vec<PageMention>),
    /// Runs calculations based on properties in a related database.
    ///
    /// # 🚧
    /// For rollup properties with more than 25 references, use the Retrieve a
    /// page property endpoint
    ///
    /// Both the Retrieve a page and Retrieve a page property endpoints will
    /// return information related to the page properties. In cases where a
    /// rollup property has more than 25 references, the Retrieve a page
    /// property endpoint must but used.
    ///
    /// # 🚧
    /// The API does not support updating rollup page property values.
    ///
    /// To change a page's rollup property, use the Notion UI.
    ///
    /// # 📘
    /// The Retrieve a page endpoint returns a maximum of 25 populated inline
    /// page or person references for a rich_text property. If a rich_text
    /// property includes more than 25 references, then you can use the Retrieve
    /// a page property item endpoint for the specific rich_text property to get
    /// its complete list of references.
    // TODO: rollup is broken
    Rollup(Rollup),
    /// Basic text for notes, descriptions and comments about database items.
    RichText(RichText),
    /// Select: Dropdown menu of tags that can be selected one at a time.
    Select(Select),
    /// Status: Dropdown menu of tags that are grouped by status (i.e. To-do, In
    /// Progress, Complete).
    ///
    /// Functions basically the same as a Select, except they are sorted into 3
    /// categories in the UI, and the value must be set.
    Status(Status),
    /// Whatever you're calling your item, i.e. the title of the page in your
    /// database.
    ///
    /// An array of rich text objects, but no rich text actually is visible from
    /// the UI.
    Title(Title),
    /// Accepts a link to a website relevant to your database item.
    ///
    /// A string that describes a web address.
    Url(Url),
    /// A short, unique id for each item in a database. This is not the same
    /// as the `id` property of the page object.
    UniqueId(UniqueId),
}
