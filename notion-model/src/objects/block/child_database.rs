use serde::{Deserialize, Serialize};

/// # Child database
///
/// # 📘 Creating and updating child_database blocks
///
/// To create or update child_database type blocks, use the Create a database
/// and the Update a database endpoints, specifying the ID of the parent page in
/// the parent body param.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildDatabase {
    /// The plain text title of the database.
    pub title: String,
}
