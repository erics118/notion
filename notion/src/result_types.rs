//! Internal module to store the results of the API calls.
//!
//! The API returns a JSON object with a `object` field that indicates the type
//! of the result. This module defines the types of the result and the
//! deserialization logic.
//!
//! For the user-facing API, we return the deserialized result or an error,
//! rather than a struct in this module.
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum Block {
    Block(crate::model::objects::block::Block),
    Error(crate::errors::ErrorInfo),
}

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum Page {
    Page(crate::model::objects::page::Page),
    Error(crate::errors::ErrorInfo),
}

#[derive(Deserialize)]
#[serde(tag = "object", rename_all = "snake_case")]
pub enum List<T> {
    List(crate::model::pagination::List<T>),
    Error(crate::errors::ErrorInfo),
}
