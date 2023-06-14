use serde::{Deserialize, Serialize};

use crate::objects::color::Color;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct Callout {
    /// An emoji or file object that represents the callout's icon.
    pub icon: super::FileOrEmoji,
    /// The color of the block.
    pub color: Color,
}
