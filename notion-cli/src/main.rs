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

pub mod cli;
pub mod config;
pub mod error;

#[allow(unused)]
use std::collections::HashMap;
#[allow(unused)]
use std::str::FromStr;

#[allow(unused)]
use anyhow::{Context, Result};
use notion::model::objects::{
    date::DateOrDateTime,
    properties::{Select, SelectOption, StatusOption},
    rich_text::DateMention,
};
#[allow(unused)]
use notion::model::objects::{page::Page, parent::ParentData, properties};
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
    pub const DATABASE: &str = "fe45735bf4dc420695eb1be8d96b2184";
    pub const DATABASE_PAGE: &str = "228a649d2e844f7fa57535a9f2c4debf";
}

/// TODO: write tests for all block struct buixlders
/// TODO: test all builders and make sure they work, from the app and API
/// TODO: make sure all block structs have builder function for everything
/// TODO: use &[] instead of vec![] everywhere
#[tokio::main]
pub async fn main() -> Result<()> {
    let Config { api_token } = load_config()?;
    let notion = Notion::new(&api_token).context("Failed to create api client")?;

    // let res = notion
    //     .create_page(
    //         Page::new()
    //             .parent(Some(ParentData::from(DatabaseId::from_str(
    //                 "45089d19e5b84b82b1dd7cc9f93559b4",
    //             )?)))
    //             .properties(HashMap::from([
    //                 properties::Title::new("hello
    // world2").build_with_name("Name"),
    // properties::Select::new(SelectOption {                     name:
    // Some("Homework".to_string()),                     color: None,
    //                     id: None,
    //                 })
    //                 .build_with_name("Type"),
    //                 properties::Select::new(SelectOption {
    //                     name: Some("Spanish".to_string()),
    //                     color: None,
    //                     id: None,
    //                 })
    //                 .build_with_name("Subject"),
    //                 properties::Status::new(StatusOption {
    //                     name: Some("To Review".to_string()),
    //                     color: None,
    //                     id: None,
    //                 })
    //                 .build_with_name("Status"),
    //             ])),
    //     )
    //     .await?;

    let res = notion
        .retrieve_page(PageId::from_str(ids::DATABASE_PAGE)?, None)
        .await?;

    // let res = notion
    //     .retrieve_database(DatabaseId::from_str("
    // 45089d19e5b84b82b1dd7cc9f93559b4")?)     .await?;

    // let res = notion
    // .retrieve_page(PageId::from_str("67ace61a7fd24ab78e892b1dc9b252e4")?, None)
    // .await?;

    println!("{:#?}", res);

    // let value = Bookmar::new().build();
    // println!("{}", serde_json::to_string(&value).unwrap());

    Ok(())
}
