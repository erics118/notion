use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::{
    ids::{DatabaseId, PageId, UserId},
    objects::color::Color,
};

/// The types of rich text objects.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]

pub enum RichTextType {
    #[default]
    Text,
    Mention,
    Equation,
}

/// 📘 Rich text object limits
///
/// Refer to the request limits documentation page for information about limits
/// on the size of rich text objects.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
pub struct RichText {
    // /// The type of this rich text object.
    #[serde(rename = "type")]
    pub type_: RichTextType,
    /// An object containing type-specific configuration.
    pub annotations: Annotations,
    /// The plain text without annotations.
    pub plain_text: String,
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
            type_: RichTextType::Text,
            annotations: Annotations::default(),
            plain_text: String::default(),
            href: None,
            data: RichTextData::Text {
                text: Text {
                    content: a,
                    link: None,
                },
            },
        }
    }

    // pub fn new_mention(plain_text: String, href: String, mention_type:
    // MentionType) -> Self {     Self {
    //         type_: RichTextType::Mention,
    //         annotations: Annotations::default(),
    //         plain_text: String::default(),
    //         href: Some(href),
    //         data: RichTextData::Mention(Mention {
    //             mention: MentionType::User,
    //         }),
    //     }
    // }

    // pub fn new_equation(plain_text: String, expression: String) -> Self {
    //     Self {
    //         type_: RichTextType::Equation,
    //         annotations: Annotations::default(),
    //         plain_text: String::default(),
    //         href: None,
    //         data: RichTextData::Equation(Equation { expression }),
    //     }
    // }

    pub fn text(mut self, text: Text) -> Self {
        self.data = RichTextData::Text { text };
        self
    }

    /// directly provide wrappers for annotations
    // pub fn annotations(mut self, annotations: Annotations) -> Self {
    //     self.annotations = annotations;
    //     self
    // }

    pub fn bold(mut self, bold: bool) -> Self {
        self.annotations.bold = bold;
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.annotations.italic = italic;
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.annotations.strikethrough = strikethrough;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.annotations.underline = underline;
        self
    }

    pub fn code(mut self, code: bool) -> Self {
        self.annotations.code = code;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.annotations.color = color;
        self
    }

    /// plain_text does not need to be set when making api calls
    // pub fn plain_text(&mut self, plain_text: String) -> &mut Self {
    //     self.plain_text = plain_text;
    //     self
    // }

    pub fn href(mut self, href: String) -> Self {
        self.href = Some(href);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase", untagged)]
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
#[serde(rename_all = "lowercase")]
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
#[serde(rename_all = "lowercase")]
pub struct Equation {
    /// The LaTeX string representing the inline equation.
    pub expression: String,
}

#[derive(Copy, Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MentionType {
    Database,
    Data,
    LinkPreview,
    Page,
    User,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mention {
    Database(DatabaseMention),
    Date(DateMention),
    LinkPreview(LinkPreviewMention),
    Page(PageMention),
    Template(TemplateMention),
    User(UserMention),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct DatabaseMention {
    pub id: DatabaseId,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct DateMention {
    /// An ISO 8601 format date, with optional time.
    start: DateTime<Utc>,
    /// An ISO 8601 formatted date, with optional time. Represents the end of a
    /// date range.
    ///
    /// If null, this property's date value is not a range.
    end: Option<DateTime<Utc>>,
    /// Time zone information for start and end. Possible values are extracted
    /// from the IANA database and they are based on the time zones from
    /// Moment.js.
    ///
    /// When time zone is provided, start and end should not have any UTC
    /// offset. In addition, when time zone is provided, start and end cannot be
    /// dates without time information.
    ///
    /// If null, time zone information will be contained in UTC offsets in start
    /// and end.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "time_zone")]
    timezone: Option<Tz>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct LinkPreviewMention {
    /// If a user opts to share a Link Preview as a mention, then the API
    /// handles the Link Preview mention as a rich text object with a type value
    /// of link_preview. Link preview rich text mentions contain a corresponding
    /// link_preview object that includes the url that is used to create the
    /// Link Preview mention.
    pub url: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
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

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TemplateMention {
    /// The type of the date mention. Possible values include: "today" and
    /// "now".
    Date(String),
    /// The type of the user mention. The only possible value is "me".
    User(String),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct UserMention {
    /// If a rich text object’s type value is "user", then the corresponding
    /// user field contains a user object.
    ///
    /// 📘 If your integration doesn’t yet have access to the mentioned user,
    /// then the plain_text that would include a user’s name reads as
    /// "@Anonymous". To update the integration to get access to the user,
    /// update the integration capabilities on the integration settings page.
    pub user: UserId,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct Text {
    /// The plain text without annotations.
    pub content: String,
    /// An array of rich text objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
