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
        objects::{block::{*, synced_block::SyncedFrom}, rich_text::RichText},
    },
};

use crate::config::{load_config, Config};

#[allow(unused)]
mod ids {
    use notion::model::ids::BlockId;

    pub const PARAGRAPH_BLOCK: &str = "d3d710f97c874e6c8e4d9b2576a6fb29";

    pub const TOGGLE_BLOCK: &str = "413085318c3741808899ada14b5e8095";

    pub const FILE_INTERNAL_BLOCK: &str = "20017e7c5a3e42858f92abf2ca237c55";
    pub const FILE_EXTERNAL_BLOCK: &str = "ea3b7f53f121486c9e129e2d54fdbc1f";

    pub const CALLOUT_INTERNAL_AKA_IMAGE_BLOCK: &str = "7d7771b0b76442548e2c5d1ac8bbb617";
    pub const CALLOUT_EMOJI_BLOCK: &str = "1a3c9a923fdc4e79a57e0c9fcf65d092";
    pub const CALLOUT_ICON_BLOCK: &str = "eccbd66c5b584823b14b7229c5a39e17";

    pub const MENTION_LINK_PREVIEW_BLOCK: &str = "5075f75611ed49a7a06a4e00288a94d5";
    pub const MENTION_DATE_BLOCK: &str = "7038faa8cd454a3baff1182e3fe662ee"; // doesn't work, bc DateTime<Utc> needs a time
    pub const MENTION_DATE_TIME_BLOCK: &str = "85f28b47c81f4fb4b27bd330be2f1423";
    pub const MENTION_DATE_TIME_START_END_BLOCK: &str = "ccabb0b6afec440787ae12906ab832c6";
    pub const MENTION_USER_BLOCK: &str = "22eea0fa1e40486b9cea173bb944939f";

    pub const IMAGE_BLOCK: &str = "61af5174fdf24726921517e6f0d05132";
    pub const IMAGE_BLOCK2: &str = "c3c3214bd57d4068bebe0c4ff2de1476";

    pub const CODE_BLOCK: &str = "6e9612c81c7d4356ba9153eab009e6f4";

    pub const PDF_BLOCK: &str = "3e12bcd5ae6a4c97a8eb22baf4462f2a";

    pub const VIDEO_BLOCK: &str = "cf3482826bbc401abbca94147543211f";

    pub const PAGE: &str = "67ace61a7fd24ab78e892b1dc9b252e4";

    pub const COLUMN1: &str = "8a85ea985af84fae87c9a1fbe22f0b92";
    pub const COLUMN_PARENT: &str = "e3c4733e-0349-48f8-afee-31838e08abed";
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token)?;

    let res = notion
        .append_block_children(
            BlockId::from_str(ids::PAGE)?,
            vec![
                SyncedBlock::new()
                    .synced_from(Some(SyncedFrom {
                        block_id: BlockId::from_str("140029e0-2894-4f99-be08-61b64e0bf3a8")?,
                    }))
                    .children(Some(vec![
                        Paragraph::new()
                            .rich_text(vec![RichText::new_text("Hello World")])
                            .build(),
                    ]))
                    .build(),
            ],
        )
        .await?;

    println!("{:#?}", res);

    Ok(())
}
