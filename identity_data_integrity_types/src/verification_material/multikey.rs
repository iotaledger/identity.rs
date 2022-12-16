// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::utils::Base;

use crate::error::Error;
use crate::error::Result;

/// A multicodec code that can be encoded as an [unsigned varint](https://github.com/multiformats/unsigned-varint).
// The internal representation is a u64 because of: https://github.com/multiformats/unsigned-varint#practical-maximum-of-9-bytes-for-security.
#[derive(Debug, PartialEq)]
#[repr(transparent)]
pub struct Codec(u64);

impl Codec {
  /// The `ed25519-pub` multicodec.
  pub const ED25519_PUB: Self = Self::new(0xed);

  /// Creates a new multicodec from an arbitrary code.
  pub const fn new(code: u64) -> Self {
    Self(code)
  }

  /// Encodes the codec as an unsigned varint and returns the minimal buffer that represents that varint,
  /// i.e. there will be no trailing zeroes.
  pub fn encode(&self) -> Vec<u8> {
    // 1. Encode the code as a u64.
    let mut encoded: [u8; 10] = unsigned_varint::encode::u64_buffer();
    unsigned_varint::encode::u64(self.0, &mut encoded);

    // 2. Remove trailing zeros from the output to return the shortest possible buffer that represents the code.
    let required_len = 1
      + encoded
        .iter()
        .enumerate()
        .find(|(_, &byte)| unsigned_varint::decode::is_last(byte))
        .expect("we should have created a valid varint so the search for the last byte will succeed")
        .0;

    let mut encoded_vec: Vec<u8> = Vec::with_capacity(required_len);
    for byte in encoded {
      // Output must be non-empty.
      if !encoded_vec.is_empty() && byte == 0 {
        break;
      }

      encoded_vec.push(byte);
    }

    encoded_vec
  }

  /// Attempts to decode the given slice into a codec.
  /// The slice is expected to be encoded as an unsigned varint.
  /// Returns the remaining slice.
  ///
  /// # Errors
  ///
  /// Returns an error when the input is too long or too short.
  pub fn decode(varint: &[u8]) -> Result<(Self, &[u8])> {
    // Attempt to decode the vector into a `u64` which succeeds for all validly encoded varints.
    unsigned_varint::decode::u64(varint)
      .map_err(|err| Error::MultikeyDecode("invalid multicodec prefix", Some(Box::new(err))))
      .map(|(code, tail)| (Self(code), tail))
  }

  /// Returns true if the given multicodec code matches this codec's code.
  pub fn is_code(&self, expected: u64) -> bool {
    self.0 == expected
  }
}

/// A lazily-evaluated implementation of a Multikey.
///
/// A Multikey is multibase-encoded multicodec-prefixed key material.
pub struct Multikey(String);

impl Multikey {
  pub fn new(codec: Codec, key: &[u8]) -> Self {
    Multikey::new_with_base(Base::Base58Btc, codec, key)
  }

  pub fn new_with_base(base: Base, codec: Codec, key: &[u8]) -> Self {
    let mut input: Vec<u8> = codec.encode();
    input.extend_from_slice(key);
    let key: String = identity_core::utils::BaseEncoding::encode_multibase(&input, Some(base));
    Multikey(key)
  }

  /// Decodes the multikey into its codec and data. The data should be checked for validity.
  pub fn decode(&self) -> crate::error::Result<(Codec, Vec<u8>)> {
    let mut decoded: Vec<u8> = identity_core::utils::BaseEncoding::decode_multibase(&self.0)
      .map_err(|err| Error::MultikeyDecode("multibase decoding", Some(Box::new(err))))?;

    let (codec, tail) = Codec::decode(decoded.as_slice())?;

    // Remove the bytes representing the codec from the vector.
    // SAFETY: This is fine because tail.len() is always < decoded.len().
    let _ = decoded.drain(..(decoded.len() - tail.len()));

    Ok((codec, decoded))
  }
}

#[cfg(test)]
mod tests {
  use std::error::Error;

  use super::Codec;
  use super::Multikey;

  #[test]
  fn test_multicodec_encode_decode() {
    for number in [0, 1, 127, 256, 300, 1024, 10_000, 64_000, 66_000] {
      let encoded = Codec::new(number).encode();
      assert!(Codec::decode(encoded.as_slice()).is_ok());
    }
  }

  #[test]
  fn test_multicodec_decode_error_cases() {
    // Two bytes indicating continuation, i.e. input is too short.
    let invalid_encoding: [u8; 2] = [0b1000_0000; 2];
    let result = Codec::decode(&invalid_encoding);
    let error = result.unwrap_err();
    let source = error.source().unwrap();
    assert_eq!(
      source.downcast_ref::<unsigned_varint::decode::Error>().unwrap(),
      &unsigned_varint::decode::Error::Insufficient
    );

    // Input is too long and doesn't fit in u64.
    let too_long: [u8; 11] = [0xff; 11];
    let result = Codec::decode(&too_long);
    let error = result.unwrap_err();
    let source = error.source().unwrap();
    assert_eq!(
      source.downcast_ref::<unsigned_varint::decode::Error>().unwrap(),
      &unsigned_varint::decode::Error::Overflow
    );
  }

  #[test]
  fn test_multikey_encoded_decode() {
    let key: [u8; 32] = [0x25; 32];
    let codec: Codec = Codec::ED25519_PUB;
    let multikey = Multikey::new(codec, &key);

    let (codec, decoded_key) = multikey.decode().unwrap();

    assert_eq!(codec, Codec::ED25519_PUB);
    assert_eq!(key.to_vec(), decoded_key);
  }

  #[test]
  fn test_multikey_decode_len_0_key() {
    let key: Vec<u8> = vec![];
    let codec_code = 0xec;
    let multikey = Multikey::new(Codec::new(codec_code), &key);

    let (codec, decoded_key) = multikey.decode().unwrap();

    assert!(codec.is_code(codec_code));
    assert_eq!(key.to_vec(), decoded_key);
  }
}
