// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::hash::Hash;
use core::hash::Hasher;
use identity_core::convert::FromJson;
use identity_core::convert::ToJson;
use identity_core::utils::decode_b64;
use identity_core::utils::encode_b64;
use serde::de;
use serde::ser;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

use crate::error::Result;
use crate::identity::IdentityId;
use crate::identity::IdentityName;

/// Information used to identify an identity.
#[derive(Clone)]
pub struct IdentityTag {
  name: IdentityName,
  method_id: String,
}

impl IdentityTag {
  /// Creates a new IdentityTag with a default name.
  pub fn new(method_id: String) -> Self {
    Self {
      name: IdentityName::Default,
      method_id,
    }
  }

  /// Creates a new IdentityTag with an explicit name.
  pub fn named(method_id: String, name: String) -> Self {
    Self {
      name: IdentityName::Literal(name),
      method_id,
    }
  }

  /// Returns the user-assigned name of the identity.
  pub fn name(&self) -> Option<&str> {
    self.name.as_opt()
  }

  /// Returns the name of the identity, whether user-assigned or default.
  pub fn fullname(&self, id: IdentityId) -> Cow<'_, str> {
    self.name.as_str(id)
  }

  /// Returns the method id of the Identity DID Document.
  pub fn method_id(&self) -> &str {
    &self.method_id
  }

  // Returns the identity tag as a base64-encoded JSON string.
  fn encode(&self) -> Result<String> {
    let data: ProxySerialize<'_> = ProxySerialize {
      method_id: &self.method_id,
      name: &self.name,
    };

    let json: Vec<u8> = data.to_json_vec()?;
    let base: String = encode_b64(&json);

    Ok(base)
  }

  // Decodes an identity tag from a base64-encoded JSON string.
  fn decode(string: &str) -> Result<Self> {
    let json: Vec<u8> = decode_b64(string)?;
    let data: ProxyDeserialize = ProxyDeserialize::from_json_slice(&json)?;

    Ok(Self {
      method_id: data.method_id,
      name: data.name,
    })
  }
}

impl Debug for IdentityTag {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.write_fmt(format_args!("IdentityTag({}, {:?})", self.method_id, self.name))
  }
}

impl PartialEq for IdentityTag {
  fn eq(&self, other: &Self) -> bool {
    self.method_id.eq(&other.method_id)
  }
}

impl Eq for IdentityTag {}

impl Hash for IdentityTag {
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    self.method_id.hash(hasher);
  }
}

// =============================================================================
// Serde
// =============================================================================

#[derive(Serialize)]
struct ProxySerialize<'a> {
  #[serde(rename = "1")]
  method_id: &'a str,
  #[serde(rename = "2")]
  name: &'a IdentityName,
}

#[derive(Deserialize)]
struct ProxyDeserialize {
  #[serde(rename = "1")]
  method_id: String,
  #[serde(rename = "2")]
  name: IdentityName,
}

impl Serialize for IdentityTag {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    match self.encode() {
      Ok(data) => serializer.serialize_str(&data),
      Err(error) => Err(ser::Error::custom(error)),
    }
  }
}

impl<'de> Deserialize<'de> for IdentityTag {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    struct Visitor;

    impl<'de> de::Visitor<'de> for Visitor {
      type Value = IdentityTag;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a base64-encoded string")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        IdentityTag::decode(value).map_err(E::custom)
      }
    }

    deserializer.deserialize_str(Visitor)
  }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compare() {
    let a: IdentityTag = IdentityTag::new("abcde".into());
    let b: IdentityTag = IdentityTag::named("abcde".into(), "Foo".into());

    assert_eq!(a, b);
    assert_eq!(a.method_id(), b.method_id());
    assert_ne!(a.name(), b.name());
  }
}
