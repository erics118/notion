//! notion-api
//! Library to interact with the Notion API.
//!
//! # Introduction
//! The reference is your key to a comprehensive understanding of the Notion
//! API.
//!
//! Integrations use the API to access Notion's pages, databases, and users.
//! Integrations can connect services to Notion and build interactive
//! experiences for users within Notion. Using the navigation on the left,
//! you'll find details for objects and endpoints used in the API.
//!
//! ## 📘
//! You need an integration token to interact with the Notion API. You can find
//! an integration token after you create an integration on the integration
//! settings page. If this is your first look at the Notion API, we recommend
//! beginning with the Getting started guide to learn how to create an
//! integration.
//!
//! If you want to work on a specific integration, but can't access the token,
//! confirm that you are an admin in the associated workspace. You can check
//! inside the Notion UI via Settings & Members in the left sidebar. If you're
//! not an admin in any of your workspaces, you can create a personal workspace
//! for free.
//!
//! # Conventions
//!
//! The base URL to send all API requests is <https://api.notion.com>. HTTPS is
//! required for all API requests.
//!
//! The Notion API follows RESTful conventions when possible, with most
//! operations performed via GET, POST, PATCH, and DELETE requests on page and
//! database resources. Request and response bodies are encoded as JSON.
//!
//! ## JSON conventions
//!
//! Top-level resources have an "object" property. This property can be used to
//! determine the type of the resource (e.g. "database", "user", etc.)
//! Top-level resources are addressable by a UUIDv4 "id" property. You may omit
//! dashes from the ID when making requests to the API, e.g. when copying the ID
//! from a Notion URL.
//! Property names are in snake_case (not camelCase or kebab-case).
//! Temporal values (dates and datetimes) are encoded in ISO 8601 strings.
//! Datetimes will include the time value (2020-08-12T02:12:33.231Z) while dates
//! will include only the date (2020-08-12)
//! The Notion API does not support empty strings. To unset a string value for
//! properties like a url Property value object, for example, use an explicit
//! null instead of "".

#![forbid(unsafe_code)]
#![warn(
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    // missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    // single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    // variant_size_differences,
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    unused_qualifications,
    clippy::all,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::option_if_let_else)]

pub mod constants;
pub mod ids;
pub mod objects;
pub mod pagination;
