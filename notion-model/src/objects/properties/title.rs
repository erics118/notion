use serde::{Deserialize, Serialize};

use super::RichText;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Title {
    pub title: Vec<RichText>,
}
