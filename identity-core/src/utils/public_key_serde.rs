//! Provides (de)serialization functions for [`PublicKey`]

use serde::{
  de::{self, Visitor},
  Deserializer, Serializer,
};

use crate::{crypto::PublicKey, utils::decode_b58};

use super::encode_b58;

/// Serialize the given `pubkey` as a base58-encoded string
pub fn serialize<S>(pubkey: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  let b58 = encode_b58(pubkey);
  serializer.serialize_str(&b58)
}

/// Deserialize a base58-encoded string to a [`PublicKey`]
pub fn deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
where
  D: Deserializer<'de>,
{
  struct PublicKeyVisitor;

  impl<'de> Visitor<'de> for PublicKeyVisitor {
    type Value = PublicKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      formatter.write_str("a base58-encoded string representing a public key")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
      E: de::Error,
    {
      let bytes = decode_b58(value).map_err(E::custom)?;
      Ok(PublicKey::from(bytes))
    }
  }

  deserializer.deserialize_str(PublicKeyVisitor)
}
