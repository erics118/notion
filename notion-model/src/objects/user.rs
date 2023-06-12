use monostate::MustBe;
use serde::{Deserialize, Serialize};

use crate::ids::UserId;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct PartialUser {
    object: MustBe!("user"),
    pub id: UserId,
}
