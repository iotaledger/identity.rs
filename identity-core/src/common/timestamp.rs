// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Debug;
use core::fmt::Display;
use core::fmt::Formatter;

use crate::diff;
use core::str::FromStr;
use identity_diff::Diff;
use identity_diff::DiffString;
use time::format_description::well_known::Rfc3339;
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
  #[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
  pub fn now_utc() -> Self {
    Self(truncate_fractional_seconds(OffsetDateTime::now_utc()))
  }

  /// Creates a new `Timestamp` with the current date and time, normalized to UTC+00:00 with
  /// fractional seconds truncated.
  ///
  /// See the [`datetime` DID-core specification](https://www.w3.org/TR/did-core/#production).
  #[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
  pub fn now_utc() -> Self {
    let milliseconds_since_unix_epoch: i64 = js_sys::Date::now() as i64;
    let seconds: i64 = milliseconds_since_unix_epoch / 1000;
    // expect is okay, we assume the current time is between 0AD and 9999AD
    Self::from_unix(seconds).expect("Timestamp failed to convert system datetime")
  }

  /// Returns the `Timestamp` as an RFC 3339 `String`.
  ///
  /// See: https://tools.ietf.org/html/rfc3339
  pub fn to_rfc3339(&self) -> String {
    // expect is okay, constructors ensure RFC 3339 compatible timestamps.
    // Making this fallible would break our interface such as From<Timestamp> for String.
    self.0.format(&Rfc3339).expect("Timestamp incompatible with RFC 3339")
  }

  /// Returns the `Timestamp` as a Unix timestamp.
  pub fn to_unix(&self) -> i64 {
    self.0.unix_timestamp()
  }

  /// Creates a new `Timestamp` from the given Unix timestamp.
  ///
  /// The timestamp must be in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
  ///
  /// # Errors
  /// [`Error::InvalidTimestamp`] if `seconds` is outside of the interval [-62167219200,253402300799].
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

  /// Computes `self + duration`
  ///
  /// # Errors
  /// Errors if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
  pub fn try_add(self, duration: Duration) -> Result<Self> {
    self
      .0
      .checked_add(duration.0)
      .ok_or_else(|| time::error::Error::Format(time::error::Format::InvalidComponent("invalid year")).into())
      .and_then(|offset_date_time| Self::from_unix(offset_date_time.unix_timestamp()))
  }

  /// Computes `self - duration`
  ///
  /// # Errors
  /// Errors if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).
  pub fn try_sub(self, duration: Duration) -> Result<Self> {
    self
      .0
      .checked_sub(duration.0)
      .ok_or_else(|| time::error::Error::Format(time::error::Format::InvalidComponent("invalid year")).into())
      .and_then(|offset_date_time| Self::from_unix(offset_date_time.unix_timestamp()))
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
  offset_date_time - time::Duration::nanoseconds(offset_date_time.nanosecond() as i64)
}

impl Diff for Timestamp {
  type Type = DiffString;

  fn diff(&self, other: &Self) -> diff::Result<Self::Type> {
    self.to_string().diff(&other.to_string())
  }

  fn merge(&self, diff: Self::Type) -> diff::Result<Self> {
    self
      .to_string()
      .merge(diff)
      .and_then(|this| Self::parse(&this).map_err(diff::Error::merge))
  }

  fn from_diff(diff: Self::Type) -> diff::Result<Self> {
    String::from_diff(diff).and_then(|this| Self::parse(&this).map_err(diff::Error::convert))
  }

  fn into_diff(self) -> diff::Result<Self::Type> {
    self.to_string().into_diff()
  }
}

/// A span of time.
///
/// This type is typically used to increment or decrement a [`Timestamp`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
pub struct Duration(time::Duration);

// copy a small subset of the functionality offered by time::Duration. By not copying everything it will be easier
// to migrate to another implementation if that will be necessary in the future.
// Do not allow negative Durations hence all constructors take u32 instead of i64, this makes it possible for us to
// switch to relying on (for instance) std::time::Duration if we want to.
impl Duration {
  /// A second
  pub const SECOND: Self = Self(time::Duration::SECOND);
  /// A minute
  pub const MINUTE: Self = Self(time::Duration::MINUTE);
  /// An hour
  pub const HOUR: Self = Self(time::Duration::HOUR);
  /// A day
  pub const DAY: Self = Self(time::Duration::DAY);
  /// A week
  pub const WEEK: Self = Self(time::Duration::WEEK);

  /// Create a new [`Duration`] with the given amount of seconds.
  pub const fn seconds(seconds: u32) -> Self {
    Self(time::Duration::seconds(seconds as i64))
  }
  /// Create a new [`Duration`] with the given amount of minutes.
  pub const fn minutes(minutes: u32) -> Self {
    Self(time::Duration::minutes(minutes as i64))
  }

  /// Create a new [`Duration`] with the given amount of hours.
  pub const fn hours(hours: u32) -> Self {
    Self(time::Duration::hours(hours as i64))
  }

  /// Create a new [`Duration`] with the given amount of weeks.
  pub const fn weeks(weeks: u32) -> Self {
    Self(time::Duration::weeks(weeks as i64))
  }
}

#[cfg(test)]
mod tests {
  use identity_diff::Diff;
  use identity_diff::DiffString;
  use proptest::proptest;

  const LAST_VALID_UNIX_TIMESTAMP: i64 = 253402300799; // 9999-12-31T23:59:59Z
  const FIRST_VALID_UNIX_TIMESTAMP: i64 = -62167219200; // 0000-01-01T00:00:00Z
  use crate::common::Timestamp;
  use crate::convert::FromJson;
  use crate::convert::ToJson;

  use super::Duration;

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
  fn test_try_add() {
    let timestamp = Timestamp::parse("1980-01-01T12:34:56Z").unwrap();
    let second_later = timestamp.try_add(Duration::SECOND).unwrap();
    assert_eq!(second_later.to_rfc3339(), "1980-01-01T12:34:57Z");
    let minute_later = timestamp.try_add(Duration::MINUTE).unwrap();
    assert_eq!(minute_later.to_rfc3339(), "1980-01-01T12:35:56Z");
    let hour_later = timestamp.try_add(Duration::HOUR).unwrap();
    assert_eq!(hour_later.to_rfc3339(), "1980-01-01T13:34:56Z");
    let week_later = timestamp.try_add(Duration::WEEK).unwrap();
    assert_eq!(week_later.to_rfc3339(), "1980-01-08T12:34:56Z");

    // check overflow
    assert!(Timestamp::from_unix(LAST_VALID_UNIX_TIMESTAMP)
      .unwrap()
      .try_add(Duration::SECOND)
      .is_err());
  }

  #[test]
  fn test_try_sub() {
    let timestamp = Timestamp::parse("1980-01-01T12:34:56Z").unwrap();
    let second_earlier = timestamp.try_sub(Duration::SECOND).unwrap();
    assert_eq!(second_earlier.to_rfc3339(), "1980-01-01T12:34:55Z");
    let minute_earlier = timestamp.try_sub(Duration::MINUTE).unwrap();
    assert_eq!(minute_earlier.to_rfc3339(), "1980-01-01T12:33:56Z");
    let hour_earlier = timestamp.try_sub(Duration::HOUR).unwrap();
    assert_eq!(hour_earlier.to_rfc3339(), "1980-01-01T11:34:56Z");
    let week_earlier = timestamp.try_sub(Duration::WEEK).unwrap();
    assert_eq!(week_earlier.to_rfc3339(), "1979-12-25T12:34:56Z");

    // check underflow
    assert!(Timestamp::from_unix(FIRST_VALID_UNIX_TIMESTAMP)
      .unwrap()
      .try_sub(Duration::SECOND)
      .is_err());
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

  proptest! {
    #[test]
    fn test_from_unix_to_rfc3339_valid_no_panic(seconds in FIRST_VALID_UNIX_TIMESTAMP..=LAST_VALID_UNIX_TIMESTAMP) {
      let timestamp = Timestamp::from_unix(seconds).unwrap();
      let expected_length = "dddd-dd-ddTdd:dd:ddZ".len();
      assert_eq!(timestamp.to_rfc3339().len(), expected_length);
    }
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

  #[test]
  fn test_timestamp_diff() {
    let time1: Timestamp = Timestamp::parse("2021-01-01T12:00:01Z").unwrap();
    let time2: Timestamp = Timestamp::parse("2022-01-02T12:00:02Z").unwrap();

    // Diff
    let non_diff: DiffString = time1.diff(&time1).unwrap();
    assert!(non_diff.0.is_none());
    let diff12: DiffString = time1.diff(&time2).unwrap();
    assert_eq!(String::from_diff(diff12.clone()).unwrap(), time2.to_string());
    let diff21: DiffString = time2.diff(&time1).unwrap();
    assert_eq!(String::from_diff(diff21.clone()).unwrap(), time1.to_string());

    // Merge
    assert_eq!(time1.merge(non_diff.clone()).unwrap(), time1);
    assert_eq!(time2.merge(non_diff).unwrap(), time2);
    assert_eq!(time1.merge(diff12.clone()).unwrap(), time2);
    assert_eq!(time1.merge(diff21.clone()).unwrap(), time1);
    assert_eq!(time2.merge(diff12).unwrap(), time2);
    assert_eq!(time2.merge(diff21).unwrap(), time1);
  }
}
