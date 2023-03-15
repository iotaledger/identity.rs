// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::key_id_storage::KeyIdStorageError;
use crate::key_storage::KeyStorageError;

/// Errors that can occur when working with the [`JwkStorageDocumentExt`](crate::storage::JwkStorageDocumentExt) API.
#[derive(Debug, thiserror::Error)]
pub enum JwkStorageDocumentError {
  #[error("storage operation failed: key storage error")]
  KeyStorageError(KeyStorageError),
  #[error("storage operation failed: key id storage error")]
  KeyIdStorageError(KeyIdStorageError),
  #[error("could not add method. The fragment already exists")]
  FragmentAlreadyExists,
  #[error("method not found")]
  MethodNotFound,
  #[error("method generation failed: unable to create a valid verification method")]
  VerificationMethodConstructionError(#[source] identity_verification::Error),
  #[error("could not produce jwt: encoding error")]
  EncodingError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
  #[error("storage operation failed after altering state. Unable to undo operation(s): {message}")]
  UndoOperationFailed {
    message: String,
    source: Box<Self>,
    undo_error: Box<Self>,
  },
}

#[cfg(test)]
mod tests {
  use super::JwkStorageDocumentError;
  fn is_send_sync<T: Send + Sync + 'static>(input: T) {}

  #[test]
  fn error_is_send_sync() {
    let _assert_error_is_send_sync = |input: JwkStorageDocumentError| is_send_sync(input);
  }
}
