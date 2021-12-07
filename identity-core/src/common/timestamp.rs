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
use time::UtcOffset;

use crate::error::Error;
use crate::error::Result;

/// A parsed Timestamp.
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
#[serde(try_from = "String", into = "String")]
pub struct Timestamp(OffsetDateTime);

impl Timestamp {
  /// Parses a `Timestamp` from the provided input string, normalized to UTC+00:00 with fractional
  /// seconds truncated.
  ///
  /// See the [`datetime` DID-core specification](https://www.w3.org/TR/did-core/#production).
  pub fn parse(input: &str) -> Result<Self> {
    let offset_date_time = OffsetDateTime::parse(input, &Rfc3339)
      .map_err(time::Error::from)?
      .to_offset(UtcOffset::UTC);
    Ok(Timestamp(truncate_fractional_seconds(offset_date_time)))
  }

  /// Creates a new `Timestamp` with the current date and time, normalized to UTC+00:00 with
  /// fractional seconds truncated.
  ///
  /// See the [`datetime` DID-core specification](https://www.w3.org/TR/did-core/#production).
  pub fn now_utc() -> Self {
    Self(truncate_fractional_seconds(OffsetDateTime::now_utc()))
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
  use quickcheck::Arbitrary;
  use quickcheck_macros::quickcheck;
  const LAST_VALID_UNIX_TIMESTAMP: i64 = 253402300799;
  const FIRST_VALID_UNIX_TIMESTAMP: i64 = -62167219200;
  use crate::common::Timestamp;
  use crate::convert::FromJson;
  use crate::convert::ToJson;

  #[derive(Debug, Clone)]
  struct ValidUnixTimestampFixture(i64);

  impl Arbitrary for ValidUnixTimestampFixture {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
      // collect 10_000 numbers between FIRST_VALID_UNIX_TIMESTAMP and LAST_VALID_UNIX_TIMESTAMP
      // such that the distance between any two consecutive numbers is the same.
      let interval_length = (LAST_VALID_UNIX_TIMESTAMP - FIRST_VALID_UNIX_TIMESTAMP) / 10_000;
      let markers: Vec<i64> = (FIRST_VALID_UNIX_TIMESTAMP..LAST_VALID_UNIX_TIMESTAMP)
        .step_by(interval_length as usize)
        .collect();
      let random_marker = *g.choose(&markers).unwrap();
      // add another randomly chosen number between 1 and 1000 for extra randomness
      let first_thousand: Vec<i64> = (1..=1000).collect();
      let random_number = *g.choose(&first_thousand).unwrap();
      Self(random_marker + random_number)
    }
  }

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
  fn test_parse_valid_offset_normalised() {
    let original = "1937-01-01T12:00:27.87+00:20";
    let expected = "1937-01-01T11:40:27Z";
    let timestamp = Timestamp::parse(original).unwrap();

    assert_eq!(timestamp.to_rfc3339(), expected);
  }

  #[test]
  fn test_from_unix_zero_to_rfc3339() {
    let unix_epoch = Timestamp::from_unix(0).unwrap();
    assert_eq!(unix_epoch.to_rfc3339(), "1970-01-01T00:00:00Z");
  }

  #[test]
  fn test_from_unix_invalid_edge_cases() {
    assert!(Timestamp::from_unix(LAST_VALID_UNIX_TIMESTAMP + 1).is_err());
    assert!(Timestamp::from_unix(FIRST_VALID_UNIX_TIMESTAMP - 1).is_err());
  }

  #[test]
  fn test_from_unix_to_rfc3339_boundaries() {
    let beginning = Timestamp::from_unix(FIRST_VALID_UNIX_TIMESTAMP).unwrap();
    let end = Timestamp::from_unix(LAST_VALID_UNIX_TIMESTAMP).unwrap();
    assert_eq!(beginning.to_rfc3339(), "0000-01-01T00:00:00Z");
    assert_eq!(end.to_rfc3339(), "9999-12-31T23:59:59Z");
  }

  #[quickcheck]
  fn test_valid_random_values_from_unix_to_rfc3339(value: ValidUnixTimestampFixture) -> bool {
    let timestamp = Timestamp::from_unix(value.0).unwrap();
    let expected_length = "dddd-dd-ddTdd:dd:ddZ".len();
    timestamp.to_rfc3339().len() == expected_length
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
