use serde::{Deserialize, Serialize};

/// Block color
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default]
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    GreenBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
}

impl Color {
    pub fn is_default(&self) -> bool {
        let default: Self = Default::default();
        self == &default
    }
}

/// Option color
///
/// Only has the foreground colors, no background colors.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OptionColor {
    #[default]
    Default,
    Gray,
    Brown,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Red,
}

impl OptionColor {
    pub fn is_default(&self) -> bool {
        let default: Self = Default::default();
        self == &default
    }
}
