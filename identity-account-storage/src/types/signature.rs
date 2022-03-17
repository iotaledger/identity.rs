// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;
#[cfg(feature = "bindings-derive")]
use serde::Deserialize;
#[cfg(feature = "bindings-derive")]
use serde::Serialize;

/// A digital signature and associated public key.
#[cfg_attr(feature = "bindings-derive", derive(Clone, Deserialize, Serialize))]
#[cfg_attr(not(feature = "bindings-derive"), derive(Clone))]
pub struct Signature {
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

  /// Returns the signature data as a slice of bytes.
  pub fn data(&self) -> &[u8] {
    &self.data
  }
}
