use chrono::{DateTime, NaiveDateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::{
    ids::{DatabaseId, PageId, UserId},
    objects::color::Color,
};

/// 📘 Rich text object limits
///
/// Refer to the request limits documentation page for information about limits
/// on the size of rich text objects.
///
/// Many block types support rich text. In cases where it is supported, a
/// rich_text object will be included in the block type object. All rich_text
/// objects will include a plain_text property, which provides a convenient way
/// for developers to access unformatted text from the Notion block.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
pub struct RichText {
    /// An object containing type-specific configuration.
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
    pub fn new_text(plain_text: impl Into<String>) -> Self {
        let a = plain_text.into();
        Self {
            annotations: None,
            plain_text: None,
            href: None,
            data: RichTextData::Text {
                text: Text {
                    content: a,
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

    pub fn new_equation(expression: String) -> Self {
        Self {
            annotations: None,
            plain_text: None,
            href: None,
            data: RichTextData::Equation {
                equation: Equation { expression },
            },
        }
    }

    pub fn text(mut self, text: Text) -> Self {
        self.data = RichTextData::Text { text };
        self
    }

    /// directly provide wrappers for annotations
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
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
    ///
    /// If the color is [`Color::Default`], then the color is inherited from
    /// the parent.
    pub color: Color,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Equation {
    /// The LaTeX string representing the inline equation.
    pub expression: String,
}

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DatabaseMention {
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

// TODO: somehow allow for both `chrono::NaiveDate` and `chrono::NaiveDateTime`
// TODO: for some reason using NaiveDateTime causes an error: trailing input
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
/// No builder for LinkPreview bc API doesn't support it
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct PageMention {
    /// Page mentions contain a page reference within the corresponding page
    /// field. A page reference is an object with an id property and a string
    /// value (UUIDv4) corresponding to a page ID.
    ///
    /// If an integration doesn’t have access to the mentioned page, then the
    /// mention is returned with just the ID. The plain_text value that would be
    /// a title appears as "Untitled" and the annotation object’s values are
    /// defaults.
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

/// no template builder because template building is not supported by the API
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateMention {
    /// The type of the date mention. Possible values include: "today" and
    /// "now".
    Date(String),
    /// The type of the user mention. The only possible value is "me".
    User(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct UserMention {
    /// If a rich text object’s type value is "user", then the corresponding
    /// user field contains a user object.
    ///
    /// 📘 If your integration doesn’t yet have access to the mentioned user,
    /// then the plain_text that would include a user’s name reads as
    /// "@Anonymous". To update the integration to get access to the user,
    /// update the integration capabilities on the integration settings page.
    pub id: UserId,
    // pub name: String,
    // pub avatar_url: Option<String>,
    // #[serde(flatten)]
    // pub data: UserMentionData,
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Text {
    /// The plain text without annotations.
    pub content: String,
    /// An array of rich text objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
