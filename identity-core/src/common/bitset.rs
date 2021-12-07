// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::utils;
use core::fmt::Formatter;
pub(crate) use errors::BitSetDecodingError;
pub(crate) use errors::BitSetEncodingError;
use roaring::RoaringBitmap;
use serde::de;
use serde::de::Deserializer;
use serde::de::Visitor;
use serde::ser::Error as _;
use serde::ser::Serializer;
use serde::Deserialize;
use serde::Serialize;

/// A general-purpose compressed bitset.
#[derive(Clone, Debug, PartialEq)]
pub struct BitSet(RoaringBitmap);

impl BitSet {
  /// Creates a new [`BitSet`].
  pub fn new() -> Self {
    Self(RoaringBitmap::new())
  }

  /// Returns the total number of values in the set.
  pub fn len(&self) -> u64 {
    self.0.len()
  }

  /// Returns true if the set is empty.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  /// Clears all values in the set.
  pub fn clear(&mut self) {
    self.0.clear();
  }

  /// Returns `true` if the set contains the specified `index`.
  pub fn contains(&self, index: u32) -> bool {
    self.0.contains(index)
  }

  /// Adds a new `index` to the set.
  pub fn insert(&mut self, index: u32) -> bool {
    self.0.insert(index)
  }

  /// Extends the set with the indices from `iter`.
  pub fn insert_all<I>(&mut self, iter: I)
  where
    I: IntoIterator<Item = u32>,
  {
    for index in iter.into_iter() {
      self.0.insert(index);
    }
  }

  /// Removes the specified `index` from the set.
  pub fn remove(&mut self, index: u32) -> bool {
    self.0.remove(index)
  }

  /// Serializes the [`BitSet`] as a base64-encoded `String`.
  fn serialize_b64(&self) -> Result<String, BitSetEncodingError> {
    self.serialize_vec().map(|data| utils::encode_b64(&data))
  }

  /// Serializes the [`BitSet`] as a vector of bytes.
  fn serialize_vec(&self) -> Result<Vec<u8>, BitSetEncodingError> {
    let mut output: Vec<u8> = Vec::with_capacity(self.0.serialized_size());

    self.0.serialize_into(&mut output)?;

    Ok(output)
  }

  /// Deserializes a [`BitSet`] from base64-encoded `data`.
  fn deserialize_b64(data: &str) -> Result<Self, BitSetDecodingError> {
    Self::deserialize_slice(
      &utils::decode_b64(data)
        .map_err(|decode_error| std::io::Error::new(std::io::ErrorKind::InvalidData, decode_error))?,
    )
  }

  /// Deserializes a [`BitSet`] from a slice of bytes.
  fn deserialize_slice(data: &[u8]) -> Result<Self, BitSetDecodingError> {
    Ok(Self(RoaringBitmap::deserialize_from(data)?))
  }
}

impl Default for BitSet {
  fn default() -> Self {
    Self::new()
  }
}

impl Serialize for BitSet {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self
      .serialize_b64()
      .map_err(S::Error::custom)
      .and_then(|data| serializer.serialize_str(&data))
  }
}

impl<'de> Deserialize<'de> for BitSet {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor;

    impl<'de> Visitor<'de> for __Visitor {
      type Value = BitSet;

      fn expecting(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("a base64-encoded string")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        BitSet::deserialize_b64(value).map_err(E::custom)
      }
    }

    deserializer.deserialize_str(__Visitor)
  }
}
mod errors {

  use thiserror::Error as DeriveError;
  /// Caused by a failure to encode a Roaring Bitmap.
  #[derive(Debug, DeriveError)]
  #[error("failed to encode roaring bitmap {inner}")]
  pub(crate) struct BitSetEncodingError {
    #[from]
    inner: std::io::Error,
  }

  /// Caused by a failure to decode a Roaring Bitmap.
  #[derive(Debug, DeriveError)]
  #[error("failed to decode roaring bitmap {inner}")]
  pub(crate) struct BitSetDecodingError {
    #[from]
    inner: std::io::Error,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basics() {
    let mut set = BitSet::new();

    assert_eq!(set.len(), 0);

    for index in 0..10 {
      assert!(set.insert(index));
      assert!(!set.insert(index));
    }

    assert_eq!(set.len(), 10);

    for index in 0..10 {
      assert!(set.contains(index));
    }

    for index in 0..10 {
      assert!(set.remove(index));
      assert!(!set.remove(index));
    }

    assert_eq!(set.len(), 0);

    for index in 0..10 {
      assert!(!set.contains(index));
    }

    set.insert_all(0..1024);
    set.insert_all(0..1024);

    assert_eq!(set.len(), 1024);

    set.clear();

    assert_eq!(set.len(), 0);
  }

  // Validate that a `deserialize_b64` ∘ `serialize_b64` round-trip results in the original bitset.
  #[test]
  fn test_serialize_b64_round_trip() {
    let mut set = BitSet::new();
    for index in 0..10 {
      assert!(set.insert(index));
    }

    assert_eq!(
      BitSet::deserialize_b64(set.serialize_b64().unwrap().as_str()).unwrap(),
      set
    );
  }

  // Validate that a `deserialize_slice` ∘ `serialize_vec` round-trip results in the original bitset.
  #[test]
  fn test_serialize_slice_round_trip() {
    let mut set = BitSet::new();
    for index in 0..10 {
      assert!(set.insert(index));
    }

    assert_eq!(
      BitSet::deserialize_slice(set.serialize_vec().unwrap().as_slice()).unwrap(),
      set
    );
  }
}
