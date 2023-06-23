use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct Rollup {
    /// The function that is evaluated for every page in the relation of the
    /// rollup.
    pub function: RollupFunction,
    /// The value of the calculated rollup.
    #[serde(flatten)]
    pub data: RollupData,
}

/// The value of the calculated rollup.
/// TODO: complete all types
/// TODO: u32 is supposed to be a f64 as well
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RollupData {
    Array,
    Date,
    Incomplete,
    Number(u32),
    Unsupported,
}

/// Functions that can be used in a rollup property.
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum RollupFunction {
    Average,
    Checked,
    Count,
    CountPerGroup,
    CountValues,
    DateRange,
    EarliestDate,
    Empty,
    LatestDate,
    Max,
    Median,
    Min,
    NotEmpty,
    PercentChecked,
    PercentEmpty,
    PercentNotEmpty,
    PercentPerGroup,
    PercentUnchecked,
    Range,
    ShowOriginal,
    ShowUnique,
    Sum,
    Unchecked,
    Unique,
}
