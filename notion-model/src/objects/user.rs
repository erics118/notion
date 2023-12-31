use serde::{Deserialize, Serialize};

use crate::ids::UserId;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(tag = "object")]
pub struct PartialUser {
    pub id: UserId,
}
