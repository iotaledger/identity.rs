use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt, ops::Deref};

use crate::error::Error;

type Inner = DateTime<Utc>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Timestamp(Inner);

impl Timestamp {
    /// Creates a new `Timestamp` of the current time.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Consumes the `Timestamp` and returns the inner `DateTime`.
    pub fn into_inner(self) -> Inner {
        self.0
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

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.to_rfc3339())
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

impl From<Inner> for Timestamp {
    fn from(other: Inner) -> Self {
        Self(other)
    }
}

impl From<Timestamp> for Inner {
    fn from(other: Timestamp) -> Self {
        other.into_inner()
    }
}

impl Deref for Timestamp {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&'_ str> for Timestamp {
    type Error = Error;

    fn try_from(string: &'_ str) -> Result<Self, Self::Error> {
        match DateTime::parse_from_rfc3339(string) {
            Ok(datetime) => Ok(Self(datetime.into())),
            Err(error) => Err(Error::InvalidTimestamp(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! error_hack {
        ($expr:expr) => {
            match $expr {
                Ok(_) => {}
                Err(error) => panic!("{}", error),
            }
        };
    }

    #[test]
    fn test_parse_valid() {
        let original = "2020-01-01T00:00:00Z";
        let timestamp = Timestamp::try_from(original).unwrap();

        assert_eq!(timestamp.to_rfc3339(), original);

        let original = "1980-01-01T12:34:56Z";
        let timestamp = Timestamp::try_from(original).unwrap();

        assert_eq!(timestamp.to_rfc3339(), original);
    }

    #[test]
    #[should_panic = "premature end of input"]
    fn test_parse_invalid_empty() {
        error_hack!(Timestamp::try_from(""));
    }

    #[test]
    #[should_panic = "input contains invalid characters"]
    fn test_parse_invalid_bad_date() {
        error_hack!(Timestamp::try_from("foo bar"));
    }

    #[test]
    #[should_panic = "input contains invalid characters"]
    fn test_parse_invalid_bad_fmt() {
        error_hack!(Timestamp::try_from("2020/01/01 03:30:16"));
    }
}
