// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use serde::Deserialize;
use serde::Serialize;
/// A wrapper around a multibase encoded public key.
///
/// See the corresponding entry in the [VC-DATA-INTEGRITY specification](https://w3c.github.io/vc-data-integrity/#ref-for-dfn-publickeymultibase-1).  
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PublicKeyMultibase(String);

impl PublicKeyMultibase {
  /// Construct [`PublicKeyMultibase`] from the provided string representation.
  ///
  /// # Note
  /// The user is expected to supply a string representation of a multibase encoded public key. See the [multibase specification](https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03).
  pub fn new(value: String) -> Self {
    Self(value)
  }

  /// Returns a string slice representation of the multibase encoded public key.  
  pub fn as_str(&self) -> &str {
    self.0.as_str()
  }

  pub fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }
}
