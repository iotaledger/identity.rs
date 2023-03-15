// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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
    #[error("could not produce jwt: encoding error")]
    EncodingError(#[source] Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("storage operation failed after altering state. Unable to undo operation: {message}")]
    UndoOperationFailed {message: String, source: Box<dyn std::error::Error + Send + Sync + 'static> },
}