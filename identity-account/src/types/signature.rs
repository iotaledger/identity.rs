// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;

/// A digital signature and associated public key.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature {
  #[serde(with = "identity_core::utils::public_key_serde")]
  pub(crate) pkey: PublicKey,
  pub(crate) data: Vec<u8>,
}

impl Signature {
  /// Creates a new `Signature`.
  pub const fn new(pkey: PublicKey, data: Vec<u8>) -> Self {
    Self { pkey, data }
  }

  /// Returns the public key used to verify this signature.
  pub fn pkey(&self) -> &PublicKey {
    &self.pkey
  }

  /// Returns the the signature data as a slice of bytes.
  pub fn data(&self) -> &[u8] {
    &self.data
  }
}
