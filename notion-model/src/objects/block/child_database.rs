use serde::{Deserialize, Serialize};

/// cannot create a child database block, as child dbs are children of pages,
/// not children of blocks.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildDatabase {
    pub title: String,
}

// impl ChildDatabase {
//     pub fn builder() -> ChildDatabaseBuilder {
//         ChildDatabaseBuilder(Self {
//             title: String::new(),
//         })
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// pub struct ChildDatabaseBuilder(ChildDatabase);

// impl ChildDatabaseBuilder {
//     pub fn build(&self) -> BlockBuilder {
//         Ok(BlockBuilder::new(BlockData::ChildDatabase {
//             child_database: self.0.clone(),
//         }))
//     }

//     pub fn title(mut self, title: impl Into<String>) -> Self {
//         self.0.title = title.into();
//         self
//     }
// }
