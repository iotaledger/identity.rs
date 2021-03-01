// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::crypto::PublicKey;

#[derive(Clone, Debug)]
pub struct Signature {
  public_key: PublicKey,
  signature: Vec<u8>,
}

impl Signature {
  pub fn new(public_key: PublicKey, signature: Vec<u8>) -> Self {
    Self { public_key, signature }
  }

  pub fn public_key(&self) -> &PublicKey {
    &self.public_key
  }

  pub fn signature(&self) -> &[u8] {
    &self.signature
  }
}
