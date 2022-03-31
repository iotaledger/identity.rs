// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Deserialize;
use serde::Serialize;

/// A digital signature.
#[derive(Clone, Deserialize, Serialize)]
pub struct Signature(pub(crate) Vec<u8>);

impl Signature {
  /// Creates a `Signature`.
  pub fn new(data: Vec<u8>) -> Self {
    Signature(data)
  }

  /// Returns the signature as a slice of bytes.
  pub fn as_bytes(&self) -> &[u8] {
    &self.0
  }
}

impl From<Signature> for Vec<u8> {
  fn from(signature: Signature) -> Self {
    signature.0
  }
}
