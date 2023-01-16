// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// use crypto::ciphers::aes::Aes128Gcm;
// use crypto::ciphers::aes::Aes192Gcm;
// use crypto::ciphers::aes::Aes256Gcm;
// use crypto::ciphers::chacha::ChaCha20Poly1305;
// use crypto::ciphers::chacha::XChaCha20Poly1305;
// use crypto::ciphers::traits::Aead;
use crypto::hashes::sha::SHA256_LEN;
use crypto::hashes::sha::SHA384_LEN;
use crypto::hashes::sha::SHA512_LEN;

// use crate::jwe::JweAlgorithm;
// use crate::jwe::JweEncryption;
use crate::jwk::EcCurve;
use crate::jwk::EcdhCurve;
use crate::jwk::EcxCurve;
use crate::jwk::EdCurve;
use crate::jws::JwsAlgorithm;

/// Supported sizes for RSA key generation.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum RsaBits {
  B2048 = 2048,
  B3072 = 3072,
  B4096 = 4096,
}

impl RsaBits {
  pub const fn bits(self) -> usize {
    match self {
      Self::B2048 => 2048,
      Self::B3072 => 3072,
      Self::B4096 => 4096,
    }
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyParams {
  None,
  Oct(usize),
  Rsa(RsaBits),
  Ec(EcCurve),
  Ecx(EcxCurve),
  Ed(EdCurve),
}

impl From<usize> for KeyParams {
  fn from(other: usize) -> Self {
    Self::Oct(other)
  }
}

impl From<RsaBits> for KeyParams {
  fn from(other: RsaBits) -> Self {
    Self::Rsa(other)
  }
}

impl From<EcCurve> for KeyParams {
  fn from(other: EcCurve) -> Self {
    Self::Ec(other)
  }
}

impl From<EcxCurve> for KeyParams {
  fn from(other: EcxCurve) -> Self {
    Self::Ecx(other)
  }
}

impl From<EdCurve> for KeyParams {
  fn from(other: EdCurve) -> Self {
    Self::Ed(other)
  }
}

impl From<EcdhCurve> for KeyParams {
  fn from(other: EcdhCurve) -> Self {
    match other {
      EcdhCurve::Ec(curve) => Self::Ec(curve),
      EcdhCurve::Ecx(curve) => Self::Ecx(curve),
    }
  }
}

impl From<JwsAlgorithm> for KeyParams {
  fn from(other: JwsAlgorithm) -> Self {
    match other {
      JwsAlgorithm::HS256 => Self::Oct(SHA256_LEN),
      JwsAlgorithm::HS384 => Self::Oct(SHA384_LEN),
      JwsAlgorithm::HS512 => Self::Oct(SHA512_LEN),
      JwsAlgorithm::RS256 => Self::Rsa(RsaBits::B2048),
      JwsAlgorithm::RS384 => Self::Rsa(RsaBits::B2048),
      JwsAlgorithm::RS512 => Self::Rsa(RsaBits::B2048),
      JwsAlgorithm::PS256 => Self::Rsa(RsaBits::B2048),
      JwsAlgorithm::PS384 => Self::Rsa(RsaBits::B2048),
      JwsAlgorithm::PS512 => Self::Rsa(RsaBits::B2048),
      JwsAlgorithm::ES256 => Self::Ec(EcCurve::P256),
      JwsAlgorithm::ES384 => Self::Ec(EcCurve::P384),
      JwsAlgorithm::ES512 => Self::Ec(EcCurve::P521),
      JwsAlgorithm::ES256K => Self::Ec(EcCurve::Secp256K1),
      JwsAlgorithm::NONE => Self::None,
      JwsAlgorithm::EdDSA => Self::Ed(EdCurve::Ed25519),
    }
  }
}

// Can be uncommented when porting the jwe module from libjose to this crate.
/*
impl From<(JweAlgorithm, JweEncryption)> for KeyParams {
  fn from(other: (JweAlgorithm, JweEncryption)) -> Self {
    match other {
      (JweAlgorithm::RSA1_5, _) => Self::Rsa(RsaBits::B2048),
      (JweAlgorithm::RSA_OAEP, _) => Self::Rsa(RsaBits::B2048),
      (JweAlgorithm::RSA_OAEP_256, _) => Self::Rsa(RsaBits::B2048),
      (JweAlgorithm::RSA_OAEP_384, _) => Self::Rsa(RsaBits::B2048),
      (JweAlgorithm::RSA_OAEP_512, _) => Self::Rsa(RsaBits::B2048),
      (JweAlgorithm::A128KW, _) => Self::Oct(Aes128Gcm::KEY_LENGTH),
      (JweAlgorithm::A192KW, _) => Self::Oct(Aes192Gcm::KEY_LENGTH),
      (JweAlgorithm::A256KW, _) => Self::Oct(Aes256Gcm::KEY_LENGTH),
      (JweAlgorithm::DIR, encryption) => Self::Oct(encryption.key_len()),
      (JweAlgorithm::ECDH_ES, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::ECDH_ES_A128KW, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::ECDH_ES_A192KW, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::ECDH_ES_A256KW, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::ECDH_ES_C20PKW, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::ECDH_ES_XC20PKW, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::A128GCMKW, _) => Self::Oct(Aes128Gcm::KEY_LENGTH),
      (JweAlgorithm::A192GCMKW, _) => Self::Oct(Aes192Gcm::KEY_LENGTH),
      (JweAlgorithm::A256GCMKW, _) => Self::Oct(Aes256Gcm::KEY_LENGTH),
      (JweAlgorithm::PBES2_HS256_A128KW, _) => Self::Oct(Aes128Gcm::KEY_LENGTH),
      (JweAlgorithm::PBES2_HS384_A192KW, _) => Self::Oct(Aes192Gcm::KEY_LENGTH),
      (JweAlgorithm::PBES2_HS512_A256KW, _) => Self::Oct(Aes256Gcm::KEY_LENGTH),
      (JweAlgorithm::ECDH_1PU, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::ECDH_1PU_A128KW, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::ECDH_1PU_A192KW, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::ECDH_1PU_A256KW, _) => Self::Ecx(EcxCurve::X25519),
      (JweAlgorithm::C20PKW, _) => Self::Oct(ChaCha20Poly1305::KEY_LENGTH),
      (JweAlgorithm::XC20PKW, _) => Self::Oct(XChaCha20Poly1305::KEY_LENGTH),
    }
  }
}
*/
