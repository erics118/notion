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
        objects::{block::Heading2, color::Color, rich_text::RichText},
    },
};

use crate::{
    cli::{Cli, Commands},
    config::{load_config, Config},
};

pub const PARAGRAPH_BLOCK_ID: &str = "d3d710f97c874e6c8e4d9b2576a6fb29";
pub const TOGGLE_BLOCK_ID: &str = "413085318c3741808899ada14b5e8095";
pub const PAGE_ID: &str = "67ace61a7fd24ab78e892b1dc9b252e4";

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

#[tokio::main]
pub async fn main() -> Result<()> {
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token)?;

    // let children: Vec<BlockBuilder> = vec![BlockBuilder::new(BlockData::Heading2
    // {     heading_2: Heading2 {
    //         rich_text: vec![RichText {
    //             type_: RichTextType::Text,
    //             plain_text: "Hello, world!".to_string(),
    //             data: RichTextData::Text(Text {
    //                 content: "Hello, world!".to_string(),
    //                 ..Default::default()
    //             }),
    //             ..Default::default()
    //         }],
    //         ..Default::default()
    //     },
    // })];

    let children = vec![
        Heading2::new()
            .rich_text(vec![RichText::new_text("Hello, World!")])
            .color(Color::Brown)
            .new_block(),
    ];

    notion
        .append_block_children(BlockId::from_str(TOGGLE_BLOCK_ID)?, children)
        .await?;

    Ok(())
}
