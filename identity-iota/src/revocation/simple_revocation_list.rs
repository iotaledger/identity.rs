// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::utils::decode_b64;
use identity_core::utils::encode_b64;
use roaring::RoaringBitmap;

use super::error::Error;
use super::error::Result;

const REVOCATION_LIST_INDEX: &str = "revocationListIndex";

pub struct SimpleRevocationList2022(RoaringBitmap);

impl SimpleRevocationList2022 {
  /// Creates a new [`SimpleRevocationList2022`]
  pub fn new() -> Self {
    Self(RoaringBitmap::new())
  }

  /// Returns `true` if the credential at the given `index` is revoked.
  pub fn is_revoked(&self, index: u32) -> bool {
    self.0.contains(index)
  }

  /// Revokes the credential at the given `index`.
  pub fn revoke(&mut self, index: u32) -> bool {
    self.0.insert(index)
  }

  /// The credential at the given `index` will be set to valid.
  pub fn undo_revocation(&mut self, index: u32) -> bool {
    self.0.remove(index)
  }

  /// Deserializes a [`SimpleRevocationList2022`] from base64-encoded `data`.
  pub fn deserialize_b64(data: &str) -> Result<Self> {
    Self::deserialize_slice(&decode_b64(data).map_err(|e| Error::Base64DecodingError(data.to_owned(), e))?)
  }

  /// Serializes [`SimpleRevocationList2022`] as a base64-encoded `String`.
  pub fn serialize_b64(&self) -> Result<String> {
    self.serialize_vec().map(|data| encode_b64(&data))
  }

  /// Deserializes [`SimpleRevocationList2022`] from a slice of bytes.
  pub fn deserialize_slice(data: &[u8]) -> Result<Self> {
    RoaringBitmap::deserialize_from(data)
      .map_err(Error::DeserializationError)
      .map(Self)
  }

  /// Serializes [`SimpleRevocationList2022`] as a vector of bytes.
  pub fn serialize_vec(&self) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = Vec::with_capacity(self.0.serialized_size());
    self.0.serialize_into(&mut output).map_err(Error::SerializationError)?;
    Ok(output)
  }

  pub fn index_property() -> &'static str {
    REVOCATION_LIST_INDEX
  }
}

impl Default for SimpleRevocationList2022 {
  fn default() -> Self {
    Self::new()
  }
}
