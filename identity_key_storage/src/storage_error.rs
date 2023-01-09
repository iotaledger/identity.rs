// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::borrow::Cow;

use crate::identifiers::KeyId;

pub type StorageResult<T> = Result<T, SimpleStorageError>;

/// Errors that can occur during execution of a storage implementation.
#[derive(Debug, thiserror::Error)]
#[error("{kind}")]
pub struct SimpleStorageError {
  kind: StorageErrorKind,
  source: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl SimpleStorageError {
  /// TODO
  pub fn new(kind: StorageErrorKind) -> Self {
    Self { kind, source: None }
  }

  /// TODO
  pub fn new_with_source(kind: StorageErrorKind, source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
    Self {
      kind,
      source: Some(source),
    }
  }
}

/// Errors that can occur during execution of a storage implementation.
///
/// Implementations should map their errors to the most specific variant that
/// describes the error. Only if none of these match should the custom error be used.
///
/// Errors are considered unrecoverable unless the documentation states otherwise.
/// Variants may be considered recoverable (and thus retryable) if the error might be temporary,
/// e.g. when an API call times out or when a network connection failed.
#[derive(Debug, thiserror::Error)]
pub enum StorageErrorKind {
  /// The key with the given alias was not found.
  #[error("key `{0}` not found")]
  KeyNotFound(KeyId),
  /// A provided or derived key, or one loaded from storage did not meet the expected requirements.
  ///
  /// The `String` parameter describes why the key was invalid.
  #[error("invalid key `{0}`: {0}")]
  InvalidKey(KeyId, String),
  /// Error during encryption.
  #[error("encryption error: {0}")]
  EncryptionError(String),
  /// Error during decryption.
  #[error("encryption error: {0}")]
  DecryptionError(String),
  /// The storage doesn't support the requested functionality.
  /// For example, this can be returned from implemntations not supporting
  /// certain encryption or signing algorithms, or not supporting encryption at all.
  #[error("storage functionality not supported: {0}")]
  NotSupported(String),
  /// An error for I/O operations that may be retried, such as temporary connection failure or timeouts.
  ///
  /// Returning this error signals to the caller that the operation may be retried with a chance of success.
  /// It is at the caller's discretion whether to retry or not, and how often.
  #[error("I/O error: {0}")]
  RetryableIoError(Cow<'static, str>),
  /// An error variant that can (and should only) be used if no other variant sufficiently describes the error.
  ///
  /// In particular, this variant can be used to propagate arbitrary error instances.
  /// However, those (I/O) errors that can be retried should return [`StorageErrorKind::RetryableIoError`].
  ///
  /// The string field can be used to describe the circumstances under which the error occured
  /// or include a string or [`std::fmt::Debug`] representation of an error in cases where
  /// the source error does not implement [`std::error::Error`]. If a source error exists, it should be
  /// put into `StorageError::source`.
  #[error("{0}")]
  Other(Cow<'static, str>),
}

impl From<StorageErrorKind> for SimpleStorageError {
  fn from(kind: StorageErrorKind) -> Self {
    SimpleStorageError::new(kind)
  }
}
