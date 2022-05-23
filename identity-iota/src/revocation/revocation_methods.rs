// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::error::Error;
use super::error::Result;

/// All supported methods for revoking a credential
pub enum RevocationMethods {
  SimpleRevocationList2022,
}

impl RevocationMethods {
  pub fn name(&self) -> &str {
    match self {
      Self::SimpleRevocationList2022 => "SimpleRevocationList2022",
    }
  }
}

impl TryFrom<&str> for RevocationMethods {
  type Error = Error;

  fn try_from(value: &str) -> Result<Self> {
    match value {
      "SimpleRevocationList2022" => Ok(Self::SimpleRevocationList2022),
      _ => Err(Error::UnsupportedRevocationMethod(value.to_owned())),
    }
  }
}
