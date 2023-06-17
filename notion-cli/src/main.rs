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
    // unstable_features,
    // unused_crate_dependencies,
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
#![feature(lazy_cell)]

pub mod cli;
pub mod config;
pub mod error;

use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use notion::{client::Notion, model::ids::BlockId};

use crate::{
    cli::{Cli, Commands},
    config::{load_config, Config},
};

#[allow(unused)]
mod ids {
    use notion::model::ids::BlockId;

    pub const PARAGRAPH_BLOCK: &str = "d3d710f97c874e6c8e4d9b2576a6fb29";
    pub const TOGGLE_BLOCK: &str = "413085318c3741808899ada14b5e8095";
    pub const INTERNAL_FILE_BLOCK: &str = "20017e7c5a3e42858f92abf2ca237c55";
    pub const EXTERNAL_FILE_BLOCK: &str = "ea3b7f53f121486c9e129e2d54fdbc1f";
    pub const CALLOUT_BLOCK: &str = "7d7771b0b76442548e2c5d1ac8bbb617";

    pub const PAGE: &str = "67ace61a7fd24ab78e892b1dc9b252e4";
}

// TODO: can't add children using the builder because the builder uses
// `BlockBuilder`, but the structs require `Block`

#[tokio::main]
pub async fn main() -> Result<()> {
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token)?;

    // TODO: color does not work in a Block
    // but, color does work in a RichText

    let res = notion
        .retrieve_block(BlockId::from_str(ids::EXTERNAL_FILE_BLOCK)?)
        .await?;

    println!("{:#?}", res);

    Ok(())
}
