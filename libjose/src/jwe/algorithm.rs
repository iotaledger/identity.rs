// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt::Display;
use core::fmt::Formatter;

use crypto::ciphers::aes::Aes128Gcm;
use crypto::ciphers::aes::Aes192Gcm;
use crypto::ciphers::aes::Aes256Gcm;
use crypto::ciphers::chacha::ChaCha20Poly1305;
use crypto::ciphers::chacha::XChaCha20Poly1305;
use crypto::ciphers::traits::Aead;

use crate::error::Error;
use crate::error::Result;

/// Supported algorithms for the JSON Web Encryption `alg` claim.
///
/// [More Info](https://www.iana.org/assignments/jose/jose.xhtml#web-signature-encryption-algorithms)
///
/// [ChaCha20-Poly1305 (draft)](https://tools.ietf.org/html/draft-amringer-jose-chacha-02)
///
/// [ECDH-1PU (draft)](https://tools.ietf.org/html/draft-madden-jose-ecdh-1pu-03)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum JweAlgorithm {
  /// RSAES-PKCS1-v1_5
  RSA1_5,
  /// RSAES OAEP using default parameters
  #[serde(rename = "RSA-OAEP")]
  RSA_OAEP,
  /// RSAES OAEP using SHA-256 and MGF1 with SHA-256
  #[serde(rename = "RSA-OAEP-256")]
  RSA_OAEP_256,
  /// RSA-OAEP using SHA-384 and MGF1 with SHA-384
  #[serde(rename = "RSA-OAEP-384")]
  RSA_OAEP_384,
  /// RSA-OAEP using SHA-512 and MGF1 with SHA-512
  #[serde(rename = "RSA-OAEP-512")]
  RSA_OAEP_512,
  /// AES Key Wrap with default initial value using 128-bit key
  A128KW,
  /// AES Key Wrap with default initial value using 192-bit key
  A192KW,
  /// AES Key Wrap with default initial value using 256-bit key
  A256KW,
  /// Direct use of a shared symmetric key as the CEK
  #[serde(rename = "dir")]
  DIR,
  /// Elliptic Curve Diffie-Hellman Ephemeral Static key agreement using Concat
  /// KDF
  #[serde(rename = "ECDH-ES")]
  ECDH_ES,
  /// ECDH-ES using Concat KDF and CEK wrapped with "A128KW"
  #[serde(rename = "ECDH-ES+A128KW")]
  ECDH_ES_A128KW,
  /// ECDH-ES using Concat KDF and CEK wrapped with "A192KW"
  #[serde(rename = "ECDH-ES+A192KW")]
  ECDH_ES_A192KW,
  /// ECDH-ES using Concat KDF and CEK  wrapped with "A256KW"
  #[serde(rename = "ECDH-ES+A256KW")]
  ECDH_ES_A256KW,
  /// ECDH-ES using Concat KDF and CEK wrapped with C20PKW
  #[serde(rename = "ECDH-ES+C20PKW")]
  ECDH_ES_C20PKW,
  /// ECDH-ES using Concat KDF and CEK wrapped with XC20PKW
  #[serde(rename = "ECDH-ES+XC20PKW")]
  ECDH_ES_XC20PKW,
  /// Key wrapping with AES GCM using 128-bit key
  A128GCMKW,
  /// Key wrapping with AES GCM using 192-bit key
  A192GCMKW,
  /// Key wrapping with AES GCM using 256-bit key
  A256GCMKW,
  /// PBES2 with HMAC SHA-256 and "A128KW" wrapping
  #[serde(rename = "PBES2-HS256+A128KW")]
  PBES2_HS256_A128KW,
  /// PBES2 with HMAC SHA-384 and "A192KW" wrapping
  #[serde(rename = "PBES2-HS384+A192KW")]
  PBES2_HS384_A192KW,
  /// PBES2 with HMAC SHA-512 and "A256KW" wrapping
  #[serde(rename = "PBES2-HS512+A256KW")]
  PBES2_HS512_A256KW,
  /// ECDH One-Pass Unified Model using one-pass KDF
  #[serde(rename = "ECDH-1PU")]
  ECDH_1PU,
  /// ECDH-1PU using one-pass KDF and CEK wrapped with "A128KW"
  #[serde(rename = "ECDH-1PU+A128KW")]
  ECDH_1PU_A128KW,
  /// ECDH-1PU using one-pass KDF and CEK wrapped with "A192KW"
  #[serde(rename = "ECDH-1PU+A192KW")]
  ECDH_1PU_A192KW,
  /// ECDH-1PU using one-pass KDF and CEK wrapped with "A256KW"
  #[serde(rename = "ECDH-1PU+A256KW")]
  ECDH_1PU_A256KW,
  /// Key wrapping with ChaCha20-Poly1305
  C20PKW,
  /// Key wrapping with XChaCha20-Poly1305
  XC20PKW,
}

impl JweAlgorithm {
  pub const ALL: &'static [JweAlgorithm] = &[
    Self::RSA1_5,
    Self::RSA_OAEP,
    Self::RSA_OAEP_256,
    Self::RSA_OAEP_384,
    Self::RSA_OAEP_512,
    Self::A128KW,
    Self::A192KW,
    Self::A256KW,
    Self::DIR,
    Self::ECDH_ES,
    Self::ECDH_ES_A128KW,
    Self::ECDH_ES_A192KW,
    Self::ECDH_ES_A256KW,
    Self::ECDH_ES_C20PKW,
    Self::ECDH_ES_XC20PKW,
    Self::A128GCMKW,
    Self::A192GCMKW,
    Self::A256GCMKW,
    Self::PBES2_HS256_A128KW,
    Self::PBES2_HS384_A192KW,
    Self::PBES2_HS512_A256KW,
    Self::ECDH_1PU,
    Self::ECDH_1PU_A128KW,
    Self::ECDH_1PU_A192KW,
    Self::ECDH_1PU_A256KW,
    Self::C20PKW,
    Self::XC20PKW,
  ];

  /// Returns the JWE algorithm as a `str` slice.
  pub const fn name(self) -> &'static str {
    match self {
      Self::RSA1_5 => "RSA1_5",
      Self::RSA_OAEP => "RSA-OAEP",
      Self::RSA_OAEP_256 => "RSA-OAEP-256",
      Self::RSA_OAEP_384 => "RSA-OAEP-384",
      Self::RSA_OAEP_512 => "RSA-OAEP-512",
      Self::A128KW => "A128KW",
      Self::A192KW => "A192KW",
      Self::A256KW => "A256KW",
      Self::DIR => "dir",
      Self::ECDH_ES => "ECDH-ES",
      Self::ECDH_ES_A128KW => "ECDH-ES+A128KW",
      Self::ECDH_ES_A192KW => "ECDH-ES+A192KW",
      Self::ECDH_ES_A256KW => "ECDH-ES+A256KW",
      Self::ECDH_ES_C20PKW => "ECDH-ES+C20PKW",
      Self::ECDH_ES_XC20PKW => "ECDH-ES+XC20PKW",
      Self::A128GCMKW => "A128GCMKW",
      Self::A192GCMKW => "A192GCMKW",
      Self::A256GCMKW => "A256GCMKW",
      Self::PBES2_HS256_A128KW => "PBES2-HS256+A128KW",
      Self::PBES2_HS384_A192KW => "PBES2-HS384+A192KW",
      Self::PBES2_HS512_A256KW => "PBES2-HS512+A256KW",
      Self::ECDH_1PU => "ECDH-1PU",
      Self::ECDH_1PU_A128KW => "ECDH-1PU+A128KW",
      Self::ECDH_1PU_A192KW => "ECDH-1PU+A192KW",
      Self::ECDH_1PU_A256KW => "ECDH-1PU+A256KW",
      Self::C20PKW => "C20PKW",
      Self::XC20PKW => "XC20PKW",
    }
  }

  pub const fn key_len(self) -> Option<usize> {
    match self {
      Self::RSA1_5 => None,
      Self::RSA_OAEP => None,
      Self::RSA_OAEP_256 => None,
      Self::RSA_OAEP_384 => None,
      Self::RSA_OAEP_512 => None,
      Self::A128KW => Some(Aes128Gcm::KEY_LENGTH),
      Self::A192KW => Some(Aes192Gcm::KEY_LENGTH),
      Self::A256KW => Some(Aes256Gcm::KEY_LENGTH),
      Self::DIR => None,
      Self::ECDH_ES => None,
      Self::ECDH_ES_A128KW => Some(Aes128Gcm::KEY_LENGTH),
      Self::ECDH_ES_A192KW => Some(Aes192Gcm::KEY_LENGTH),
      Self::ECDH_ES_A256KW => Some(Aes256Gcm::KEY_LENGTH),
      Self::ECDH_ES_C20PKW => Some(ChaCha20Poly1305::KEY_LENGTH),
      Self::ECDH_ES_XC20PKW => Some(XChaCha20Poly1305::KEY_LENGTH),
      Self::A128GCMKW => Some(Aes128Gcm::KEY_LENGTH),
      Self::A192GCMKW => Some(Aes192Gcm::KEY_LENGTH),
      Self::A256GCMKW => Some(Aes256Gcm::KEY_LENGTH),
      Self::PBES2_HS256_A128KW => Some(Aes128Gcm::KEY_LENGTH),
      Self::PBES2_HS384_A192KW => Some(Aes192Gcm::KEY_LENGTH),
      Self::PBES2_HS512_A256KW => Some(Aes256Gcm::KEY_LENGTH),
      Self::ECDH_1PU => None,
      Self::ECDH_1PU_A128KW => Some(Aes128Gcm::KEY_LENGTH),
      Self::ECDH_1PU_A192KW => Some(Aes192Gcm::KEY_LENGTH),
      Self::ECDH_1PU_A256KW => Some(Aes256Gcm::KEY_LENGTH),
      Self::C20PKW => Some(ChaCha20Poly1305::KEY_LENGTH),
      Self::XC20PKW => Some(XChaCha20Poly1305::KEY_LENGTH),
    }
  }

  pub fn try_key_len(self) -> Result<usize> {
    self.key_len().ok_or_else(|| Error::KeyError(self.name()))
  }
}

impl Display for JweAlgorithm {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.name())
  }
}
