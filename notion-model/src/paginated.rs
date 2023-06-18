//! # Pagination
//!
//! Endpoints that return lists of objects support cursor-based pagination
//! requests. By default, Notion returns ten items per API call. If the number
//! of items in a response from a support endpoint exceeds the default, then an
//! integration can use pagination to request a specific set of the results
//! and/or to limit the number of returned items.

use serde::{Deserialize, Serialize};

/// Paginated list of blocks
///
/// see https://developers.notion.com/reference/intro#parameters-for-paginated-requests
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(tag = "object", rename = "list", rename_all = "snake_case")]
pub struct List<T> {
    ///     Whether the response includes the end of the list. false if there
    /// are no more results. Otherwise, true.
    pub has_more: bool,
    /// A string that can be used to retrieve the next page of results by
    /// passing the value as the start_cursor parameter to the same endpoint.
    ///
    /// Only available when `has_more` is true.
    pub next_cursor: Option<String>,
    /// The list, or partial list, of endpoint-specific results. Refer to a
    /// supported endpoint's individual documentation for details.
    pub results: Vec<T>,
    /// An object containing type-specific pagination information.
    ///
    /// For `property_items`, the value corresponds to the paginated page
    /// property type. For all other types, the value is an empty object.
    #[serde(flatten)]
    pub data: ListData,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ListData {
    Block,
    Comment,
    Database,
    Page,
    PageOrDatabase,
    PropertyItem { property_item: PropertyItem },
    User,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct PropertyItem {}
