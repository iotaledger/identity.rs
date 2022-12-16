// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use identity_core::utils::Base;
use identity_data_integrity::verification_material::Multicodec;
use identity_data_integrity::verification_material::Multikey;

use super::KeyId;

/// Parameters for Key Storage generation of key material compliant with the `Multikey` format.
pub struct MultikeySchema {
  multicodec: Multicodec,
  multibase: Base,
}

impl MultikeySchema {
  pub fn new(multicodec: Multicodec, multibase: Base) -> Self {
    MultikeySchema { multicodec, multibase }
  }

  pub fn multicodec(&self) -> Multicodec {
    self.multicodec
  }

  pub fn multibase(&self) -> Base {
    self.multibase
  }

  /// Creates a [`MultikeySchema`] representing parameters for generating an `Ed25519` public key with base58-encoding
  /// using the Bitcoin base-encoding alphabet.
  pub fn ed25519_public_key() -> Self {
    Self {
      multicodec: Multicodec::ED25519_PUB,
      multibase: Base::Base58Btc,
    }
  }
}

/// The output when generating a key pair according to the [`MultikeySchema`](crate::key_generation::MultikeySchema).
///
/// See [`KeyStorage::generate_multikey`](crate::key_storage::KeyStorage::generate_multikey()).
pub struct MultikeyOutput {
  identifier: KeyId,
  public_key: Multikey,
}

impl MultikeyOutput {
  /// Constructs a new [`MultikeyOutput`].
  ///
  /// # Important
  ///
  /// It is crucial that the provided `identifier` corresponds to the same public key as `public_key`.
  pub fn new(identifier: KeyId, public_key: Multikey) -> Self {
    Self { identifier, public_key }
  }
}
