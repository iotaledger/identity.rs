// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::result::Result;

/// A common interface for cryptographic key exchange procedures, e.g. Diffie-Hellman.
pub trait KeyExchange {
  /// Private key type for the first party.
  type Private: ?Sized;
  /// Corresponding public key type for the second party.
  type Public: ?Sized;
  /// Shared secret output type of the key exchange process.
  type Output;
  /// Error type on failure.
  type Error;

  /// Performs a cryptographic key exchange process (e.g. Diffie-Hellman) using the private key
  /// of the first party with with the public key of the second party, resulting in a shared secret.
  fn key_exchange(private: &Self::Private, public: &Self::Public) -> Result<Self::Output, Self::Error>;
}
