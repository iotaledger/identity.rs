// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::KeyType;
use identity_core::crypto::PrivateKey;
use identity_core::crypto::PublicKey;

/// Method content for creating new verification methods.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum MethodContent {
  /// Generate and store a new Ed25519 keypair for a new
  /// [`Ed25519VerificationKey2018`](identity_did::verification::MethodType::ED25519_VERIFICATION_KEY_2018)
  /// method.
  GenerateEd25519,
  /// Store an existing Ed25519 private key and derive a public key from it for a new
  /// [`Ed25519VerificationKey2018`](identity_did::verification::MethodType::ED25519_VERIFICATION_KEY_2018)
  /// method.
  PrivateEd25519(PrivateKey),
  /// Insert an existing Ed25519 public key into a new
  /// [`Ed25519VerificationKey2018`](identity_did::verification::MethodType::ED25519_VERIFICATION_KEY_2018)
  /// method, without generating or storing a private key.
  ///
  /// NOTE: the method will be unable to be used to sign anything without a private key.
  PublicEd25519(PublicKey),
  /// Generate and store a new X25519 keypair for a new
  /// [`X25519KeyAgreementKey2019`](identity_did::verification::MethodType::X25519_KEY_AGREEMENT_KEY_2019)
  /// method.
  GenerateX25519,
  /// Store an existing X25519 private key and derive a public key from it for a new
  /// [`X25519KeyAgreementKey2019`](identity_did::verification::MethodType::X25519_KEY_AGREEMENT_KEY_2019)
  /// method.
  PrivateX25519(PrivateKey),
  /// Insert an existing X25519 public key into a new
  /// [`X25519KeyAgreementKey2019`](identity_did::verification::MethodType::X25519_KEY_AGREEMENT_KEY_2019)
  /// method, without generating or storing a private key.
  ///
  /// NOTE: the method will be unable to be used for key exchange without a private key.
  PublicX25519(PublicKey),
}

impl MethodContent {
  /// Returns the [`MethodType`](identity_did::verification::MethodType) associated with the `MethodContent` variant.
  #[cfg(test)]
  pub(crate) fn method_type(&self) -> identity_did::verification::MethodType {
    match self {
      MethodContent::GenerateEd25519 => identity_did::verification::MethodType::ED25519_VERIFICATION_KEY_2018,
      MethodContent::PrivateEd25519(_) => identity_did::verification::MethodType::ED25519_VERIFICATION_KEY_2018,
      MethodContent::PublicEd25519(_) => identity_did::verification::MethodType::ED25519_VERIFICATION_KEY_2018,
      MethodContent::GenerateX25519 => identity_did::verification::MethodType::X25519_KEY_AGREEMENT_KEY_2019,
      MethodContent::PrivateX25519(_) => identity_did::verification::MethodType::X25519_KEY_AGREEMENT_KEY_2019,
      MethodContent::PublicX25519(_) => identity_did::verification::MethodType::X25519_KEY_AGREEMENT_KEY_2019,
    }
  }

  /// Returns the [`KeyType`] associated with the `MethodContent` variant.
  pub(crate) fn key_type(&self) -> KeyType {
    match self {
      MethodContent::GenerateEd25519 => KeyType::Ed25519,
      MethodContent::PrivateEd25519(_) => KeyType::Ed25519,
      MethodContent::PublicEd25519(_) => KeyType::Ed25519,
      MethodContent::GenerateX25519 => KeyType::X25519,
      MethodContent::PrivateX25519(_) => KeyType::X25519,
      MethodContent::PublicX25519(_) => KeyType::X25519,
    }
  }
}
