// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::error::Error;
use crate::error::Result;

/// A [Multibase]-supported base. See [multibase::Base] for more information.
///
/// Excludes the identity (0x00) base as arbitrary bytes cannot be encoded to a valid UTF-8 string
/// in general.
///
/// [Multibase]: https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum Base {
  /// 8-bit binary (encoder and decoder keeps data unmodified).
  /// Base2 (alphabet: 01).
  Base2,
  /// Base8 (alphabet: 01234567).
  Base8,
  /// Base10 (alphabet: 0123456789).
  Base10,
  /// Base16 lower hexadecimal (alphabet: 0123456789abcdef).
  Base16Lower,
  /// Base16 upper hexadecimal (alphabet: 0123456789ABCDEF).
  Base16Upper,
  /// Base32, rfc4648 no padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
  Base32Lower,
  /// Base32, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
  Base32Upper,
  /// Base32, rfc4648 with padding (alphabet: abcdefghijklmnopqrstuvwxyz234567).
  Base32PadLower,
  /// Base32, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZ234567).
  Base32PadUpper,
  /// Base32hex, rfc4648 no padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
  Base32HexLower,
  /// Base32hex, rfc4648 no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
  Base32HexUpper,
  /// Base32hex, rfc4648 with padding (alphabet: 0123456789abcdefghijklmnopqrstuv).
  Base32HexPadLower,
  /// Base32hex, rfc4648 with padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUV).
  Base32HexPadUpper,
  /// z-base-32 (used by Tahoe-LAFS) (alphabet: ybndrfg8ejkmcpqxot1uwisza345h769).
  Base32Z,
  /// Base36, [0-9a-z] no padding (alphabet: 0123456789abcdefghijklmnopqrstuvwxyz).
  Base36Lower,
  /// Base36, [0-9A-Z] no padding (alphabet: 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ).
  Base36Upper,
  /// Base58 flicker (alphabet: 123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ).
  Base58Flickr,
  /// Base58 bitcoin (alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz).
  Base58Btc,
  /// Base64, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
  Base64,
  /// Base64, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/).
  Base64Pad,
  /// Base64 url, rfc4648 no padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
  Base64Url,
  /// Base64 url, rfc4648 with padding (alphabet: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_).
  Base64UrlPad,
}

/// Wrap [multibase::Base] to exclude the identity (0x00) and avoid exporting from a pre-1.0 crate.
impl From<Base> for multibase::Base {
  fn from(base: Base) -> Self {
    match base {
      Base::Base2 => multibase::Base::Base2,
      Base::Base8 => multibase::Base::Base8,
      Base::Base10 => multibase::Base::Base10,
      Base::Base16Lower => multibase::Base::Base16Lower,
      Base::Base16Upper => multibase::Base::Base16Upper,
      Base::Base32Lower => multibase::Base::Base32Lower,
      Base::Base32Upper => multibase::Base::Base32Upper,
      Base::Base32PadLower => multibase::Base::Base32PadLower,
      Base::Base32PadUpper => multibase::Base::Base32PadUpper,
      Base::Base32HexLower => multibase::Base::Base32HexLower,
      Base::Base32HexUpper => multibase::Base::Base32HexUpper,
      Base::Base32HexPadLower => multibase::Base::Base32HexPadLower,
      Base::Base32HexPadUpper => multibase::Base::Base32HexPadUpper,
      Base::Base32Z => multibase::Base::Base32Z,
      Base::Base36Lower => multibase::Base::Base36Lower,
      Base::Base36Upper => multibase::Base::Base36Upper,
      Base::Base58Flickr => multibase::Base::Base58Flickr,
      Base::Base58Btc => multibase::Base::Base58Btc,
      Base::Base64 => multibase::Base::Base64,
      Base::Base64Pad => multibase::Base::Base64Pad,
      Base::Base64Url => multibase::Base::Base64Url,
      Base::Base64UrlPad => multibase::Base::Base64UrlPad,
    }
  }
}

/// Provides utility functions for encoding and decoding between various bases.
pub struct BaseEncoding;

impl BaseEncoding {
  /// Encodes the given `data` to the specified [`base`](Base).
  pub fn encode<T>(data: &T, base: Base) -> String
  where
    T: AsRef<[u8]> + ?Sized,
  {
    multibase::Base::from(base).encode(data)
  }

  /// Decodes the given `data` encoded as the specified [`base`](Base).
  pub fn decode<T>(data: &T, base: Base) -> Result<Vec<u8>>
  where
    T: AsRef<str> + ?Sized,
  {
    multibase::Base::from(base)
      .decode(data)
      .map_err(|err| Error::DecodeBase(base, err))
  }

  /// Encodes the given `data` to [`Base::Base58Btc`].
  ///
  /// Equivalent to `encode(data, Base58Btc)`.
  pub fn encode_base58<T>(data: &T) -> String
  where
    T: AsRef<[u8]> + ?Sized,
  {
    Self::encode(data, Base::Base58Btc)
  }

  /// Decodes the given `data` encoded as [`Base::Base58Btc`].
  ///
  /// Equivalent to `decode(data, Base58Btc)`.
  pub fn decode_base58<T>(data: &T) -> Result<Vec<u8>>
  where
    T: AsRef<str> + ?Sized,
  {
    Self::decode(data, Base::Base58Btc)
  }

  /// Encodes the given `data` as [Multibase] with the given [`base`](Base), defaults to
  /// [`Base::Base58Btc`] if omitted.
  ///
  /// NOTE: [`encode_multibase`](Self::encode_multibase) is different from [`encode`](Self::encode) because the
  /// [Multibase] format prepends a character representing the base-encoding to the output.
  ///
  /// [Multibase]: https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03
  pub fn encode_multibase<T>(data: &T, base: Option<Base>) -> String
  where
    T: AsRef<[u8]> + ?Sized,
  {
    multibase::encode(multibase::Base::from(base.unwrap_or(Base::Base58Btc)), data)
  }

  /// Decodes the given `data` encoded as [Multibase], with the [`base`](Base) inferred from the
  /// leading character.
  ///
  /// [Multibase]: https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03
  pub fn decode_multibase<T>(data: &T) -> Result<Vec<u8>>
  where
    T: AsRef<str> + ?Sized,
  {
    if data.as_ref().is_empty() {
      return Ok(Vec::new());
    }
    multibase::decode(data)
      .map(|(_base, output)| output)
      .map_err(Error::DecodeMultibase)
  }
}

#[cfg(test)]
mod tests {
  use quickcheck_macros::quickcheck;

  use super::*;

  #[test]
  fn test_decode_base58_empty() {
    assert_eq!(BaseEncoding::decode_base58("").unwrap(), Vec::<u8>::new());
  }

  #[quickcheck]
  fn test_base58_random(data: Vec<u8>) {
    assert_eq!(
      BaseEncoding::decode_base58(&BaseEncoding::encode_base58(&data)).unwrap(),
      data
    );
  }

  #[quickcheck]
  fn test_base64_random(data: Vec<u8>) {
    assert_eq!(
      BaseEncoding::decode(&BaseEncoding::encode(&data, Base::Base64Url), Base::Base64Url).unwrap(),
      data
    );
  }

  /// Base58 test vectors from Internet Engineering Task Force (IETF) Draft.
  /// https://datatracker.ietf.org/doc/html/draft-msporny-base58-02#section-5
  #[test]
  fn test_b58() {
    let test_vectors: [(&[u8], &str); 3] = [
      (b"Hello World!", "2NEpo7TZRRrLZSi2U"),
      (
        b"The quick brown fox jumps over the lazy dog.",
        "USm3fpXnKG5EUBx2ndxBDMPVciP5hGey2Jh4NDv6gmeo1LkMeiKrLJUUBk6Z",
      ),
      (&[0, 0, 0, 40, 127, 180, 205], "111233QC4"),
    ];
    for (test_decoded, test_encoded) in test_vectors {
      let encoded: String = BaseEncoding::encode_base58(test_decoded);
      assert_eq!(encoded, test_encoded, "encode failed on {test_decoded:?}");

      let decoded: Vec<u8> = BaseEncoding::decode_base58(test_encoded).unwrap();
      assert_eq!(decoded, test_decoded, "decode failed on {test_encoded}");
    }
  }

  #[test]
  fn test_decode_multibase_empty() {
    assert_eq!(BaseEncoding::decode_multibase("").unwrap(), Vec::<u8>::new());
  }

  #[quickcheck]
  fn test_multibase_random(data: Vec<u8>) {
    assert_eq!(
      BaseEncoding::decode_multibase(&BaseEncoding::encode_multibase(&data, None)).unwrap(),
      data
    );
  }

  #[quickcheck]
  fn test_multibase_bases_random(data: Vec<u8>) {
    let bases = [
      Base::Base2,
      Base::Base8,
      Base::Base10,
      Base::Base16Lower,
      Base::Base16Upper,
      Base::Base32Lower,
      Base::Base32Upper,
      Base::Base32PadLower,
      Base::Base32PadUpper,
      Base::Base32HexLower,
      Base::Base32HexUpper,
      Base::Base32HexPadLower,
      Base::Base32HexPadUpper,
      Base::Base32Z,
      Base::Base36Lower,
      Base::Base36Upper,
      Base::Base58Flickr,
      Base::Base58Btc,
      Base::Base64,
      Base::Base64Pad,
      Base::Base64Url,
      Base::Base64UrlPad,
    ];
    for base in bases {
      assert_eq!(
        BaseEncoding::decode_multibase(&BaseEncoding::encode_multibase(&data, Some(base))).unwrap(),
        data
      );
    }
  }

  /// Multibase test vectors from Internet Engineering Task Force (IETF) draft.
  /// https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03#appendix-B
  #[test]
  fn test_multibase() {
    // Encode.
    let data: &str = r"Multibase is awesome! \o/";
    for (base, expected) in [
      (Base::Base16Upper, "F4D756C74696261736520697320617765736F6D6521205C6F2F"),
      (Base::Base32Upper, "BJV2WY5DJMJQXGZJANFZSAYLXMVZW63LFEEQFY3ZP"),
      (Base::Base58Btc, "zYAjKoNbau5KiqmHPmSxYCvn66dA1vLmwbt"),
      (Base::Base64Pad, "MTXVsdGliYXNlIGlzIGF3ZXNvbWUhIFxvLw=="),
    ] {
      let encoded: String = BaseEncoding::encode_multibase(data, Some(base));
      assert_eq!(encoded, expected);
    }

    // Decode.
    let expected: Vec<u8> = data.as_bytes().to_vec();
    for encoded in [
      "F4D756C74696261736520697320617765736F6D6521205C6F2F",
      "BJV2WY5DJMJQXGZJANFZSAYLXMVZW63LFEEQFY3ZP",
      "zYAjKoNbau5KiqmHPmSxYCvn66dA1vLmwbt",
      "MTXVsdGliYXNlIGlzIGF3ZXNvbWUhIFxvLw==",
    ] {
      let decoded: Vec<u8> = BaseEncoding::decode_multibase(encoded).unwrap();
      assert_eq!(decoded, expected, "failed on {encoded}");
    }
  }
}
