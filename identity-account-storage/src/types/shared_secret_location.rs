// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use rand::rngs::OsRng;
use rand::Rng;

/// Helper struct used to generate a random location for the shared secret generated in the diffie-hellman key exchange
pub struct SharedSecretLocation(pub(crate) Vec<u8>);

impl SharedSecretLocation {
  /// Cretes a random shared secret location
  pub fn random() -> Self {
    let location: [u8; 32] = OsRng.sample(rand::distributions::Standard);
    SharedSecretLocation(location.to_vec())
  }
}
