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

/// Supported algorithms for the JSON Web Encryption `zip` claim.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-encryption-compression-algorithms)
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JweCompression {
  /// Compression with the DEFLATE [RFC1951](https://tools.ietf.org/html/rfc1951) algorithm.
  Deflate,
  /// Custom compression algorithm.
  Custom(String),
}

impl JweCompression {
  pub fn name(&self) -> &str {
    match self {
      Self::Deflate => "DEF",
      Self::Custom(inner) => inner.as_str(),
    }
  }
}

impl Default for JweCompression {
  fn default() -> Self {
    Self::Deflate
  }
}

impl Display for JweCompression {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    f.write_fmt(format_args!("{}", self.name()))
  }
}

impl<'a> From<&'a str> for JweCompression {
  fn from(other: &'a str) -> Self {
    other.to_string().into()
  }
}

impl From<String> for JweCompression {
  fn from(other: String) -> Self {
    if other == "DEF" {
      Self::Deflate
    } else {
      Self::Custom(other)
    }
  }
}

impl<'de> Deserialize<'de> for JweCompression {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = JweCompression;

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

impl Serialize for JweCompression {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.name())
  }
}
