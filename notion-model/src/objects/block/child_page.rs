use serde::{Deserialize, Serialize};

/// # Child page
///
/// # 📘 Creating and updating child_page blocks
///
/// To create or update child_page type blocks, use the Create a page and the
/// Update page endpoints, specifying the ID of the parent page in the parent
/// body param.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct ChildPage {
    /// The plain text title of the page.
    pub title: String,
}
