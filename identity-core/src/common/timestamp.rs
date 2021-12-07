// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;

use core::str::FromStr;
use time::format_description::well_known::Rfc3339;
use time::Duration;
use time::OffsetDateTime;

use crate::error::Error;
use crate::error::Result;

/// A parsed Timestamp.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Timestamp(OffsetDateTime);

impl Timestamp {
  /// Parses a `Timestamp` from the provided input string.
  pub fn parse(input: &str) -> Result<Self> {
    let offset_date_time = OffsetDateTime::parse(input, &Rfc3339).map_err(time::Error::from)?;
    Ok(Timestamp(truncate(offset_date_time)))
  }

  /// Creates a new `Timestamp` with the current date and time.
  pub fn now_utc() -> Self {
    Self(truncate(OffsetDateTime::now_utc()))
  }

  /// Returns the `Timestamp` as an RFC 3339 `String`.
  ///
  /// See: https://tools.ietf.org/html/rfc3339
  pub fn to_rfc3339(&self) -> String {
    // expect is OK, because all constructors provide well defined RFC 3339 compatible OffSetDateTime
    // if this is to be made fallible instead then we need to change some things in our interface such as the
    // From<Timestamp> for String implementation.
    self
      .0
      .format(&Rfc3339)
      .expect("unexpected Timestamp incompatibility with RFC 3339")
  }

  /// Returns the `Timestamp` as a Unix timestamp.
  pub fn to_unix(&self) -> i64 {
    self.0.unix_timestamp()
  }

  /// Creates a new `Timestamp` from the given Unix timestamp.
  pub fn from_unix(seconds: i64) -> Result<Self> {
    let offset_date_time = OffsetDateTime::from_unix_timestamp(seconds).map_err(time::error::Error::from)?;
    // Reject years outside of the range 0000AD - 9999AD per Rfc3339
    // upfront to prevent conversion errors in to_rfc3339().
    // https://datatracker.ietf.org/doc/html/rfc3339#section-1
    if !(0..10_000).contains(&offset_date_time.year()) {
      return Err(time::error::Error::Format(time::error::Format::InvalidComponent("invalid year")).into());
    }
    Ok(Self(offset_date_time))
  }
}

impl Default for Timestamp {
  fn default() -> Self {
    Self::now_utc()
  }
}

impl Debug for Timestamp {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{:?}", self.to_rfc3339())
  }
}

impl Display for Timestamp {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(f, "{}", self.to_rfc3339())
  }
}

impl From<Timestamp> for String {
  fn from(timestamp: Timestamp) -> Self {
    timestamp.to_rfc3339()
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

/// Truncates an `OffsetDateTime` to the second.
fn truncate_fractional_seconds(offset_date_time: OffsetDateTime) -> OffsetDateTime {
  offset_date_time - Duration::nanoseconds(offset_date_time.nanosecond() as i64)
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
    let time1: Timestamp = Timestamp::now_utc();
    let json: Vec<u8> = time1.to_json_vec().unwrap();
    let time2: Timestamp = Timestamp::from_json_slice(&json).unwrap();

    assert_eq!(time1, time2);
  }
}
