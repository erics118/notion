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

// TODO: parse the response from the API into a struct

#[tokio::main]
pub async fn main() -> Result<()> {
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token)?;

    // TODO: color does not work in a Block
    // but, color does work in a RichText

    // api suggests there is an `audio` block, probably unsupported

    let _bookmark = Bookmark::builder()
        .url("https://google.com/".to_string())
        .build();

    let _breadcrumb = Breadcrumb::builder().build();

    let _bulleted_item_list = BulletedListItem::builder()
        .rich_text(vec![RichText::new_text("hi").color(Color::Blue)])
        .build();

    let _callout = Callout::builder().build(); // TODO

    let _code = Code::builder()
        .language(CodeLanguage::Markdown)
        .rich_text(vec![RichText::new_text("# heading\nthis is **bold** text")])
        .build();

    let _divider = Divider::builder().build();

    let _embed = Embed::builder()
        .url("https://www.youtube.com/embed/dQw4w9WgXcQ".to_string())
        .build();

    let _equation = Equation::builder().build();

    let _heading1 = Heading1::builder().build();

    let _heading2 = Heading2::builder().build();

    let _heading3 = Heading3::builder().build();

    let _quote = Quote::builder()
        .rich_text(vec![RichText::new_text("hi").color(Color::Blue)])
        .build();

    let _table_of_contents = TableOfContents::builder().build();

    let _toggle = Toggle::builder().build();

    let children = vec![_heading2; 2];

    let res = notion
        .append_block_children(BlockId::from_str(ids::TOGGLE_BLOCK)?, children)
        .await?;

    // let res = notion
    //     .retrieve_block(BlockId::from_str("d3d710f97c874e6f8e4d9b2576a6fb29")?)
    //     .await?;

    println!("{:#?}", res);
    Ok(())
}
