// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::str::FromStr;
use std::borrow::Cow;

use identity_core::crypto::KeyType;

use crate::error::Error;
use crate::error::Result;

/// Supported verification method types.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct MethodType(Cow<'static, str>);

impl MethodType {
  //TODO: Document these public constants.
  pub const MULTIKEY: Self = Self(Cow::Borrowed("Multikey"));
  pub const ED25519_VERIFICATION_KEY_2018: Self = Self(Cow::Borrowed("Ed25519VerificationKey2018"));
  pub const X25519_KEY_AGREEMENT_KEY_2019: Self = Self(Cow::Borrowed("X25519KeyAgreementKey2019"));
}

impl MethodType {
  pub fn as_str(&self) -> &str {
    self.0.as_ref()
  }
}

impl AsRef<str> for MethodType {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
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
      "Ed25519VerificationKey2018" => Ok(Self::ED25519_VERIFICATION_KEY_2018),
      "X25519KeyAgreementKey2019" => Ok(Self::X25519_KEY_AGREEMENT_KEY_2019),
      _ => Ok(Self(Cow::Owned(string.to_owned()))),
    }
  }
}

// TODO: Remove FromStr impl as it cannot fail?

impl From<String> for MethodType {
  fn from(string: String) -> Self {
    match string.as_str() {
      "Ed25519VerificationKey2018" => Self::ED25519_VERIFICATION_KEY_2018,
      "X25519KeyAgreementKey2019" => Self::X25519_KEY_AGREEMENT_KEY_2019,
      _ => Self(Cow::Owned(string.to_owned())),
    }
  }
}

// TODO: Is this the right place for this? Is this even needed?
impl TryFrom<MethodType> for KeyType {
  // TODO: Find a better error type.
  type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

  fn try_from(method_type: MethodType) -> Result<Self, Self::Error> {
    match method_type {
      ty if ty == MethodType::ED25519_VERIFICATION_KEY_2018 => Ok(KeyType::Ed25519),
      ty if ty == MethodType::X25519_KEY_AGREEMENT_KEY_2019 => Ok(KeyType::X25519),
      type_ if type_.as_str().starts_with("Ed25519") => Ok(KeyType::Ed25519),
      other => Err(
        format!(
          "method type {} could not be converted to a KeyType recognized by the IOTA Identity framework",
          other
        )
        .into(),
      ),
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
      MethodType::ED25519_VERIFICATION_KEY_2018,
      MethodType::X25519_KEY_AGREEMENT_KEY_2019,
    ] {
      let ser: Value = serde_json::to_value(method_type).unwrap();
      assert_eq!(ser.as_str().unwrap(), method_type.as_str());
      let de: MethodType = serde_json::from_value(ser.clone()).unwrap();
      assert_eq!(de, method_type);

      assert_eq!(MethodType::from_str(method_type.as_str()).unwrap(), method_type);
      assert_eq!(MethodType::from_str(ser.as_str().unwrap()).unwrap(), method_type);
    }
  }
}
