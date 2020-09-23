use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use serde::de;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

use crate::alloc::String;
use crate::alloc::ToString;

/// Supported algorithms for the JSON Web Key `key_ops` property.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-operations)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JwkOperation {
  /// Compute digital signature or MAC.
  Sign,
  /// Verify digital signature or MAC.
  Verify,
  /// Encrypt content.
  Encrypt,
  /// Decrypt content and validate decryption, if applicable.
  Decrypt,
  /// Encrypt key.
  WrapKey,
  /// Decrypt key and validate decryption, if applicable.
  UnwrapKey,
  /// Derive key.
  DeriveKey,
  /// Derive bits not to be used as a key.
  DeriveBits,
  /// Custom key operation.
  Custom(String),
}

impl JwkOperation {
  pub fn name(&self) -> &str {
    match self {
      Self::Sign => "sign",
      Self::Verify => "verify",
      Self::Encrypt => "encrypt",
      Self::Decrypt => "decrypt",
      Self::WrapKey => "wrapKey",
      Self::UnwrapKey => "unwrapKey",
      Self::DeriveKey => "deriveKey",
      Self::DeriveBits => "deriveBits",
      Self::Custom(inner) => inner.as_str(),
    }
  }
}

impl Default for JwkOperation {
  fn default() -> Self {
    Self::Custom("unknown".into())
  }
}

impl Display for JwkOperation {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl<'a> From<&'a str> for JwkOperation {
  fn from(other: &'a str) -> Self {
    other.to_string().into()
  }
}

impl From<String> for JwkOperation {
  fn from(other: String) -> Self {
    match other.as_str() {
      "sign" => Self::Sign,
      "verify" => Self::Verify,
      "encrypt" => Self::Encrypt,
      "decrypt" => Self::Decrypt,
      "wrapKey" => Self::WrapKey,
      "unwrapKey" => Self::UnwrapKey,
      "deriveKey" => Self::DeriveKey,
      "deriveBits" => Self::DeriveBits,
      _ => Self::Custom(other),
    }
  }
}

impl<'de> Deserialize<'de> for JwkOperation {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = JwkOperation;

      fn expecting(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("a UTF-8 string")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(value.into())
      }

      fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        Ok(value.into())
      }
    }

    deserializer.deserialize_any(Visitor)
  }
}

impl Serialize for JwkOperation {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.name())
  }
}
