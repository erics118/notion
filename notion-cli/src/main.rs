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
use notion::{
    client::Notion,
    model::{
        ids::BlockId,
        objects::{block::*, code_languages::CodeLanguage, color::Color, rich_text::RichText},
    },
};

use crate::{
    cli::{Cli, Commands},
    config::{load_config, Config},
};

#[allow(unused)]
mod ids {
    use notion::model::ids::BlockId;

    pub const PARAGRAPH_BLOCK: &str = "d3d710f97c874e6c8e4d9b2576a6fb29";
    pub const TOGGLE_BLOCK: &str = "413085318c3741808899ada14b5e8095";
    pub const PAGE: &str = "67ace61a7fd24ab78e892b1dc9b252e4";
}

pub async fn stuff() -> Result<()> {
    let cli = Cli::parse();
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token)?;

    match cli.command {
        Commands::RetrieveBlock { block_id } => {
            let block = notion.retrieve_block(BlockId::from_str(&block_id)?).await?;
            println!("{block:#?}");

            println!("{}", serde_json::to_string(&block)?);
        },
    }

    Ok(())
}

// TODO: can't add children using the builder because the builder uses
// `BlockBuilder`, but the structs require `Block`

#[tokio::main]
pub async fn main() -> Result<()> {
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token)?;

    // TODO: color does not work in a Block
    // but, color does work in a RichText

    // let res = notion
    //     .append_block_children(BlockId::from_str(ids::TOGGLE_BLOCK)?, children)
    //     .await?;

    let res = notion
        .delete_block(BlockId::from_str("9b5d5adca0b043c6904f570d3e423c66")?)
        .await?;

    println!("{:#?}", res);

    Ok(())
}

pub fn build() {
    // api suggests there is an `audio` block, probably unsupported

    let _bookmark = Bookmark::new()
        .url(Some("https://google.com/".to_string()))
        .build_block();

    let _breadcrumb = Breadcrumb::new().build_block();

    let _bulleted_item_list = BulletedListItem::new()
        .rich_text(vec![RichText::new_text("hi").color(Color::Blue)])
        .build_block();

    let _callout = Callout::new().build_block(); // TODO

    let _code = Code::new()
        .language(CodeLanguage::Markdown)
        .rich_text(vec![RichText::new_text("# heading\nthis is **bold** text")])
        .build_block();

    let _divider = Divider::new().build_block();

    let _embed = Embed::new()
        .url("https://www.youtube.com/embed/dQw4w9WgXcQ".to_string())
        .build_block();

    let _equation = Equation::new().build_block();

    let _heading1 = Heading1::new().build_block();

    let _heading2 = Heading2::new().build_block();

    let _heading3 = Heading3::new().build_block();

    let _quote = Quote::new()
        .rich_text(vec![RichText::new_text("hi").color(Color::Blue)])
        .build_block();

    let _table_of_contents = TableOfContents::new().build_block();

    let _toggle = Toggle::new().build_block();
}
