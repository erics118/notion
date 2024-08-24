#![forbid(unsafe_code)]
#![warn(
    absolute_paths_not_starting_with_crate,
    missing_copy_implementations,
    missing_debug_implementations,
    unused_qualifications,
    clippy::all,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]

pub use notion_model as model;

pub mod client;
pub mod errors;
pub mod utils;

pub(crate) mod result_types;

mod authentication;
mod blocks;
mod comments;
mod databases;
mod pages;
mod search;
mod users;

#[allow(unused)]
pub(crate) fn test_json() -> String {
    String::from_utf8_lossy(include_bytes!("../../test_data/block.json")).to_string()
}
