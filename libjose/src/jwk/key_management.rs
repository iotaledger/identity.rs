// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::jwe::JweAlgorithm;

/// Type of cryptographic algorithm used to encrypt or determine the Content
/// Encryption Key (CEK).
///
/// [More Info](https://tools.ietf.org/html/rfc7518#section-4)
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyManagement {
  /// A Key Management Mode in which the CEK value is encrypted to
  /// the intended recipient using an asymmetric encryption algorithm.
  EncryptionAsymmetric,
  /// A Key Management Mode in which the CEK value is encrypted to the
  /// intended recipient using a symmetric key wrapping algorithm.
  KeyWrapSymmetric,
  /// A Key Management Mode in which a key agreement algorithm is used to
  /// agree upon the CEK value.
  KeyAgreementDirect,
  /// A Key Management Mode in which a key agreement algorithm is used to
  /// agree upon a symmetric key used to encrypt the CEK value to the
  /// intended recipient using a symmetric key wrapping algorithm.
  KeyAgreementKeyWrap,
  /// A Key Management Mode in which the CEK value used is the secret
  /// symmetric key value shared between the parties.
  EncryptionDirect,
}

impl KeyManagement {
  /// Returns true if this management mode uses a direct key value.
  pub const fn is_direct(self) -> bool {
    matches!(self, Self::KeyAgreementDirect | Self::EncryptionDirect)
  }
}

impl From<JweAlgorithm> for KeyManagement {
  fn from(other: JweAlgorithm) -> Self {
    match other {
      JweAlgorithm::RSA1_5 => Self::EncryptionAsymmetric,
      JweAlgorithm::RSA_OAEP => Self::EncryptionAsymmetric,
      JweAlgorithm::RSA_OAEP_256 => Self::EncryptionAsymmetric,
      JweAlgorithm::RSA_OAEP_384 => Self::EncryptionAsymmetric,
      JweAlgorithm::RSA_OAEP_512 => Self::EncryptionAsymmetric,
      JweAlgorithm::A128KW => Self::KeyWrapSymmetric,
      JweAlgorithm::A192KW => Self::KeyWrapSymmetric,
      JweAlgorithm::A256KW => Self::KeyWrapSymmetric,
      JweAlgorithm::DIR => Self::EncryptionDirect,
      JweAlgorithm::ECDH_ES => Self::KeyAgreementDirect,
      JweAlgorithm::ECDH_ES_A128KW => Self::KeyAgreementKeyWrap,
      JweAlgorithm::ECDH_ES_A192KW => Self::KeyAgreementKeyWrap,
      JweAlgorithm::ECDH_ES_A256KW => Self::KeyAgreementKeyWrap,
      JweAlgorithm::ECDH_ES_C20PKW => Self::KeyAgreementKeyWrap,
      JweAlgorithm::ECDH_ES_XC20PKW => Self::KeyAgreementKeyWrap,
      JweAlgorithm::A128GCMKW => Self::KeyWrapSymmetric,
      JweAlgorithm::A192GCMKW => Self::KeyWrapSymmetric,
      JweAlgorithm::A256GCMKW => Self::KeyWrapSymmetric,
      JweAlgorithm::PBES2_HS256_A128KW => Self::KeyWrapSymmetric,
      JweAlgorithm::PBES2_HS384_A192KW => Self::KeyWrapSymmetric,
      JweAlgorithm::PBES2_HS512_A256KW => Self::KeyWrapSymmetric,
      JweAlgorithm::ECDH_1PU => Self::KeyAgreementDirect,
      JweAlgorithm::ECDH_1PU_A128KW => Self::KeyAgreementKeyWrap,
      JweAlgorithm::ECDH_1PU_A192KW => Self::KeyAgreementKeyWrap,
      JweAlgorithm::ECDH_1PU_A256KW => Self::KeyAgreementKeyWrap,
      JweAlgorithm::C20PKW => Self::KeyWrapSymmetric,
      JweAlgorithm::XC20PKW => Self::KeyWrapSymmetric,
    }
  }
}
