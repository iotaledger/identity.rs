// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::io::Write;

use flate2::write::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use identity_core::utils::decode_b64;
use identity_core::utils::encode_b64;
use roaring::RoaringBitmap;
use serde::de;
use serde::de::Deserializer;
use serde::de::Visitor;
use serde::ser::Error as _;
use serde::ser::Serializer;
use serde::Deserialize;
use serde::Serialize;

use super::error::Result;
use super::error::RevocationMethodError;

pub const EMBEDDED_REVOCATION_METHOD_NAME: &str = "EmbeddedRevocationList";
const REVOCATION_LIST_INDEX: &str = "revocationListIndex";

pub struct EmbeddedRevocationList(RoaringBitmap);

impl EmbeddedRevocationList {
  /// Returns the name of the revocation method.
  pub fn name() -> &'static str {
    EMBEDDED_REVOCATION_METHOD_NAME
  }

  // Returns the name of the property that contains the index of the credential to be checked.
  pub fn credential_list_index_property() -> &'static str {
    REVOCATION_LIST_INDEX
  }

  /// Creates a new `EmbeddedRevocationList` revocation method.
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

  /// Deserializes a compressed [`EmbeddedRevocationList`] base64-encoded `data`.
  pub fn deserialize_compressed_b64(data: &str) -> Result<Self> {
    let decoded_data: Vec<u8> =
      decode_b64(data).map_err(|e| RevocationMethodError::Base64DecodingError(data.to_owned(), e))?;
    let decompressed_data: Vec<u8> = Self::decompress_zlib(decoded_data)?;
    Self::deserialize_slice(&decompressed_data)
  }

  /// Serializes and compressess [`EmbeddedRevocationList`] as a base64-encoded `String`.
  pub fn serialize_compressed_b64(&self) -> Result<String> {
    let serialized_data: Vec<u8> = self.serialize_vec()?;
    Self::compress_zlib(&serialized_data).map(|data| encode_b64(&data))
  }

  /// Deserializes [`EmbeddedRevocationList`] from a slice of bytes.
  pub fn deserialize_slice(data: &[u8]) -> Result<Self> {
    RoaringBitmap::deserialize_from(data)
      .map_err(RevocationMethodError::DeserializationError)
      .map(Self)
  }

  /// Serializes a [`EmbeddedRevocationList`] as a vector of bytes.
  pub fn serialize_vec(&self) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = Vec::with_capacity(self.0.serialized_size());
    self
      .0
      .serialize_into(&mut output)
      .map_err(RevocationMethodError::SerializationError)?;
    Ok(output)
  }

  fn compress_zlib<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
      .write_all(input.as_ref())
      .map_err(RevocationMethodError::CompressionError)?;
    encoder.finish().map_err(RevocationMethodError::CompressionError)
  }

  fn decompress_zlib<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    let mut writer = Vec::new();
    let mut decoder = ZlibDecoder::new(writer);
    decoder
      .write_all(input.as_ref())
      .map_err(RevocationMethodError::DecompressionError)?;
    writer = decoder.finish().map_err(RevocationMethodError::DecompressionError)?;
    Ok(writer)
  }
}

impl Serialize for EmbeddedRevocationList {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    self
      .serialize_compressed_b64()
      .map_err(S::Error::custom)
      .and_then(|data| serializer.serialize_str(&data))
  }
}

impl<'de> Deserialize<'de> for EmbeddedRevocationList {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor;

    impl<'de> Visitor<'de> for __Visitor {
      type Value = EmbeddedRevocationList;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a base64-encoded string")
      }

      fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        EmbeddedRevocationList::deserialize_compressed_b64(value).map_err(E::custom)
      }
    }

    deserializer.deserialize_str(__Visitor)
  }
}
