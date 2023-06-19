use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::{
    ids::{DatabaseId, PageId, UserId},
    objects::color::Color,
};

/// Rich text objects contain the data that Notion uses to display Notion
/// blocks, such as formatted text, mentions, and inline equations. Arrays of
/// rich text objects within database property objects and page property value
/// objects are used to create what a user experiences as a single text value in
/// Notion.
///
/// # 📘
///
/// Many block types support rich text. In cases where it is supported, a
/// rich_text object will be included in the block type object. All rich_text
/// objects will include a plain_text property, which provides a convenient way
/// for developers to access unformatted text from the Notion block.
///
/// # 📘 Rich text object limits
///
/// Refer to the request limits documentation page for information about [limits
/// on the size of rich text objects](https://developers.notion.com/reference/request-limits#limits-for-property-values).
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct RichText {
    /// The information used to style the rich text object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    /// The plain text without annotations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
    /// The URL of any link or Notion mention in this text, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// An object containing type-specific configuration.
    #[serde(flatten)]
    pub data: RichTextData,
}

impl RichText {
    pub fn new_text(text: &str) -> Self {
        Self {
            annotations: None,
            plain_text: None,
            href: None,
            data: RichTextData::Text {
                text: Text {
                    content: text.into(),
                    link: None,
                },
            },
        }
    }

    pub fn new_mention(mention: Mention) -> Self {
        Self {
            annotations: None,
            plain_text: None,
            href: None,
            data: RichTextData::Mention { mention },
        }
    }

    pub fn new_equation(expression: &str) -> Self {
        Self {
            annotations: None,
            plain_text: None,
            href: None,
            data: RichTextData::Equation {
                equation: Equation {
                    expression: expression.into(),
                },
            },
        }
    }

    pub fn text(mut self, text: Text) -> Self {
        self.data = RichTextData::Text { text };
        self
    }

    /// also directly provide wrappers for annotations
    pub fn annotations(mut self, annotations: Option<Annotations>) -> Self {
        self.annotations = annotations;
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.annotations.get_or_insert_with(Default::default).bold = bold;
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.annotations.get_or_insert_with(Default::default).italic = italic;
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.annotations
            .get_or_insert_with(Default::default)
            .strikethrough = strikethrough;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.annotations
            .get_or_insert_with(Default::default)
            .underline = underline;
        self
    }

    pub fn code(mut self, code: bool) -> Self {
        self.annotations.get_or_insert_with(Default::default).code = code;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.annotations.get_or_insert_with(Default::default).color = color;
        self
    }

    /// plain_text does not need to be set when making api calls
    pub fn plain_text(&mut self, plain_text: Option<String>) -> &mut Self {
        self.plain_text = plain_text;
        self
    }

    pub fn href(mut self, href: Option<String>) -> Self {
        self.href = href;
        self
    }
}

/// TODO: remove tag
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RichTextData {
    Text {
        text: Text,
    },
    Mention {
        mention: Mention,
    },
    Equation {
        equation: Equation,
    },
    #[default]
    Idk,
}

/// All rich text objects contain an annotations object that sets the styling
/// for the rich text.
///
/// This field does not need to be set when making API calls, where it will
/// default to the default values.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Annotations {
    /// Whether the text is bolded.
    pub bold: bool,
    /// Whether the text is italicized.
    pub italic: bool,
    /// Whether the text is struck through.
    pub strikethrough: bool,
    /// Whether the text is underlined.
    pub underline: bool,
    /// Whether the text is code.
    pub code: bool,
    /// The color of the text.
    pub color: Color,
}

/// # Equation object
///
/// Notion supports inline LaTeX equations as rich text objects.
///
/// For some reason, the Notion API documents this
/// at <https://developers.notion.com/reference/block#equation>, but this is
/// incorrect, as an Equation is not a block, but a type of rich text object.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Equation {
    /// The LaTeX string representing the inline equation.
    pub expression: String,
}
/// # Mention object
///
/// Mention objects represent an inline mention of a database, date, link
/// preview mention, page, template mention, or user. A mention is created in
/// the Notion UI when a user types `@` followed by the name of the reference.
///
/// TODO: remove tag
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Mention {
    Database { database: DatabaseMention },
    Date { date: DateMention },
    LinkPreview { link_preview: LinkPreviewMention },
    Page { page: PageMention },
    Template { template: TemplateMention },
    User { user: UserMention },
}

/// # Database mention object
///
/// Database mentions contain a database reference within the corresponding
/// database field. A database reference is an object with an id key and a
/// string value (UUIDv4) corresponding to a database id.
///
/// If an integration doesn't have access to the mentioned database, then the
/// mention is returned with just the id. The plain_text value that would be a
/// title appears as "Untitled" and the annotation object's values are defaults.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DatabaseMention {
    /// The id of the database.
    pub id: DatabaseId,
}

impl DatabaseMention {
    pub fn new(id: DatabaseId) -> Self {
        Self { id }
    }

    pub fn id(mut self, id: DatabaseId) -> Self {
        self.id = id;
        self
    }

    pub fn build(self) -> Mention {
        Mention::Database { database: self }
    }
}

/// Date mentions contain a [date property value](https://developers.notion.com/reference/property-value-object#date-property-values) object within the corresponding
/// date field.
///
/// TODO: somehow allow for both `chrono::NaiveDate` and `chrono::NaiveDateTime`
/// TODO: for some reason using NaiveDateTime causes an error: trailing input
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DateMention {
    /// An ISO 8601 format date, with optional time.
    start: DateTime<Utc>,
    /// An ISO 8601 formatted date, with optional time. Represents the end of a
    /// date range.
    ///
    /// If `None`, this property's date value is not a range.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    end: Option<DateTime<Utc>>,
    /// Time zone information for start and end. Possible values are extracted
    /// from the IANA database and they are based on the time zones from
    /// Moment.js.
    ///
    /// When time zone is provided, start and end should not have any UTC
    /// offset. In addition, when time zone is provided, start and end cannot be
    /// dates without time information.
    ///
    /// If `None`, time zone information will be contained in UTC offsets in
    /// start and end.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "time_zone")]
    timezone: Option<Tz>,
}

impl DateMention {
    pub fn new(start: DateTime<Utc>) -> Self {
        Self {
            start,
            end: None,
            timezone: None,
        }
    }

    pub fn start(mut self, start: DateTime<Utc>) -> Self {
        self.start = start;
        self
    }

    pub fn end(mut self, end: Option<DateTime<Utc>>) -> Self {
        self.end = end;
        self
    }

    pub fn timezone(mut self, timezone: Option<Tz>) -> Self {
        self.timezone = timezone;
        self
    }

    pub fn build(self) -> Mention {
        Mention::Date { date: self }
    }
}
/// If a user opts to share a Link Preview as a mention, then the API handles
/// the Link Preview mention as a rich text object with a type value of
/// link_preview. Link preview rich text mentions contain a corresponding
/// link_preview object that includes the url that is used to create the Link
/// Preview mention.
///
/// There is no builder for LinkPreview because there is no documentation on it.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct LinkPreviewMention {
    /// If a user opts to share a Link Preview as a mention, then the API
    /// handles the Link Preview mention as a rich text object with a type value
    /// of link_preview. Link preview rich text mentions contain a corresponding
    /// link_preview object that includes the url that is used to create the
    /// Link Preview mention.
    pub url: String,
}

/// Page mentions contain a page reference within the corresponding page field.
/// A page reference is an object with an id property and a string value
/// (UUIDv4) corresponding to a page ID.
///
/// If an integration doesn't have access to the mentioned page, then the
/// mention is returned with just the ID. The plain_text value that would be a
/// title appears as "Untitled" and the annotation object's values are defaults.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PageMention {
    /// The id of the page.
    pub id: PageId,
}

impl PageMention {
    pub fn new(id: PageId) -> Self {
        Self { id }
    }

    pub fn id(mut self, id: PageId) -> Self {
        self.id = id;
        self
    }

    pub fn build(self) -> Mention {
        Mention::Page { page: self }
    }
}

/// The content inside a template button in the Notion UI can include
/// placeholder date and user mentions that populate when a template is
/// duplicated. Template mention type objects contain these populated values.
///
/// Template mention rich text objects contain a template_mention object with a
/// nested type key that is either "template_mention_date" or
/// "template_mention_user".
/// no template builder because template building is not supported by the API
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateMention {
    /// The type of the date mention. Possible values include: "today" and
    /// "now".
    #[serde(rename = "template_mention_date")]
    Date(String),
    /// The type of the user mention. The only possible value is "me".
    #[serde(rename = "template_mention_user")]
    User(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct UserMention {
    /// If a rich text object's type value is "user", then the corresponding
    /// user field contains a user object.
    ///
    /// # 📘
    ///
    /// If your integration doesn't yet have access to the mentioned user,
    /// then the plain_text that would include a user's name reads as
    /// "@Anonymous". To update the integration to get access to the user,
    /// update the integration capabilities on the integration settings page.
    pub id: UserId,
}

impl UserMention {
    pub fn new(id: UserId) -> Self {
        Self { id }
    }

    pub fn id(mut self, id: UserId) -> Self {
        self.id = id;
        self
    }

    pub fn build(self) -> Mention {
        Mention::User { user: self }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct Text {
    /// The plain text without annotations.
    pub content: String,
    /// An object with information about any inline link in this text, if
    /// included.
    ///
    /// If the text contains an inline link, then the object key is url and the
    /// value is the URL's string web address.
    ///
    /// If the text doesn't have any inline links, then the value is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Default)]
pub struct Link {
    /// The URL of the link.
    pub url: String,
}
