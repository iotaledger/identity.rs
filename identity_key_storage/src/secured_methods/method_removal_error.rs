// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::identity_storage::IdentityStorageError;
use crate::key_storage::KeyStorageError;

#[derive(Debug, thiserror::Error)]
pub enum MethodRemovalError {
  /// A method with the provided [`DIDUrl`](::identity_did::did::DIDUrl) was not found in the document.
  #[error("method removal failed: could not find the method with the specified DIDUrl")]
  MethodNotFound,

  /// Unable to retrieve the persisted metadata associated with the method.
  #[error("method removal failed: could not retrieve method metadata from storage")]
  MetadataLookup(#[source] IdentityStorageError),

  /// Unable to remove the key associated with the specified method.
  #[error("method removal failed: could not remove key material from storage")]
  KeyRemoval(#[source] KeyStorageError),

  /// The method's associated key material was removed from the key storage, but
  /// the method's metadata could not be removed the identity storage.
  #[error("method removal partially failed: unable to remove method metadata from storage")]
  PartialRemoval(#[source] IdentityStorageError),
}
