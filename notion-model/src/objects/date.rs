use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Dates in notion have an optional time component.
/// Since chrono does not support optional time components, we use this enum to
/// represent it.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum DateOrDateTime {
    Date(NaiveDate),
    DateTime(DateTime<FixedOffset>),
}

impl Serialize for DateOrDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Date(date) => serializer.serialize_str(&date.format("%Y-%m-%d").to_string()),
            Self::DateTime(datetime) => serializer.serialize_str(&datetime.to_rfc3339()),
        }
    }
}

impl<'de> Deserialize<'de> for DateOrDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
            Ok(Self::DateTime(dt))
        } else if let Ok(d) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            Ok(Self::Date(d))
        } else {
            Err(serde::de::Error::custom(format!(
                "invalid date format: {}",
                s
            )))
        }
    }
}

impl ToString for DateOrDateTime {
    fn to_string(&self) -> String {
        match self {
            Self::Date(date) => date.format("%Y-%m-%d").to_string(),
            Self::DateTime(datetime) => datetime.to_rfc3339(),
        }
    }
}

impl FromStr for DateOrDateTime {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
            Ok(Self::DateTime(dt))
        } else if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Ok(Self::Date(d))
        } else {
            Err(anyhow::anyhow!("invalid date format: {}", s))
        }
    }
}

// TODO: more tests
#[cfg(test)]
mod test {
    use chrono::{DateTime, NaiveDate, Utc};

    use super::DateOrDateTime;

    #[test]
    fn date() {
        let date = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let date_or_datetime = DateOrDateTime::Date(date).to_string();

        assert_eq!(date_or_datetime, "2021-01-01");
    }

    #[test]
    fn datetime() {
        let datetime = NaiveDate::from_ymd_opt(2021, 1, 1)
            .unwrap()
            .and_hms_opt(12, 34, 56)
            .unwrap();

        let datetime2: DateTime<Utc> = DateTime::from_utc(datetime, Utc);
        let date_or_datetime = DateOrDateTime::DateTime(datetime2.into()).to_string();

        assert_eq!(date_or_datetime, "2021-01-01T12:34:56+00:00");
    }
}
