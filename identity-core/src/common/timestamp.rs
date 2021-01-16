use chrono::{DateTime, SecondsFormat, Utc};
use core::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use crate::error::{Error, Result};

/// A parsed Timestamp.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// Parses a `Timestamp` from the provided input string.
    pub fn parse(input: &str) -> Result<Self> {
        DateTime::parse_from_rfc3339(input)
            .map_err(Into::into)
            .map(Into::into)
            .map(Self)
    }

    /// Creates a new `Timestamp` with the current date and time.
    pub fn now() -> Self {
        Self::parse(&Self::to_rfc3339(&Self(Utc::now()))).unwrap()
    }

    /// Returns the `Timestamp` as a Unix timestamp.
    pub fn to_unix(&self) -> i64 {
        self.0.timestamp()
    }

    /// Returns the `Timestamp` as an RFC 3339 `String`.
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339_opts(SecondsFormat::Secs, true)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl Debug for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{:?}", self.to_rfc3339())
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.to_rfc3339())
    }
}

impl From<Timestamp> for String {
    fn from(other: Timestamp) -> Self {
        other.to_rfc3339()
    }
}

impl TryFrom<&'_ str> for Timestamp {
    type Error = Error;

    fn try_from(string: &'_ str) -> Result<Self, Self::Error> {
        Self::parse(string)
    }
}

impl TryFrom<String> for Timestamp {
    type Error = Error;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        Self::parse(&string)
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::parse(string)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Timestamp;

    #[test]
    fn test_parse_valid() {
        let original = "2020-01-01T00:00:00Z";
        let timestamp = Timestamp::parse(original).unwrap();

        assert_eq!(timestamp.to_rfc3339(), original);

        let original = "1980-01-01T12:34:56Z";
        let timestamp = Timestamp::parse(original).unwrap();

        assert_eq!(timestamp.to_rfc3339(), original);
    }

    #[test]
    #[should_panic = "InvalidTimestamp"]
    fn test_parse_empty() {
        Timestamp::parse("").unwrap();
    }

    #[test]
    #[should_panic = "InvalidTimestamp"]
    fn test_parse_invalid_date() {
        Timestamp::parse("foo bar").unwrap();
    }

    #[test]
    #[should_panic = "InvalidTimestamp"]
    fn test_parse_invalid_fmt() {
        Timestamp::parse("2020/01/01 03:30:16").unwrap();
    }
}
