use serde::{Deserialize, Serialize};

/// cannot create a child page block, as child pages are children of pages,
/// not children of blocks.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPage {
    pub title: String,
}

// impl ChildPage {
//     pub fn builder() -> ChildPageBuilder {
//         ChildPageBuilder(Self {
//             title: String::new(),
//         })
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// pub struct ChildPageBuilder(ChildPage);

// impl ChildPageBuilder {
//     pub fn build(&self) -> BlockBuilder {
//         Ok(BlockBuilder::new(BlockData::ChildPage {
//             child_page: self.0.clone(),
//         }))
//     }

//     pub fn title(mut self, title: String) -> Self {
//         self.0.title = title;
//         self
//     }
// }
