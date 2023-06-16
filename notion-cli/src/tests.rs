// TODO: move these tests to notion-model
// TODO: remove notion-cli
// TODO: make sure to have multiple test for each builder, with different conditions
// TODO: tests for RichText too
use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use clap::Parser;
use notion::{
    client::Notion,
    model::{
        ids::BlockId,
        objects::{block::*, code_languages::CodeLanguage, color::Color, rich_text::RichText},
    },
};

use crate::config::{load_config, Config};

// static NOTION: LazyLock<Notion> = LazyLock::new(|| {
//     let Config { api_token } = load_config().unwrap();
//     let notion = Notion::new(&api_token).unwrap();

//     notion
// });

pub mod builders {
    use notion::model::objects::{block::*, rich_text::RichText};

    #[test]
    fn bookmark() {
        let a = serde_json::to_string(
            &Bookmark::builder()
                .url("https://google.com/".to_string())
                .build(),
        )
        .unwrap();

        assert_eq!(
            a,
            r#"{"object":"block","type":"bookmark","bookmark":{"url":"https://google.com/"}}"#,
        );
    }

    #[test]
    fn paragraph() {
        let a = serde_json::to_string(
            &Heading2::builder()
                .rich_text(vec![RichText::new_text("Lacinato kale")])
                .build(),
        )
        .unwrap();

        assert_eq!(
            a,
            r#"{"object":"block","type":"heading_2","heading_2":{"rich_text":[{"type":"text","text":{"content":"Lacinato kale"}}]}}"#,
        );
    }
}