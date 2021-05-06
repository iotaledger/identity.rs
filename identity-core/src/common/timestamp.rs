// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::SecondsFormat;
use chrono::Timelike;
use chrono::Utc;
use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::str::FromStr;

use crate::error::Error;
use crate::error::Result;

/// A parsed Timestamp.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
  /// Parses a `Timestamp` from the provided input string.
  pub fn parse(input: &str) -> Result<Self> {
    let this: DateTime<Utc> = DateTime::parse_from_rfc3339(input)?.into();
    let this: DateTime<Utc> = Self::truncate(this);

    Ok(Self(this))
  }

  /// Creates a new `Timestamp` with the current date and time.
  pub fn now() -> Self {
    Self(Self::truncate(Utc::now()))
  }

  /// Returns the `Timestamp` as a Unix timestamp.
  #[allow(clippy::wrong_self_convention)]
  pub fn to_unix(&self) -> i64 {
    self.0.timestamp()
  }

  /// Creates a new `Timestamp` from the given Unix timestamp.
  pub fn from_unix(seconds: i64) -> Self {
    Self(DateTime::from_utc(NaiveDateTime::from_timestamp(seconds, 0), Utc))
  }

  /// Returns the `Timestamp` as an RFC 3339 `String`.
  #[allow(clippy::wrong_self_convention)]
  pub fn to_rfc3339(&self) -> String {
    self.0.to_rfc3339_opts(SecondsFormat::Secs, true)
  }

  fn truncate(value: DateTime<Utc>) -> DateTime<Utc> {
    // safe to unwrap because 0 is a valid nanosecond
    value.with_nanosecond(0).unwrap()
  }
}

impl Default for Timestamp {
  fn default() -> Self {
    Self::now()
  }
}

impl Debug for Timestamp {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{:?}", self.to_rfc3339())
  }
}

impl Display for Timestamp {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
  use crate::convert::FromJson;
  use crate::convert::ToJson;

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
  fn test_parse_valid_truncated() {
    let original = "1980-01-01T12:34:56.789Z";
    let expected = "1980-01-01T12:34:56Z";
    let timestamp = Timestamp::parse(original).unwrap();

    assert_eq!(timestamp.to_rfc3339(), expected);
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

  #[test]
  fn test_json_roundtrip() {
    let time1: Timestamp = Timestamp::now();
    let json: Vec<u8> = time1.to_json_vec().unwrap();
    let time2: Timestamp = Timestamp::from_json_slice(&json).unwrap();

    assert_eq!(time1, time2);
  }
}
