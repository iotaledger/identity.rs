use identity_diff::{self as diff, Diff, string::DiffString};
use chrono::{DateTime, SecondsFormat, Utc};
use core::{convert::TryFrom, fmt, ops::Deref};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

type Inner = DateTime<Utc>;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Timestamp(Inner);

impl Timestamp {
    pub fn from_str(string: &str) -> Result<Self> {
        match DateTime::parse_from_rfc3339(string) {
            Ok(datetime) => Ok(Self(datetime.into())),
            Err(error) => Err(Error::InvalidTimestamp(error)),
        }
    }

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
        Self::from_str(string)
    }
}

impl Diff for Timestamp {
    type Type = DiffString;

    fn diff(&self, other: &Self) -> Result<Self::Type, diff::Error> {
        self.to_rfc3339().diff(&other.to_rfc3339())
    }

    fn merge(&self, diff: Self::Type) -> Result<Self, diff::Error> {
        let time: String = self.to_rfc3339().merge(diff)?;

        let time: Self = Self::from_str(time.as_str())
            .map_err(|error| diff::Error::MergeError(format!("{}", error)))?;

        Ok(time)
    }

    fn from_diff(diff: Self::Type) -> Result<Self, diff::Error> {
        let time: String = String::from_diff(diff)?;

        let time: Self =  Self::from_str(time.as_str())
            .map_err(|error| diff::Error::MergeError(format!("{}", error)))?;

        Ok(time)
    }

    fn into_diff(self) -> Result<Self::Type, diff::Error> {
        self.to_rfc3339().into_diff()
    }
}
