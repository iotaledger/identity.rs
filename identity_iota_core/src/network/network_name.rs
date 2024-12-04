// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::TryFrom;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Deref;
use std::fmt::Debug;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;

/// Network name compliant with the [`crate::IotaDID`] method specification.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
pub struct NetworkName(String);

impl NetworkName {
  /// The maximum length of a network name.
  pub const MAX_LENGTH: usize = 8;

  /// Validates whether a string is a spec-compliant IOTA DID [`NetworkName`].
  pub fn validate_network_name(name: &str) -> Result<()> {
    Some(())
      .filter(|_| {
        !name.is_empty()
          && (name.len() <= Self::MAX_LENGTH)
          && name.chars().all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit())
      })
      .ok_or_else(|| Error::InvalidNetworkName(name.to_owned()))
  }
}

impl AsRef<str> for NetworkName {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl Deref for NetworkName {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<String> for NetworkName {
  type Error = Error;
  fn try_from(value: String) -> Result<Self> {
    Self::validate_network_name(&value)?;
    Ok(Self(value))
  }
}

impl<'a> TryFrom<&'a str> for NetworkName {
  type Error = Error;
  fn try_from(value: &'a str) -> Result<Self> {
    value.to_string().try_into()
  }
}

impl FromStr for NetworkName {
  type Err = Error;
  fn from_str(name: &str) -> Result<Self> {
    Self::validate_network_name(name)?;
    Ok(Self(name.to_string()))
  }
}

impl Debug for NetworkName {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_ref())
  }
}

impl Display for NetworkName {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_ref())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Rules are: at least one character, at most eight characters and may only contain digits and/or lowercase ascii
  // characters.
  const VALID_NETWORK_NAMES: &[&str] = &[
    "main", "dev", "smr", "rms", "test", "foo", "foobar", "123456", "0", "foo42", "bar123", "42foo", "1234567",
    "foobar0",
  ];

  const INVALID_NETWORK_NAMES: &[&str] = &["Main", "fOo", "deV", "féta", "", "  ", "foo ", " foo"];

  #[test]
  fn valid_validate_network_name() {
    for name in VALID_NETWORK_NAMES {
      assert!(NetworkName::validate_network_name(name).is_ok());
    }
  }

  #[test]
  fn invalid_validate_network_name() {
    for name in INVALID_NETWORK_NAMES {
      assert!(NetworkName::validate_network_name(name).is_err());
    }
  }
}
