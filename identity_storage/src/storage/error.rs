// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_id_storage::KeyIdStorageError;
use crate::key_id_storage::MethodDigestConstructionError;
use crate::key_storage::KeyStorageError;

/// Errors that can occur when working with the [`JwkDocumentExt`](crate::storage::JwkDocumentExt) API.
#[derive(Debug, thiserror::Error)]
pub enum JwkStorageDocumentError {
  #[error("storage operation failed: key storage error")]
  KeyStorageError(KeyStorageError),
  #[error("storage operation failed: key id storage error")]
  KeyIdStorageError(KeyIdStorageError),
  #[error("could not add method: the fragment already exists")]
  FragmentAlreadyExists,
  #[error("method not found")]
  MethodNotFound,
  #[error("invalid method data format: expected publicKeyJwk")]
  NotPublicKeyJwk,
  #[error("invalid JWS algorithm")]
  InvalidJwsAlgorithm,
  #[error("cannot create jws: unable to produce kid header")]
  MissingKid,
  #[error("method generation failed: unable to create a valid verification method")]
  VerificationMethodConstructionError(#[source] identity_verification::Error),
  #[error("could not produce jwt: encoding error")]
  EncodingError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
  #[error("unable to produce method digest")]
  MethodDigestConstructionError(#[source] MethodDigestConstructionError),
  #[error("could not produce JWS payload from the given claims: serialization failed")]
  ClaimsSerializationError(#[source] identity_credential::Error),
  #[error("storage operation failed after altering state. Unable to undo operation(s): {message}")]
  UndoOperationFailed {
    message: String,
    source: Box<Self>,
    undo_error: Option<Box<Self>>,
  },
  #[error("{0}")]
  Custom(
    &'static str,
    #[source] Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
  ),
}

#[cfg(test)]
mod tests {
  use super::JwkStorageDocumentError;
  fn is_send_sync<T: Send + Sync + 'static>(_input: T) {}

  #[test]
  fn error_is_send_sync() {
    is_send_sync(JwkStorageDocumentError::FragmentAlreadyExists);
  }
}
