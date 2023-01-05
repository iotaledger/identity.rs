// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::identity_storage::IdentityStorageError;
use crate::key_storage::KeyStorageError;

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum MethodCreationError {
  /// The provided fragment is used by another resource in the DID Document.
  #[error("method creation failed: fragment in use")]
  FragmentInUse,

  /// The provided fragment representation does not comply with the [specified syntax](https://www.w3.org/TR/did-core/#fragment).
  #[error("method creation failed: invalid fragment syntax")]
  InvalidFragmentSyntax,

  /// The provided [`Storage`] was unable to generate a new key pair.
  #[error("method creation failed: storage failed to generate a new key pair")]
  KeyGeneration(#[source] KeyStorageError),

  #[error("method creation failed: unable to persist metadata")]
  /// Could not persist cryptographic metadata.
  MetadataPersistence(#[source] IdentityStorageError),
}
