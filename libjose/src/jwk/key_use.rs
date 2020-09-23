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

/// Supported algorithms for the JSON Web Key `use` property.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-key-use)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JwkUse {
  /// Digital Signature or MAC.
  Signature,
  /// Encryption.
  Encryption,
  /// Custom key use.
  Custom(String),
}

impl JwkUse {
  pub fn name(&self) -> &str {
    match self {
      Self::Signature => "sig",
      Self::Encryption => "enc",
      Self::Custom(inner) => inner.as_str(),
    }
  }
}

impl Default for JwkUse {
  fn default() -> Self {
    Self::Custom("unknown".into())
  }
}

impl Display for JwkUse {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl<'a> From<&'a str> for JwkUse {
  fn from(other: &'a str) -> Self {
    other.to_string().into()
  }
}

impl From<String> for JwkUse {
  fn from(other: String) -> Self {
    match other.as_str() {
      "sig" => Self::Signature,
      "enc" => Self::Encryption,
      _ => Self::Custom(other),
    }
  }
}

impl<'de> Deserialize<'de> for JwkUse {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = JwkUse;

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

impl Serialize for JwkUse {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.name())
  }
}
