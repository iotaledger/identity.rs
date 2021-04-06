// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;

#[derive(Clone, Debug)]
pub struct Signature {
  pub(crate) pkey: PublicKey,
  pub(crate) data: Vec<u8>,
}

impl Signature {
  pub const fn new(pkey: PublicKey, data: Vec<u8>) -> Self {
    Self { pkey, data }
  }

  pub fn pkey(&self) -> &PublicKey {
    &self.pkey
  }

  pub fn data(&self) -> &[u8] {
    &self.data
  }
}
