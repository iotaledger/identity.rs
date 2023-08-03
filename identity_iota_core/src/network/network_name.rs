// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use core::convert::TryFrom;
use core::fmt::Display;
use core::fmt::Formatter;
use core::ops::Deref;
use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;

use crate::error::Error;
use crate::error::Result;

/// Network name compliant with the [`crate::IotaDID`] method specification.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[repr(transparent)]
pub struct NetworkName(Cow<'static, str>);

impl NetworkName {
  /// The maximum length of a network name.
  pub const MAX_LENGTH: usize = 6;

  /// Creates a new [`NetworkName`] if the name passes validation.
  pub fn try_from<T>(name: T) -> Result<Self>
  where
    T: Into<Cow<'static, str>>,
  {
    let name_cow: Cow<'static, str> = name.into();
    Self::validate_network_name(&name_cow)?;
    Ok(Self(name_cow))
  }

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

impl From<NetworkName> for Cow<'static, str> {
  fn from(network_name: NetworkName) -> Self {
    network_name.0
  }
}

impl Deref for NetworkName {
  type Target = Cow<'static, str>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<&'static str> for NetworkName {
  type Error = Error;

  fn try_from(name: &'static str) -> Result<Self, Self::Error> {
    Self::try_from(Cow::Borrowed(name))
  }
}

impl TryFrom<String> for NetworkName {
  type Error = Error;

  fn try_from(name: String) -> Result<Self, Self::Error> {
    Self::try_from(Cow::Owned(name))
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

#[cfg(feature = "client")]
mod try_from_network_name {
  use iota_sdk::types::block::address::Hrp;

  use crate::Error;
  use crate::NetworkName;
  use std::str::FromStr;

  impl TryFrom<&NetworkName> for Hrp {
    type Error = Error;

    fn try_from(network_name: &NetworkName) -> std::result::Result<Self, Self::Error> {
      Hrp::from_str(network_name.as_ref())
        .map_err(|err| Error::InvalidNetworkName(format!("could not convert network name to HRP: {err}")))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Rules are: at least one character, at most six characters and may only contain digits and/or lowercase ascii
  // characters.
  const VALID_NETWORK_NAMES: [&str; 12] = [
    "main", "dev", "smr", "rms", "test", "foo", "foobar", "123456", "0", "foo42", "bar123", "42foo",
  ];

  const INVALID_NETWORK_NAMES: [&str; 10] = [
    "Main", "fOo", "deV", "féta", "", "  ", "foo ", " foo", "1234567", "foobar0",
  ];

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
