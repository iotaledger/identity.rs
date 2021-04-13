// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;
use crypto::ciphers::aes::Aes128Gcm;
use crypto::ciphers::aes::Aes192Gcm;
use crypto::ciphers::aes::Aes256Gcm;
use crypto::ciphers::aes_cbc::Aes128CbcHmac256;
use crypto::ciphers::aes_cbc::Aes192CbcHmac384;
use crypto::ciphers::aes_cbc::Aes256CbcHmac512;
use crypto::ciphers::chacha::ChaCha20Poly1305;
use crypto::ciphers::chacha::XChaCha20Poly1305;
use crypto::ciphers::traits::Aead;

/// Supported algorithms for the JSON Web Encryption `enc` claim.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-signature-encryption-algorithms)
///
/// [ChaCha20-Poly1305 (draft)](https://tools.ietf.org/html/draft-amringer-jose-chacha-02)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum JweEncryption {
  /// AES_128_CBC_HMAC_SHA_256 authenticated encryption algorithm.
  #[serde(rename = "A128CBC-HS256")]
  A128CBC_HS256,
  /// AES_192_CBC_HMAC_SHA_384 authenticated encryption algorithm.
  #[serde(rename = "A192CBC-HS384")]
  A192CBC_HS384,
  /// AES_256_CBC_HMAC_SHA_512 authenticated encryption algorithm.
  #[serde(rename = "A256CBC-HS512")]
  A256CBC_HS512,
  /// AES GCM using 128-bit key.
  A128GCM,
  /// AES GCM using 192-bit key.
  A192GCM,
  /// AES GCM using 256-bit key.
  A256GCM,
  /// ChaCha20-Poly1305.
  C20P,
  /// ChaCha20-Poly1305.
  XC20P,
}

impl JweEncryption {
  pub const ALL: &'static [Self] = &[
    Self::A128CBC_HS256,
    Self::A192CBC_HS384,
    Self::A256CBC_HS512,
    Self::A128GCM,
    Self::A192GCM,
    Self::A256GCM,
    Self::C20P,
    Self::XC20P,
  ];

  /// Returns the JWE "enc" claim as a `str` slice.
  pub const fn name(self) -> &'static str {
    match self {
      Self::A128CBC_HS256 => "A128CBC-HS256",
      Self::A192CBC_HS384 => "A192CBC-HS384",
      Self::A256CBC_HS512 => "A256CBC-HS512",
      Self::A128GCM => "A128GCM",
      Self::A192GCM => "A192GCM",
      Self::A256GCM => "A256GCM",
      Self::C20P => "C20P",
      Self::XC20P => "XC20P",
    }
  }

  pub const fn key_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 => Aes128CbcHmac256::KEY_LENGTH,
      Self::A192CBC_HS384 => Aes192CbcHmac384::KEY_LENGTH,
      Self::A256CBC_HS512 => Aes256CbcHmac512::KEY_LENGTH,
      Self::A128GCM => Aes128Gcm::KEY_LENGTH,
      Self::A192GCM => Aes192Gcm::KEY_LENGTH,
      Self::A256GCM => Aes256Gcm::KEY_LENGTH,
      Self::C20P => ChaCha20Poly1305::KEY_LENGTH,
      Self::XC20P => XChaCha20Poly1305::KEY_LENGTH,
    }
  }

  pub const fn iv_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 => Aes128CbcHmac256::NONCE_LENGTH,
      Self::A192CBC_HS384 => Aes192CbcHmac384::NONCE_LENGTH,
      Self::A256CBC_HS512 => Aes256CbcHmac512::NONCE_LENGTH,
      Self::A128GCM => Aes128Gcm::NONCE_LENGTH,
      Self::A192GCM => Aes192Gcm::NONCE_LENGTH,
      Self::A256GCM => Aes256Gcm::NONCE_LENGTH,
      Self::C20P => ChaCha20Poly1305::NONCE_LENGTH,
      Self::XC20P => XChaCha20Poly1305::NONCE_LENGTH,
    }
  }

  pub const fn tag_len(self) -> usize {
    match self {
      Self::A128CBC_HS256 => Aes128CbcHmac256::TAG_LENGTH,
      Self::A192CBC_HS384 => Aes192CbcHmac384::TAG_LENGTH,
      Self::A256CBC_HS512 => Aes256CbcHmac512::TAG_LENGTH,
      Self::A128GCM => Aes128Gcm::TAG_LENGTH,
      Self::A192GCM => Aes192Gcm::TAG_LENGTH,
      Self::A256GCM => Aes256Gcm::TAG_LENGTH,
      Self::C20P => ChaCha20Poly1305::TAG_LENGTH,
      Self::XC20P => XChaCha20Poly1305::TAG_LENGTH,
    }
  }
}

impl Display for JweEncryption {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.write_str(self.name())
  }
}
