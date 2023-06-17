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
use notion::{
    client::Notion,
    model::{
        ids::BlockId,
        objects::{block::*, rich_text::RichText, color::Color},
    },
};

use crate::config::{load_config, Config};

#[allow(unused)]
mod ids {
    pub const PAGE: &str = "67ace61a7fd24ab78e892b1dc9b252e4";
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token)?;

    let res = notion
        .append_block_children(
            BlockId::from_str(ids::PAGE)?,
            vec![
                Table::new()
                    .table_width(10)
                    .row_header(true)
                    .column_header(false)
                    .children(Some(vec![
                        TableRow::new()
                            .cells(vec![vec![RichText::new_text("1"), RichText::new_text("fsdfsa").color(Color::BlueBackground)]; 10])
                            .build(),
                        TableRow::new()
                            .cells(vec![vec![RichText::new_text("1")]; 10])
                            .build(),
                    ]))
                    .build(),
            ],
        )
        .await?;

    println!("{:#?}", res);

    Ok(())
}
