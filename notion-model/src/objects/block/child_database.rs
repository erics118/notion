use serde::{Deserialize, Serialize};

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
//     pub fn build(&self) -> Result<BlockBuilder> {
//         Ok(BlockBuilder::new(BlockData::ChildDatabase {
//             child_database: self.0.clone(),
//         }))
//     }

//     pub fn title(mut self, title: impl Into<String>) -> Self {
//         self.0.title = title.into();
//         self
//     }
// }
