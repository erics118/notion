use serde::{Deserialize, Serialize};

// Creation of template blocks are no longer supported by the API
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy, Default)]
pub struct Template;
