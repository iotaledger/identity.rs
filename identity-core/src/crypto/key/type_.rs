// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use crate::error::Error;
use crate::error::Result;

/// Supported cryptographic key types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum KeyType {
  /// Identifies an `Ed25519` public/secret key.
  #[serde(rename = "ed25519")]
  Ed25519,
}

impl KeyType {
  /// Returns the [`KeyType`] name as a static `str`.
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::Ed25519 => "ed25519",
    }
  }
}

impl FromStr for KeyType {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    if string.eq_ignore_ascii_case("ed25519") {
      Ok(Self::Ed25519)
    } else {
      Err(Error::InvalidKeyFormat)
    }
  }
}
