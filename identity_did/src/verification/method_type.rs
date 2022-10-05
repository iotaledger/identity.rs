// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;

use crate::error::Error;
use crate::error::Result;

/// Supported verification method types.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum MethodType {
  Ed25519VerificationKey2018,
  X25519KeyAgreementKey2019,
  // Other(String),
}

impl MethodType {
  pub fn as_str(&self) -> &str {
    match self {
      Self::Ed25519VerificationKey2018 => "Ed25519VerificationKey2018",
      Self::X25519KeyAgreementKey2019 => "X25519KeyAgreementKey2019",
      // Self::Other(other) => other.as_str()
    }
  }
}

impl Display for MethodType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl FromStr for MethodType {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    match string {
      "Ed25519VerificationKey2018" => Ok(Self::Ed25519VerificationKey2018),
      "X25519KeyAgreementKey2019" => Ok(Self::X25519KeyAgreementKey2019),
      // other => Ok(Self::Other(other.to_owned())),
      _ => Err(Error::UnknownMethodType),
    }
  }
}

#[cfg(test)]
mod tests {
  use serde_json::Value;

  use super::*;

  #[test]
  fn test_method_type_serde() {
    for method_type in [
      MethodType::Ed25519VerificationKey2018,
      MethodType::X25519KeyAgreementKey2019,
    ] {
      let ser: Value = serde_json::to_value(&method_type).unwrap();
      assert_eq!(ser.as_str().unwrap(), method_type.as_str());
      let de: MethodType = serde_json::from_value(ser.clone()).unwrap();
      assert_eq!(de, method_type);

      assert_eq!(MethodType::from_str(method_type.as_str()).unwrap(), method_type);
      assert_eq!(MethodType::from_str(ser.as_str().unwrap()).unwrap(), method_type);
    }
  }
}
