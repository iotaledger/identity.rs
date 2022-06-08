// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::io::Write;

use dataurl::DataUrl;
use flate2::write::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use identity_core::common::Url;
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

use crate::did::DID;
use crate::error::Error;
use crate::error::Result;
use crate::service::Service;
use crate::service::ServiceEndpoint;

/// A compressed bitmap for managing credential revocation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RevocationBitmap(RoaringBitmap);

impl RevocationBitmap {
  /// The name of the service type.
  pub const TYPE: &'static str = "RevocationBitmap2022";

  /// Constructs a new empty [`RevocationBitmap`].
  pub fn new() -> Self {
    Self(RoaringBitmap::new())
  }

  /// Returns `true` if the credential at the given `index` is revoked.
  pub fn is_revoked(&self, index: u32) -> bool {
    self.0.contains(index)
  }

  /// Mark the given `index` as revoked.
  ///
  /// Returns true if the `index` was absent from the set.
  pub fn revoke(&mut self, index: u32) -> bool {
    self.0.insert(index)
  }

  /// Mark the `index` as not revoked.
  ///
  /// Returns true if the `index` was present in the set.
  pub fn unrevoke(&mut self, index: u32) -> bool {
    self.0.remove(index)
  }

  pub fn to_endpoint(&self) -> Result<ServiceEndpoint> {
    let mut data_url: DataUrl = DataUrl::new();

    let endpoint_data: String = self.serialize_compressed_b64()?;

    // TODO: Set mime type, validate in other TryFrom impl.
    // data_url.set_media_type(new_media_type);

    data_url.set_data(endpoint_data.as_bytes());

    Ok(ServiceEndpoint::One(Url::parse(data_url.to_string())?))
  }

  /// Deserializes a compressed [`RevocationBitmap`] base64-encoded `data`.
  pub(crate) fn deserialize_compressed_b64<T>(data: &T) -> Result<Self>
  where
    T: AsRef<str> + ?Sized,
  {
    let decoded_data: Vec<u8> =
      decode_b64(data).map_err(|e| Error::Base64DecodingError(data.as_ref().to_owned(), e))?;
    let decompressed_data: Vec<u8> = Self::decompress_zlib(decoded_data)?;
    Self::deserialize_slice(&decompressed_data)
  }

  /// Serializes and compressess [`RevocationBitmap`] as a base64-encoded `String`.
  pub(crate) fn serialize_compressed_b64(&self) -> Result<String> {
    let serialized_data: Vec<u8> = self.serialize_vec()?;
    Self::compress_zlib(&serialized_data).map(|data| encode_b64(&data))
  }

  /// Deserializes [`RevocationBitmap`] from a slice of bytes.
  fn deserialize_slice(data: &[u8]) -> Result<Self> {
    RoaringBitmap::deserialize_from(data)
      .map_err(Error::BitmapDecodingError)
      .map(Self)
  }

  /// Serializes a [`RevocationBitmap`] as a vector of bytes.
  fn serialize_vec(&self) -> Result<Vec<u8>> {
    let mut output: Vec<u8> = Vec::with_capacity(self.0.serialized_size());
    self.0.serialize_into(&mut output).map_err(Error::BitmapEncodingError)?;
    Ok(output)
  }

  fn compress_zlib<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_ref()).map_err(Error::BitmapEncodingError)?;
    encoder.finish().map_err(Error::BitmapEncodingError)
  }

  fn decompress_zlib<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    let mut writer = Vec::new();
    let mut decoder = ZlibDecoder::new(writer);
    decoder.write_all(input.as_ref()).map_err(Error::BitmapDecodingError)?;
    writer = decoder.finish().map_err(Error::BitmapDecodingError)?;
    Ok(writer)
  }
}

impl Serialize for RevocationBitmap {
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

impl<'de> Deserialize<'de> for RevocationBitmap {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct __Visitor;

    impl<'de> Visitor<'de> for __Visitor {
      type Value = RevocationBitmap;

      fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a base64-encoded string")
      }

      fn visit_str<E>(self, string: &str) -> Result<Self::Value, E>
      where
        E: de::Error,
      {
        RevocationBitmap::deserialize_compressed_b64(string).map_err(E::custom)
      }
    }

    deserializer.deserialize_str(__Visitor)
  }
}

impl<D: DID + Sized> TryFrom<&Service<D>> for RevocationBitmap {
  type Error = Error;

  fn try_from(service: &Service<D>) -> Result<Self> {
    if service.type_() != Self::TYPE {
      return Err(Error::InvalidService("invalid type - unexpected revocation method"));
    }
    match service.service_endpoint() {
      ServiceEndpoint::One(url) => {
        if service.type_() != Self::TYPE {
          Err(Error::InvalidService(
            "invalid service - expected a `RevocationBitmap2022`",
          ))
        } else {
          let data_url: DataUrl =
            DataUrl::parse(url.as_str()).map_err(|_| Error::InvalidService("invalid url - expected a data url"))?;

          RevocationBitmap::deserialize_compressed_b64(
            std::str::from_utf8(data_url.get_data())
              .map_err(|_| Error::InvalidService("invalid data url - expected valid utf-8"))?,
          )
        }
      }
      ServiceEndpoint::Map(_) | ServiceEndpoint::Set(_) => {
        Err(Error::InvalidService("invalid endpoint - expected a single data url"))
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::RevocationBitmap;

  #[test]
  fn test_serialize_b64_round_trip() {
    let mut embedded_revocation_list = RevocationBitmap::new();
    let b64_compressed_revocation_list: String = embedded_revocation_list.serialize_compressed_b64().unwrap();
    assert_eq!(&b64_compressed_revocation_list, "eJyzMmAAAwADKABr");
    assert_eq!(
      RevocationBitmap::deserialize_compressed_b64(&b64_compressed_revocation_list).unwrap(),
      embedded_revocation_list
    );

    for credential in [0, 5, 6, 8] {
      embedded_revocation_list.revoke(credential);
    }
    let b64_compressed_revocation_list: String = embedded_revocation_list.serialize_compressed_b64().unwrap();
    assert_eq!(&b64_compressed_revocation_list, "eJyzMmBgYGQAAWYGATDNysDGwMEAAAscAJI=");
    assert_eq!(
      RevocationBitmap::deserialize_compressed_b64(&b64_compressed_revocation_list).unwrap(),
      embedded_revocation_list
    );
  }
}
