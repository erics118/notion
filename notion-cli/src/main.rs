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
    single_use_lifetimes,
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
    variant_size_differences,
    explicit_outlives_requirements,
    elided_lifetimes_in_paths,
    unused_qualifications,
    clippy::all,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used
)]

pub mod cli;
pub mod config;
pub mod error;

#[allow(unused)]
use std::str::FromStr;

#[allow(unused)]
use anyhow::{Context, Result};
#[allow(unused)]
use notion::{
    client::Notion,
    model::{
        ids::*,
        objects::{block::*, rich_text::RichText},
    },
    utils,
};

use crate::config::{load_config, Config};

#[allow(unused)]
mod ids {
    pub const PARAGRAPH: &str = "d3d710f97c874e6c8e4d9b2576a6fb29";
    pub const PAGE: &str = "67ace61a7fd24ab78e892b1dc9b252e4";
    pub const USER: &str = "3e1fc0f5d02e48ae84c07ae06deece9f";
    pub const DATABASE: &str = "41ef19e3335a434281c95cbec7345198";
    pub const DATABASE_PAGE: &str = "1866cccaa3f842c68cd8e5b0aabeebc8";
}

// TODO: test all block structs for children, from the API, and from the app
// TODO: make sure all block structs have builder function for everything
// TODO: maybe somehow force ColumnList to have Column as children
// TODO: maybe somehow force Table to have TableRow as children
#[tokio::main]
pub async fn main() -> Result<()> {
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token)?;

    let res = notion
        .append_block_children(
            BlockId::from_str(ids::PAGE)?,
            vec![
                Heading1::new()
                    .rich_text(vec![RichText::new_text("Hello, world!")])
                    .build(),
            ],
        )
        .await?;

    println!("{:#?}", res);

    Ok(())
}
