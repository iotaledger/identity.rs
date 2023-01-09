// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::identity_storage::IdentityStorageError;

#[derive(Debug, thiserror::Error)]
pub enum SigningMaterialConstructionError {
  /// Could not find a method in the document corresponding to the provided fragment.
  #[error("could not obtain remote key: method not found")]
  MethodNotFound,
  /// Unable to retrieve the [`KeyId`](crate::identifiers::KeyId) corresponding to the desired method.
  #[error("could not obtain remote key: metadata lookup failed")]
  KeyIdRetrievalFailure(IdentityStorageError),
}
